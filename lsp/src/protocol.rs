//! Provides functionalities related to LSP. Though `lsp-server` and `lsp-types`
//! provides invaluable resources for this, it does not really ties onto our
//! business logic or structure of the code.
//!
//! This `protocol` module serves as an abstract translation layer onto our  
//! business logic needs for Zork to suit our specific needs. This is to manage
//! communication between applicaion and LSP client
//!
//! This handles reading and writing of LSP messages, manage life-cycle of the
//! connection, handle protocol initialization,... This is so that the main()
//! function will just be
//! ```
//! let dotenv = load_dotenv();
//! let proto = protocol::LspProtocol::from_env(dotenv);
//!
//! // ...
//! while let Some(req) = proto.recv_message() {
//!     match req {
//!         protocol::Message::DidOpenTextDocument(params) => {
//!             // ...
//!         },
//!         protocol::Message::DidChangeTextDocument(params) => {
//!             // ...
//!         },
//!     }
//! }
//! ```
//! 
