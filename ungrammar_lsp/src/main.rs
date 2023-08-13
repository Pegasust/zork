use std::{error::Error, io::Read, path::PathBuf, str::FromStr};

use lsp_server::{Connection, Message, Notification as NotificationData, Response};
use lsp_types::{
    InitializeParams, ClientCapabilities, ServerCapabilities, 
    TextDocumentSyncCapability, TextDocumentSyncKind, 
    notification::{
        DidChangeTextDocument, Notification, LogMessage, PublishDiagnostics, 
        DidSaveTextDocument
    }, 
    DidChangeTextDocumentParams, VersionedTextDocumentIdentifier, 
    LogMessageParams, MessageType, PublishDiagnosticsParams, Diagnostic, 
    DiagnosticSeverity, Range,  Position, request::{
        Shutdown, Request
    }, 
    DidSaveTextDocumentParams, TextDocumentIdentifier
};

use serde_with::with_prefix;
use tracing_subscriber::{fmt, EnvFilter, Layer};
use tracing::{instrument, warn, debug, info, error, metadata::LevelFilter};

use ungrammar_fork::{Grammar, lexer::Location};
use serde::Deserialize;
use sec::Secret;

const BINARY_NAME: &str = "ungrammar_lsp";

#[instrument(skip(lsp))]
fn handle_notification(
    notif: NotificationData,
    lsp: &Connection
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let method: &str = &notif.method;
    debug! {?notif, "got notification"};
    match method {
        DidSaveTextDocument::METHOD => {
            let DidSaveTextDocumentParams {
                text_document: TextDocumentIdentifier {uri},
                text
            } = notif.extract(
                DidSaveTextDocument::METHOD
            )?;
            let ungrammar_str = if let Some(text) = text {
                text
            } else {
                debug! {
                    document_uri=%uri,
                    "Text not streamed from connection, we're instead reading \
                    the file instead",
                };
                if uri.scheme() != "file" {
                    let scheme = uri.scheme();
                    warn!("Got {uri} not supported uri scheme {scheme}");
                    let log_msg = LogMessageParams{
                        typ: MessageType::WARNING,
                        message: format!("Only support file:// schema, got {uri}"),
                    };
                    lsp.sender.send(Message::Notification(NotificationData {
                        method: LogMessage::METHOD.into(),
                        params: serde_json::to_value(log_msg)?,
                    }))?;
                }

                let mut file = std::fs::File::open(uri.path())?;
                let mut ungrammar_str = String::new();
                file.read_to_string(&mut ungrammar_str)?;
                ungrammar_str
            };
            let parse_err = ungrammar_str.parse::<Grammar>();
            match parse_err {
                Ok(grammar) => {
                    let log_str = format!("Successfully parsed grammar {grammar:?}");
                    debug!(?grammar, "Successfully parsed grammar");
                    let log_msg = LogMessageParams {
                        typ: MessageType::LOG,
                        message: log_str,
                    };
                    lsp.sender.send(Message::Notification(NotificationData {
                        method: LogMessage::METHOD.into(),
                        params: serde_json::to_value(log_msg)?,
                    }))?;
                },
                Err(err) => {
                    let err_dbg = err.clone();
                    let err_diag = err.into_lsp_diagnostic(
                        Some(DiagnosticSeverity::ERROR),
                        Some(BINARY_NAME.into()),
                    );
                    debug!(parse_err = ?err_dbg, ?err_diag, "Got error");
                    let diag = PublishDiagnosticsParams {
                        uri,
                        diagnostics: vec![err_diag],
                        version: None,
                    };
                    lsp.sender.send(Message::Notification(NotificationData {
                        method: PublishDiagnostics::METHOD.into(),
                        params: serde_json::to_value(diag)?,
                    }))?;
                },
            }
            
        }
        DidChangeTextDocument::METHOD => {
            info! {"Ignore didChange for simplicity"};
            let DidChangeTextDocumentParams {
                text_document,
                content_changes: _,
            }= notif.extract(
                DidChangeTextDocument::METHOD
            ).unwrap();

            let VersionedTextDocumentIdentifier{
                version, uri
            } = text_document;

            warn!(
                "Not intelligent enough to parse document changes, retracting \
                all diagnostics"
            );
            lsp.sender.send(Message::Notification(NotificationData {
                method: PublishDiagnostics::METHOD.into(),
                params: serde_json::to_value(PublishDiagnosticsParams {
                    uri,
                    diagnostics: vec![],
                    version: Some(version),
                })?,
            }))?;

        },
        ignored => {
            warn!(
                "Unhandled method {ignored:?} {notif:?}. Might be bad capabilities."
            );
        }
    }
    Ok(())
}

