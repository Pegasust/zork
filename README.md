# Zork

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

Offline-first service for Zettelkasten via Markdown

For now, we're working on LSP-as-frontend so that it works with any text editor of choice


## Cool features

- Is there anyway (and any well-established endpoint) to do streaming for gradually 
added docs/details per suggestion? (show suggestion, then gradually stream 
on-cmp-suggestion details like docs and `return_type`)

- Range-based redraw and AST-level incremental computation

- LSP as external service and our local LSP is offline-first, but can synchronize
for less computation across multiple LSP client or client-server instances in
a single workspace.

- Not only local file storage but also maybe object storage or some cloud storage

## `ungrammar_lsp`

It's in alpha right now, it's a simple fork from [`gh:rust-analyzer/ungrammar`
](https://github.com/rust-analyzer/ungrammar) with a few tweaks to make the
grammar parser compatible with LSP.

It's a simple project for sandboxing and the development of [`zork_keg.ungram`
](./markup_ungrams/zork_keg.ungram)
