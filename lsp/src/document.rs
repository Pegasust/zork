//! Responsible for keeping track of all loaded document, receives notifications
//! about changes to these documents from LSP clients and update its state
//! internally. 
//!
//! This exists to provide some functionalities that satisfy
//! some {pre,post}-condition so that the codebase is simpler and focus on 
//! specific tasks.
//!
//! This module could be designed to interface with our Markdown parser module,
//! update its internal state whenever a document is changed and providing 
//! an API to the rest of our application to query this state.
//!
//! The endgoal might look something like
//!
//! ```
//! let doc_manager = document::DocumentManager::local_file("/tmp/zork-docman");
//!
//! // ...
//! doc_manager.open_document(params.text_document);
//! // ...
//! doc_manager.refetch_document(params.text_document);
//! ```
//! 