#[instrument]
fn lsp_main() ->Result<(), Box<dyn Error + Sync + Send>>  {
    debug!{"pre: init stdio connection"};
    let (connection, io_threads) = Connection::stdio();
    debug!{"post: init stdio connection"};

    // Run the server
    let (id, params) = connection.initialize_start()?;

    let init_params: InitializeParams = serde_json::from_value(params).unwrap();
    let client_capabilities: ClientCapabilities = init_params.capabilities;
    info! {"Client cap: {client_capabilities:?}"};
    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(
            TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)
        ),
        ..Default::default()
    };



    info! {"Server cap: {server_capabilities:?}"};
    // TODO: This is fine because negotiated capabilities will always be
    // subset of client-submitted capabilities at early dev
    let negotiated_capabilities = server_capabilities;
    let initialize_data = serde_json::json!({
        "capabilities": negotiated_capabilities,
        "serverInfo": {
            "name": BINARY_NAME,
            "version": "dev"
        }
    });

    connection.initialize_finish(id, initialize_data)?;
    // Main loop where the LSP server listens for client messages.
    for message in &connection.receiver {
        debug!{"received {message:?}"}
        match message {
            Message::Request(req) if req.method == Shutdown::METHOD => {
                info!{"shutdown initiated"};
                connection.sender.send(Message::Response(Response {
                    id: req.id,
                    result: None,
                    error: None,
                }))?;
            }
            Message::Request(req) => {
                warn!(
                    "client sends req {req:?}, not sure how to handle. \
                    There might be a server-client capabilities misunderstanding."
                );
            }
            Message::Notification(notification) => {
                let notif = notification.clone();
                let notif_dbg = format!("{notif:?}");
                if let Err(err) = handle_notification(notification, &connection) {
                    error!("Error handling notif {notif_dbg}: {err}")
                }
            }
            ignore => {
                info!{"ignoring {ignore:?}"};
            }
        }
    }
    io_threads.join().map_err(Into::into)
}

const fn sandbox_default() -> bool { false }

fn default_rust_log() -> String { "debug".into() }

fn config_path() -> PathBuf { PathBuf::from("ungrammar_lsp.toml") }

with_prefix!(prefix_extern_otlp "extern_otlp_");
#[derive(Deserialize, Debug, Default)]
pub(crate) struct EnvSchema {
    #[serde(rename="sandbox", default="sandbox_default")]
    pub(crate) use_sandbox: bool,

    #[serde(rename="config", alias="ungrammar_lsp_conf")]
    pub(crate) config_loc: Option<PathBuf>,

    #[serde(rename="rust_log", alias="log_level", default="default_rust_log")]
    pub(crate) log_level: String,

    #[serde(rename="log_file")]
    pub(crate) log_loc: Option<PathBuf>,

