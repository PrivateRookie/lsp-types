mod part1;
mod part2;
mod part3;
mod patch;

use std::fmt::Debug;

pub use part1::*;
pub use part2::*;
pub use part3::*;
pub use patch::*;

/// current lsp version
pub const VERSION: &str = "3.16";

macro_rules! serde_empty {
    ($type:ty) => {
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self::default())
            }
        }

        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_none()
            }
        }
    };
}

serde_empty!(ExitParams);
serde_empty!(InitializedParams);
serde_empty!(SemanticTokensRefreshParams);
serde_empty!(ShutdownParams);
serde_empty!(WorkspaceFolderParams);
serde_empty!(Empty);

pub trait FromReq: Sized + serde::Serialize {
    const METHOD: &'static str;
    type Ret;

    /// perform message cast from raw request message
    /// if method do not match, return `OneOf::Other(request)`
    fn from_req(req: RequestMessage) -> OneOf<(ReqId, Self), RequestMessage>;

    fn into_req(self, id: ReqId) -> RequestMessage {
        RequestMessage {
            id,
            method: Self::METHOD.to_string(),
            jsonrpc: "2.0".to_string(),
            params: Some(serde_json::to_value(self).unwrap()),
        }
    }

    /// helper function to test method match or not
    fn can_cast(req: &RequestMessage) -> bool {
        Self::METHOD == req.method
    }
}

#[macro_export]
macro_rules! impl_req {
    ($type:ty, $method:literal, $ret:path) => {
        impl FromReq for $type {
            const METHOD: &'static str = $method;
            type Ret = $ret;

            fn from_req(req: RequestMessage) -> OneOf<(ReqId, Self), RequestMessage> {
                if <Self as FromReq>::can_cast(&req) {
                    let RequestMessage { id, params, .. } = req;
                    let params: Self =
                        serde_json::from_value(params.unwrap_or_else(|| serde_json::Value::Null))
                            .unwrap();
                    OneOf::This((id, params))
                } else {
                    OneOf::Other(req)
                }
            }
        }

        impl $type {
            /// helper function for user do not need to remember
            /// result type of a request
            pub fn ret(result: $ret) -> $ret {
                result
            }
        }
    };
    ($type:ty, $method:literal) => {
        impl_req!($type, $method, serde_json::Value);
    };
}

impl_req!(InitializeParams, "initialize", InitializeResult);
impl_req!(
    ShowMessageRequestParams,
    "window/showMessageRequest",
    Option<MessageActionItem>
);
impl_req!(
    ShowDocumentParams,
    "window/showDocument",
    ShowDocumentResult
);
impl_req!(RegistrationParams, "client/registerCapability", Empty);
impl_req!(UnregistrationParams, "client/unregisterCapability", Empty);
impl_req!(
    ConfigurationParams,
    "workspace/configuration",
    Vec<serde_json::Value>
);
impl_req!(
    WorkspaceSymbolParams,
    "workspace/symbol",
    Vec<SymbolInformation>
);
impl_req!(ExecuteCommandParams, "workspace/executeCommand");
impl_req!(
    ApplyWorkspaceEditParams,
    "workspace/applyEdit",
    ApplyWorkspaceEditResponse
);
impl_req!(
    CreateFilesParams,
    "workspace/willCreateFiles",
    Option<WorkspaceEdit>
);
impl_req!(
    RenameFilesParams,
    "workspace/willRenameFiles",
    Option<WorkspaceEdit>
);
impl_req!(
    DeleteFilesParams,
    "workspace/willDeleteFiles",
    Option<WorkspaceEdit>
);
impl_req!(
    WillSaveTextDocumentParams,
    "textDocument/willSaveWaitUntil",
    Vec<TextEdit>
);
impl_req!(
    CompletionParams,
    "textDocument/completion",
    OneOf<Vec<CompletionItem>, CompletionList>
);
impl_req!(CompletionItem, "completionItem/resolve", CompletionItem);
impl_req!(HoverParams, "textDocument/hover", Option<Hover>);
impl_req!(
    SignatureHelpParams,
    "textDocument/signatureHelp",
    Option<SignatureHelp>
);
impl_req!(
    DeclarationParams,
    "textDocument/declaration",
    OneOf<Vec<Location>, Vec<LocationLink>>
);
impl_req!(
    DefinitionParams,
    "textDocument/definition",
    OneOf<Vec<Location>, Vec<LocationLink>>
);
impl_req!(
    TypeDefinitionParams,
    "textDocument/typeDefinition",
    OneOf<Vec<Location>, Vec<LocationLink>>
);
impl_req!(
    ImplementationParams,
    "textDocument/implementation",
    OneOf<Vec<Location>, Vec<LocationLink>>
);
impl_req!(ReferenceParams, "textDocument/references", Vec<Location>);
impl_req!(
    DocumentHighlightParams,
    "textDocument/documentHighlight",
    Vec<DocumentHighlight>
);
impl_req!(
    DocumentSymbolParams,
    "textDocument/documentSymbol",
    OneOf<Vec<DocumentSymbol>, Vec<SymbolInformation>>
);
impl_req!(
    CodeActionParams,
    "textDocument/codeAction",
    Vec<OneOf<Command, CodeAction>>
);
impl_req!(CodeAction, "codeAction/resolve", CodeAction);
impl_req!(CodeLensParams, "textDocument/codeLens", Vec<CodeLens>);
impl_req!(CodeLens, "codeLens/resolve", CodeLens);
impl_req!(
    DocumentLinkParams,
    "textDocument/documentLink",
    Vec<DocumentLink>
);
impl_req!(DocumentLink, "documentLink/resolve", DocumentLink);
impl_req!(
    DocumentFormattingParams,
    "textDocument/formatting",
    Vec<TextEdit>
);
impl_req!(
    DocumentRangeFormattingParams,
    "DocumentRangeFormattingParams",
    Vec<TextEdit>
);
impl_req!(
    DocumentOnTypeFormattingParams,
    "textDocument/onTypeFormatting",
    Vec<TextEdit>
);
impl_req!(RenameParams, "textDocument/rename", Option<WorkspaceEdit>);
impl_req!(
    PrepareRenameParams,
    "textDocument/prepareRename",
    Option<OneOf3<Range, PrepareRenameResult1, PrepareRenameResult2>>
);
impl_req!(
    FoldingRangeParams,
    "textDocument/foldingRange",
    Vec<FoldingRange>
);
impl_req!(ShutdownParams, "shutdown", Empty);
impl_req!(
    WorkDoneProgressCreateParams,
    "window/workDoneProgress/create",
    Empty
);
impl_req!(
    CallHierarchyPrepareParams,
    "textDocument/prepareCallHierarchy",
    Vec<CallHierarchyItem>
);
impl_req!(
    CallHierarchyIncomingCallsParams,
    "callHierarchy/incomingCalls",
    Vec<CallHierarchyIncomingCall>
);
impl_req!(
    CallHierarchyOutgoingCallsParams,
    "callHierarchy/outgoingCalls",
    Vec<CallHierarchyOutgoingCall>
);
impl_req!(
    SelectionRangeParams,
    "textDocument/selectionRange",
    Vec<SelectionRange>
);
impl_req!(
    WorkspaceFolderParams,
    "workspace/workspaceFolders",
    Vec<WorkspaceFolder>
);
impl_req!(
    DocumentColorParams,
    "textDocument/documentColor",
    Vec<ColorInformation>
);
impl_req!(
    ColorPresentationParams,
    "textDocument/colorPresentation",
    Vec<ColorPresentation>
);
impl_req!(
    SemanticTokensParams,
    "textDocument/semanticTokens/full",
    Option<SemanticTokens>
);
impl_req!(
    SemanticTokensDeltaParams,
    "textDocument/semanticTokens/full/delta",
    Option<OneOf<SemanticTokens, SemanticTokensDelta>>
);
impl_req!(
    SemanticTokensRangeParams,
    "textDocument/semanticTokens/range",
    Option<SemanticTokens>
);
impl_req!(
    SemanticTokensRefreshParams,
    "workspace/semanticTokens/refresh",
    Empty
);
impl_req!(
    LinkedEditingRangeParams,
    "textDocument/linkedEditingRange",
    Option<LinkedEditingRanges>
);
impl_req!(MonikerParams, "textDocument/moniker", Vec<Moniker>);

