mod part1;
mod part2;
mod part3;
mod patch;

use std::fmt::Debug;

pub use part1::*;
pub use part2::*;
pub use part3::*;
pub use patch::*;

impl ResponseMessage {
    pub fn ok_resp<T: serde::Serialize, R: Into<Option<T>>>(id: ReqId, result: R) -> Self {
        let result = result.into().map(|v| serde_json::to_value(v).unwrap());
        Self {
            error: None,
            id: Some(id),
            jsonrpc: "2.0".to_string(),
            result,
        }
    }
}

pub trait FromReq: Sized {
    const METHOD: &'static str;

    fn from_req(req: RequestMessage) -> OneOf<(ReqId, Self), RequestMessage>;
    fn can_cast(req: &RequestMessage) -> bool {
        Self::METHOD == req.method
    }
    fn handle<C, F, I>(req: RequestMessage, context: &mut C, mut f: F) -> OneOf<I, RequestMessage>
    where
        F: FnMut(&mut C, ReqId, Self) -> I,
    {
        Self::from_req(req).map(|(id, params)| f(context, id, params))
    }
}

macro_rules! impl_req {
    ($type:ty, $method:literal) => {
        impl FromReq for $type {
            const METHOD: &'static str = $method;

            fn from_req(req: RequestMessage) -> OneOf<(ReqId, Self), RequestMessage> {
                if Self::can_cast(&req) {
                    let RequestMessage { id, params, .. } = req;
                    let params: Self =
                        serde_json::from_value(params.unwrap_or_else(|| serde_json::Value::Null))
                            .unwrap();
                    OneOf::One((id, params))
                } else {
                    OneOf::Other(req)
                }
            }
        }
    };
}

impl_req!(CancelParams, "$/cancelRequest");
impl_req!(ProgressParams, "$/progress");
impl_req!(LogTraceParams, "$/logTrace");
impl_req!(SetTraceParams, "$/setTrace");

impl_req!(InitializeParams, "initialize");
impl_req!(ShowMessageRequestParams, "window/showMessageRequest");
impl_req!(ShowDocumentParams, "window/showDocument");
impl_req!(RegistrationParams, "client/registerCapability");
impl_req!(UnregistrationParams, "client/unregisterCapability");
impl_req!(ConfigurationParams, "workspace/configuration");
impl_req!(WorkspaceSymbolParams, "workspace/symbol");
impl_req!(ExecuteCommandParams, "workspace/executeCommand");
impl_req!(ApplyWorkspaceEditParams, "workspace/applyEdit");
impl_req!(CreateFilesParams, "workspace/willCreateFiles");
impl_req!(RenameFilesParams, "workspace/willRenameFiles");
impl_req!(DeleteFilesParams, "workspace/willDeleteFiles");
impl_req!(WillSaveTextDocumentParams, "textDocument/willSaveWaitUntil");
impl_req!(CompletionParams, "textDocument/completion");
impl_req!(CompletionItem, "completionItem/resolve");
impl_req!(HoverParams, "textDocument/hover");
impl_req!(SignatureHelpParams, "textDocument/signatureHelp");
impl_req!(DeclarationParams, "textDocument/declaration");
impl_req!(DefinitionParams, "textDocument/definition");
impl_req!(TypeDefinitionParams, "textDocument/typeDefinition");
impl_req!(ImplementationParams, "textDocument/implementation");
impl_req!(ReferenceParams, "textDocument/references");
impl_req!(DocumentHighlightParams, "textDocument/documentHighlight");
impl_req!(DocumentSymbolParams, "textDocument/documentSymbol");
impl_req!(CodeActionParams, "textDocument/codeAction");
impl_req!(CodeAction, "codeAction/resolve");
impl_req!(CodeLensParams, "textDocument/codeLens");
impl_req!(CodeLens, "codeLens/resolve");
impl_req!(DocumentLinkParams, "textDocument/documentLink");
impl_req!(DocumentLink, "documentLink/resolve");
impl_req!(DocumentFormattingParams, "textDocument/formatting");
impl_req!(
    DocumentOnTypeFormattingParams,
    "textDocument/onTypeFormatting"
);
impl_req!(RenameParams, "textDocument/rename");
impl_req!(PrepareRenameParams, "textDocument/prepareRename");