    #[serde(flatten, with="prefix_extern_otlp")]
    pub(crate) extern_otlp: Option<ExternOtlpWrite>,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type", rename_all="snake_case")]
pub(crate) enum Auth {
    Basic {
        username: String,
        secret: Secret<String>,
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ExternOtlpWrite {
    pub(crate) endpoint: url::Url,
    #[serde(flatten)]
    pub(crate) auth: Option<Auth>,
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(BINARY_NAME);
    let env: EnvSchema = envy::from_env()?;
    
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::from_str(&env.log_level)?.into())
        .from_env_lossy();

    let ungrammar_lsp_conf = {
        if let Some(conf_loc) = env.config_loc {
            conf_loc
        } else if env.use_sandbox {
            xdg_dirs?.get_config_file(config_path())
        } else {
            xdg_dirs?.place_config_file(config_path())?
        }
    };

    let stderr_layer = fmt::layer()
        .with_writer(std::io::stderr);

    let aggregate = stderr_layer
        .with_filter(filter);

    // let aggregate = if let Some(log_file) = env.log_loc {
    //     aggregate.with_filter(
    //         fmt::layer().with_writer(File::)
    //     )
    // } else {
    //     aggregate
    // };

    {
        use tracing_subscriber::util::SubscriberInitExt;
        aggregate
            .with_subscriber(
                tracing_subscriber::Registry::default()
            )
            .try_init()?;
    };

    lsp_main()
}
pub(crate) trait DiagnosticExt {
    fn range(&self) -> Range;
    fn msg(&self) -> String;
    fn into_lsp_diagnostic(
        self, 
        severity: Option<DiagnosticSeverity>,
        source: Option<String>,
    ) -> Diagnostic 
    where Self: Sized {
        Diagnostic { 
            range: self.range(),
            message: self.msg(),
            severity,
            source,

            code: Default::default(),
            code_description: Default::default(),
            related_information: Default::default(),
            tags: Default::default(),
            data: Default::default(),
        }
    }
}


trait TryIntoPositionExt {
    fn try_into_position(&self) -> color_eyre::Result<Position>;
}

impl TryIntoPositionExt for Location {
    fn try_into_position(&self) -> color_eyre::Result<Position> {
        Ok(Position {
            line: self.line.try_into()?,
            character: self.column.try_into()?
        })
    }
}

impl DiagnosticExt for ungrammar_fork::Error {
    #[instrument]
    fn range(&self) -> Range {
        match self {
            ungrammar_fork::Error::Simple{ location: Some(loc), ..} => {
                let begin = loc.try_into_position().unwrap();

                let end = Position {
                    line: begin.line,
                    character: begin.character + 1,
                };
                info!{?loc};
                Range {
                    start: begin, 
                    end,
                }
            },
            ungrammar_fork::Error::Simple { location: None, .. } => {
                info!{"Encountered None for location of ungrammar_fork::Error::Simple"};
                Range::default()
            },
            ungrammar_fork::Error::Range { range: ungrammar_fork::lexer::Range {
                begin,
                ex_end,
            }, .. } => {
                Range {
                    start: begin.try_into_position().unwrap(),
                    end: ex_end.try_into_position().unwrap(),
                }
            }
        }
    }