pub trait FromNotice: Sized + serde::Serialize {
    const METHOD: &'static str;

    /// perform message cast
    /// if method do not match, return `OneOf::Other(request)`
    fn from_notice(notice: NotificationMessage) -> OneOf<Self, NotificationMessage>;

    /// helper method for specify notification down cast to generic Notification
    /// message
    fn into_notice(self) -> NotificationMessage {
        NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: <Self as FromNotice>::METHOD.to_string(),
            params: Some(serde_json::to_value(self).unwrap()),
        }
    }

    /// test method match or not
    fn can_cast(notice: &NotificationMessage) -> bool {
        Self::METHOD == notice.method
    }
}

#[macro_export]
macro_rules! impl_notice {
    ($type:ty, $method:literal) => {
        impl FromNotice for $type {
            const METHOD: &'static str = $method;

            fn from_notice(notice: NotificationMessage) -> OneOf<Self, NotificationMessage> {
                if <Self as FromNotice>::can_cast(&notice) {
                    let NotificationMessage { params, .. } = notice;
                    let params =
                        serde_json::from_value(params.unwrap_or_else(|| serde_json::Value::Null))
                            .unwrap();
                    OneOf::This(params)
                } else {
                    OneOf::Other(notice)
                }
            }
        }
    };
}

impl_notice!(CancelParams, "$/cancelRequest");
impl_notice!(ProgressParams, "$/progress");
impl_notice!(InitializedParams, "initialized");
impl_notice!(ExitParams, "exit");
impl_notice!(LogTraceParams, "$/logTrace");
impl_notice!(SetTraceParams, "$/setTrace");
impl_notice!(ShowMessageParams, "window/showMessage");
impl_notice!(LogMessageParams, "window/logMessage");
impl_notice!(
    WorkDoneProgressCancelParams,
    "window/workDoneProgress/cancel"
);
impl_notice!(
    DidChangeWorkspaceFoldersParams,
    "workspace/didChangeWorkspaceFolders"
);
impl_notice!(
    DidChangeConfigurationParams,
    "workspace/didChangeConfiguration"
);
impl_notice!(
    DidChangeWatchedFilesParams,
    "workspace/didChangeWatchedFiles"
);
impl_notice!(CreateFilesParams, "workspace/didCreateFiles");
impl_notice!(DeleteFilesParams, "workspace/didDeleteFiles");
impl_notice!(DidOpenTextDocumentParams, "textDocument/didOpen");
impl_notice!(DidChangeTextDocumentParams, "textDocument/didChange");
impl_notice!(WillSaveTextDocumentParams, "textDocument/willSave");
impl_notice!(DidSaveTextDocumentParams, "textDocument/didSave");
impl_notice!(DidCloseTextDocumentParams, "textDocument/didClose");
impl_notice!(PublishDiagnosticsParams, "textDocument/publishDiagnostics");
