//! This module's responsibility is to parse the markdown documents into a 
//! format that's easier to analyze and manipulate. Depending on how 
//! complex your Markdown flavor is, you could consider using an existing 
//! library or writing your own.
//!
//! Regardless, the goal is to convert the text into an AST or a similar 
//! data structure. This will make it easier to implement the various features 
//! your LSP server will provide, such as providing hover information or autocompletion.