    fn msg(&self) -> String {
        match self {
            ungrammar_fork::Error::Simple { message, location } => message.into(),
            ungrammar_fork::Error::Range { message, range } => message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use serde_with::with_prefix;
    use std::fmt::Debug;
    use std::collections::HashMap;

    #[test]
    fn prefix_serde() {
        with_prefix!(pub(crate) prefix_foo "foo_");

        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        pub(crate) struct Foo {
            hello: String,
            world: u16,
        }
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        pub(crate) struct Bar {
            #[serde(flatten, with="prefix_foo")]
            foo: Foo
        }

        
        assert!(matches! {
            serde_json::from_str::<Bar>(r#"{
                "hello": "L",
                "world": 42
            }"#),
            // stderr> missing field "hello" (would have been better if it 
            // hinted at "foo_hello"; ok for now I guess)
            Err(_),
        });

        assert_eq!{
            serde_json::from_str::<Bar>(r#"{
                "foo_hello": "L",
                "foo_world": 42
            }"#).expect("should be valid Bar obj with prefix foo_"),
            Bar {
                foo: Foo {
                    hello: "L".into(),
                    world: 42
                }
            }
        };

        let json_str = serde_json::to_string(&Bar {
            foo: Foo {
                hello: "ungrammar_lsp".into(),
                world: 0,
            }
        }).expect("Bar should have Serialize implemented");
        insta::assert_debug_snapshot!(&json_str);
        assert_eq!(
            serde_json::from_str::<HashMap<String, Value>>(&json_str)
                .expect("deserializable into map[str, value]"),
            {
                let mut rv = HashMap::<String, Value>::new();
                rv.insert("foo_hello".into(), "ungrammar_lsp".into());
                rv.insert("foo_world".into(), 0.into());
                rv
            }
        );
    }

    #[test]
    fn extern_otlp_config() {
        use super::EnvSchema;

        insta::assert_debug_snapshot!(
            serde_json::from_str::<EnvSchema>(r#"{
                "extern_otlp_endpoint": "localhost:8440/trace",
                "sandbox": true
            }"#)
            .expect("Should be valid EnvSchema JSON")
        );
        insta::assert_debug_snapshot!(
            serde_json::from_str::<EnvSchema>(r#"{
                "extern_otlp_endpoint": "localhost:8440/trace",
                "sandbox": true,
                "extern_otlp_type": "basic",
                "extern_otlp_username": "agent",
                "extern_otlp_secret": "P"
            }"#)
            .expect("Should be valid EnvSchema JSON")
        );
    }
}

#[cfg(test)]
mod join_err_test {
    use color_eyre::{eyre::eyre, eyre::Report, Section};
    use thiserror::Error;

    #[test]
    fn main() {
        let errors = get_errors();
        assert!(join_errors(errors).is_err());
    }

    fn join_errors(results: Vec<Result<(), SourceError>>) -> Result<(), Report> {
        if results.iter().all(|r| r.is_ok()) {
            return Ok(());
        }

        let base_err = eyre!("encountered multiple errors");
        results
            .into_iter()
            .filter(Result::is_err)
            .map(Result::unwrap_err)
            .fold(Err(base_err), |report, e| {
                report.error(e)
            })
    }

    /// Helper function to generate errors
    fn get_errors() -> Vec<Result<(), SourceError>> {
        vec![
            Err(SourceError {
                source: StrError("The task you ran encountered an error"),
                msg: "The task could not be completed",
            }),
            Err(SourceError {
                source: StrError("The machine you're connecting to is actively on fire"),
                msg: "The machine is unreachable",
            }),
            Err(SourceError {
                source: StrError("The file you're parsing is literally written in c++ instead of rust, what the hell"),
                msg: "The file could not be parsed",
            }),
        ]
    }

    /// Arbitrary error type for demonstration purposes
    #[derive(Debug, Error)]
    #[error("{0}")]
    struct StrError(&'static str);

    /// Arbitrary error type for demonstration purposes with a source error
    #[derive(Debug, Error)]
    #[error("{msg}")]
    struct SourceError {
        msg: &'static str,
        source: StrError,
    }

}


#[cfg(test)]
mod tracing_error_test {
    use tracing::instrument;
    use tracing_error::{SpanTrace, ExtractSpanTrace, TracedError};
    #[derive(Debug)]
    pub struct MyError {
        message: String,
        context: SpanTrace,
        //...
    }
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // ... format structured fields
            self.context.fmt(f)?;
            // ... format other error context info, cause chain,...
            Ok(())
        }
    }
    impl std::error::Error for MyError {}
    impl MyError {
        pub fn new(msg: String) -> Self {
            // use 
            Self {message: msg, context: SpanTrace::capture()}
        }
    }
    // cherry on top: bypass hacky ExtractSpantrace via `dyn Error + 'static`
    impl ExtractSpanTrace for MyError {
        fn span_trace(&self) -> Option<&SpanTrace> {
            Some(&self.context)
        }
    }

    #[instrument]
    fn faulty_function() -> Result<(), MyError> {
        Err(MyError::new("Something went wrong".into()))
    }

    #[test]
    fn should_output_span_trace() {

    }
}
