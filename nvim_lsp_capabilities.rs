ClientCapabilities { 
    workspace: Some(WorkspaceClientCapabilities { 
        apply_edit: Some(true), 
        workspace_edit: Some(WorkspaceEditClientCapabilities { 
            document_changes: None, 
            resource_operations: Some([Rename, Create, Delete]), 
            failure_handling: None, 
            normalizes_line_endings: None,
            change_annotation_support: None 
        }),
        did_change_configuration: None,
        did_change_watched_files: Some(DidChangeWatchedFilesClientCapabilities { 
            dynamic_registration: Some(true),
            relative_pattern_support: Some(true) 
        }),
        symbol: Some(WorkspaceSymbolClientCapabilities { 
            dynamic_registration: Some(false),
            symbol_kind: Some(SymbolKindCapability { 
                value_set: Some([
                    File, Module, Namespace, Package, Class, Method, Property,
                    Field, Constructor, Enum, Interface, Function, Variable, 
                    Constant, String, Number, Boolean, Array, Object, Key, 
                    Null, EnumMember, Struct, Event, Operator, TypeParameter
                ]) 
            }), 
            tag_support: None, 
            resolve_support: None 
        }), 
        execute_command: None, 
        workspace_folders: Some(true), 
        configuration: Some(true), 
        semantic_tokens: Some(SemanticTokensWorkspaceClientCapabilities { 
            refresh_support: Some(true) 
        }), 
        code_lens: None, 
        file_operations: None, 
        inline_value: None, 
        inlay_hint: None 
    }), 
    text_document: Some(TextDocumentClientCapabilities { 
        synchronization: Some(TextDocumentSyncClientCapabilities { 
            dynamic_registration: Some(false),
            will_save: Some(true),
            will_save_wait_until: Some(true),
            did_save: Some(true) 
        }), 
        completion: Some(CompletionClientCapabilities { 
            dynamic_registration: Some(false), 
            completion_item: Some(CompletionItemCapability { 
                snippet_support: Some(true), 
                commit_characters_support: Some(true),
                documentation_format: Some([Markdown, PlainText]),
                deprecated_support: Some(true),
                preselect_support: Some(true),
                tag_support: Some(TagSupport { value_set: [Deprecated] }), 
                insert_replace_support: Some(true), 
                resolve_support: Some(CompletionItemCapabilityResolveSupport { 
                    properties: [
                        "documentation", "detail", "additionalTextEdits", 
                        "sortText", "filterText", "insertText", "textEdit", 
                        "insertTextFormat", "insertTextMode"
                    ] 
                }), 
                insert_text_mode_support: Some(InsertTextModeSupport { 
                    value_set: [AsIs, AdjustIndentation] 
                }), 
                label_details_support: Some(true) 
            }), 
            completion_item_kind: Some(CompletionItemKindCapability { 
                value_set: Some([
                    Text, Method, Function, Constructor, Field, Variable, 
                    Class, Interface, Module, Property, Unit, Value, Enum, 
                    Keyword, Snippet, Color, File, Reference, Folder, 
                    EnumMember, Constant, Struct, Event, Operator, TypeParameter
                ]) 
            }), 
            context_support: Some(true), 
            insert_text_mode: Some(AsIs),
            completion_list: Some(CompletionListCapability { 
                item_defaults: Some([
                    "commitCharacters", "editRange", "insertTextFormat", 
                    "insertTextMode", "data"
                ]) 
            }) 
        }), 
        hover: Some(HoverClientCapabilities {
            dynamic_registration: Some(false), 
            content_format: Some([Markdown, PlainText]) 
        }), 
        signature_help: Some(SignatureHelpClientCapabilities { 
            dynamic_registration: Some(false),
            signature_information: Some(SignatureInformationSettings {
                documentation_format: Some([Markdown, PlainText]), 
                parameter_information: Some(ParameterInformationSettings {
                    label_offset_support: Some(true) 
                }), 
                active_parameter_support: Some(true) 
            }), 
            context_support: None 
        }), 
        references: Some(DynamicRegistrationClientCapabilities { 
            dynamic_registration: Some(false) 
        }), 
        document_highlight: Some(DynamicRegistrationClientCapabilities { 
            dynamic_registration: Some(false) 
        }), 
        document_symbol: Some(DocumentSymbolClientCapabilities { 
            dynamic_registration: Some(false), 
            symbol_kind: Some(SymbolKindCapability { 
                value_set: Some([
                    File, Module, Namespace, Package, Class, Method, Property,
                    Field, Constructor, Enum, Interface, Function, Variable,
                    Constant, String, Number, Boolean, Array, Object, Key,
                    Null, EnumMember, Struct, Event, Operator, TypeParameter
                ]) 
            }), 
            hierarchical_document_symbol_support: Some(true), 
            tag_support: None 
        }), 
        formatting: None, 
        range_formatting: None, 
        on_type_formatting: None,
        declaration: Some(GotoCapability {
            dynamic_registration: None, 
            link_support: Some(true) 
        }), 
        definition: Some(GotoCapability { 
            dynamic_registration: None, 
            link_support: Some(true) 
        }), 
        type_definition: Some(GotoCapability { 
            dynamic_registration: None, 
            link_support: Some(true)
        }), 
        implementation: Some(GotoCapability { 
            dynamic_registration: None,
            link_support: Some(true) 
        }), 
        code_action: Some(CodeActionClientCapabilities { 
            dynamic_registration: Some(false),
            code_action_literal_support: Some(CodeActionLiteralSupport { 
                code_action_kind: CodeActionKindLiteralSupport { 
                    value_set: [
                        "", "quickfix", "refactor", "refactor.extract", 
                        "refactor.inline", "refactor.rewrite", "source", 
                        "source.organizeImports"
                    ] 
                } 
            }), 
            is_preferred_support: Some(true), 
            disabled_support: None,
            data_support: Some(true),
            resolve_support: Some(CodeActionCapabilityResolveSupport {
                properties: ["edit"] 
            }), 
            honors_change_annotations: None 
        }), 
        code_lens: None,
        document_link: None,
        color_provider: None,
        rename: Some(RenameClientCapabilities {
            dynamic_registration: Some(false),
            prepare_support: Some(true),
            prepare_support_default_behavior: None,
            honors_change_annotations: None 
        }),
        publish_diagnostics: Some(PublishDiagnosticsClientCapabilities {
            related_information: Some(true),
            tag_support: Some(TagSupport {
                value_set: [Unnecessary, Deprecated] 
            }),
            version_support: None,
            code_description_support: None,
            data_support: None 
        }),
        folding_range: None,
        selection_range: None,
        linked_editing_range: None,
        call_hierarchy: Some(DynamicRegistrationClientCapabilities {
            dynamic_registration: Some(false) 
        }),
        semantic_tokens: Some(SemanticTokensClientCapabilities {
            dynamic_registration: Some(false),
            requests: SemanticTokensClientCapabilitiesRequests {
                range: Some(false),
                full: Some(Delta { delta: Some(true) }) 
            },
            token_types: [
                SemanticTokenType("namespace"), SemanticTokenType("type"), 
                SemanticTokenType("class"), SemanticTokenType("enum"), 
                SemanticTokenType("interface"), SemanticTokenType("struct"), 
                SemanticTokenType("typeParameter"), 
                SemanticTokenType("parameter"), SemanticTokenType("variable"), 
                SemanticTokenType("property"), SemanticTokenType("enumMember"), 
                SemanticTokenType("event"), SemanticTokenType("function"), 
                SemanticTokenType("method"), SemanticTokenType("macro"), 
                SemanticTokenType("keyword"), SemanticTokenType("modifier"), 
                SemanticTokenType("comment"), SemanticTokenType("string"), 
                SemanticTokenType("number"), SemanticTokenType("regexp"), 
                SemanticTokenType("operator"), SemanticTokenType("decorator")
            ], 
            token_modifiers: [
                SemanticTokenModifier("declaration"),
                SemanticTokenModifier("definition"),
                SemanticTokenModifier("readonly"),
                SemanticTokenModifier("static"),
                SemanticTokenModifier("deprecated"),
                SemanticTokenModifier("abstract"),
                SemanticTokenModifier("async"),
                SemanticTokenModifier("modification"),
                SemanticTokenModifier("documentation"),
                SemanticTokenModifier("defaultLibrary")
            ], 
            formats: [TokenFormat("relative")], 
            overlapping_token_support: Some(true), 
            multiline_token_support: Some(false), 
            server_cancel_support: Some(false), 
            augments_syntax_tokens: Some(true) 
        }),
        moniker: None,
        type_hierarchy: None,
        inline_value: None,
        inlay_hint: None 
    }),
    window: Some(WindowClientCapabilities {
        work_done_progress: Some(true),
        show_message: Some(ShowMessageRequestClientCapabilities { 
            message_action_item: Some(MessageActionItemCapabilities { 
                additional_properties_support: Some(false) 
            }) 
        }), show_document: Some(ShowDocumentClientCapabilities {
            support: true 
        }) 
    }), 
    general: None,
    experimental: None 
}
