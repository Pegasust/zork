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
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{instrument, warn, debug, info, error, metadata::LevelFilter};

use ungrammar_fork::Grammar;
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
                    let diag = PublishDiagnosticsParams {
                        uri,
                        diagnostics: vec![err.into_lsp_diagnostic(
                            Some(DiagnosticSeverity::ERROR),
                            Some(BINARY_NAME.into()),
                        )],
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

    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init()?;

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


impl DiagnosticExt for ungrammar_fork::Error {
    fn range(&self) -> Range {
        match self.location {
            Some(loc) => {
                let pos = Position {
                        line: (loc.line - 1).try_into().unwrap(),
                        character: (loc.column - 1).try_into().unwrap(),
                    };
                Range {
                    start: pos, 
                    end: pos,
                }
            },
            None => Range::default()
        }
    }

    fn msg(&self) -> String {
        self.message.clone()
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
