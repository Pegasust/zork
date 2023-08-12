use std::{error::Error, io::Read};

use env_logger::Env;
use lsp_server::{Connection, Message, Notification as NotificationData, Response};
use lsp_types::{
    InitializeParams, ClientCapabilities, ServerCapabilities, 
    TextDocumentSyncCapability, TextDocumentSyncKind, 
    notification::{DidChangeTextDocument, Notification, LogMessage, PublishDiagnostics}, 
    DidChangeTextDocumentParams, VersionedTextDocumentIdentifier, 
    LogMessageParams, MessageType, PublishDiagnosticsParams, Diagnostic, 
    DiagnosticSeverity, Range,  Position, request::{Shutdown, Request}
};

use tracing_subscriber::fmt;
use tracing::{instrument, warn, debug, info, error};

use ungrammar_fork::Grammar;

#[instrument(skip(lsp))]
fn handle_notification(
    notif: NotificationData,
    lsp: &Connection
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let method: &str = &notif.method;
    match method {
        DidChangeTextDocument::METHOD => {
            let params: DidChangeTextDocumentParams = notif.extract(
                DidChangeTextDocument::METHOD
            ).unwrap();

            let VersionedTextDocumentIdentifier{
                version: _, uri
            } = params.text_document;

            warn!("Not yet handling document changes");

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

            let parse_err = ungrammar_str.parse::<Grammar>();
            let mut diagnostics: Vec<Diagnostic> = Vec::with_capacity(8);
            match parse_err {
                Ok(grammar) => {
                    let log_str = format!("Successfully parsed grammar {grammar:?}");
                    debug!("{log_str}");
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
                    diagnostics.push(err.into_lsp_diagnostic(
                        Some(DiagnosticSeverity::ERROR),
                        Some("ungrammar_lsp".into()),
                    ));
                },
            }
            let diag = PublishDiagnosticsParams {
                uri,
                diagnostics,
                version: None,
            };
            lsp.sender.send(Message::Notification(NotificationData {
                method: PublishDiagnostics::METHOD.into(),
                params: serde_json::to_value(diag)?,
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
fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // NOTE: probably should take control over what the max_level is, just in
    // case tracing_subscriber::fmt doesn't know which level to look for
    let log_env = Env::default().default_filter_or("debug");
    env_logger::Builder::from_env(
        log_env
    ).try_init()?;

    fmt::try_init()?;

    let (connection, io_threads) = Connection::stdio();

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
            "name": "ungrammar_lsp",
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
