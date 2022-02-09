use super::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[doc = " A special text edit with an additional change annotation."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AnnotatedTextEdit {
    #[doc = " The actual annotation identifier."]
    #[serde(rename = "annotationId")]
    pub annotation_id: ChangeAnnotationIdentifier,
    #[doc = " The string to be inserted. For delete operations use an empty string."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range of the text document to be manipulated. To insert text into a document create a "]
    #[doc = " range where start === end."]
    pub range: Range,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ApplyWorkspaceEditParams {
    #[doc = " The edits to apply."]
    pub edit: WorkspaceEdit,
    #[doc = " An optional label of the workspace edit. This label is presented in the user interface for "]
    #[doc = " example on an undo stack to undo the workspace edit."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ApplyWorkspaceEditResponse {
    #[doc = " Indicates whether the edit was applied or not."]
    pub applied: bool,
    #[doc = " Depending on the client's failure handling strategy `failedChange` might contain the index "]
    #[doc = " of the change that failed. This property is only available if the client signals a "]
    #[doc = " `failureHandling` strategy in its client capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failedChange")]
    pub failed_change: Option<Uinteger>,
    #[doc = " An optional textual description for why the edit was not applied. This may be used by the "]
    #[doc = " server for diagnostic logging or to provide a suitable error for a request that triggered "]
    #[doc = " the edit."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CallHierarchyClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyIncomingCall {
    #[doc = " The item that makes the call."]
    pub from: CallHierarchyItem,
    #[doc = " The ranges at which the calls appear. This is relative to the caller denoted by "]
    #[doc = " [`this.from`](#CallHierarchyIncomingCall.from)."]
    #[serde(rename = "fromRanges")]
    pub from_ranges: Vec<Range>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyIncomingCallsParams {
    pub item: CallHierarchyItem,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyItem {
    #[doc = " A data entry field that is preserved between a call hierarchy prepare and incoming calls or "]
    #[doc = " outgoing calls requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " More detail for this item, e.g. the signature of a function."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = " The kind of this item."]
    pub kind: SymbolKind,
    #[doc = " The name of this item."]
    pub name: String,
    #[doc = " The range enclosing this symbol not including leading/trailing whitespace but everything "]
    #[doc = " else, e.g. comments and code."]
    pub range: Range,
    #[doc = " The range that should be selected and revealed when this symbol is being picked, e.g. the "]
    #[doc = " name of a function. Must be contained by the [`range`](#CallHierarchyItem.range)."]
    #[serde(rename = "selectionRange")]
    pub selection_range: Range,
    #[doc = " Tags for this item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SymbolTag>>,
    #[doc = " The resource identifier of this item."]
    pub uri: DocumentUri,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CallHierarchyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyOutgoingCall {
    #[doc = " The range at which this item is called. This is the range relative to the caller, e.g the "]
    #[doc = " item passed to `callHierarchy/outgoingCalls` request."]
    #[serde(rename = "fromRanges")]
    pub from_ranges: Vec<Range>,
    #[doc = " The item that is called."]
    pub to: CallHierarchyItem,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyOutgoingCallsParams {
    pub item: CallHierarchyItem,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyPrepareParams {
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CallHierarchyRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " The id used to register the request. The id can be used to deregister the request again. "]
    #[doc = " See also Registration#id."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CancelParams {
    #[doc = " The request id to cancel."]
    pub id: Option<ReqId>,
}
#[doc = " Additional information that describes document changes."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ChangeAnnotation {
    #[doc = " A human-readable string which is rendered less prominent in the user interface."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = " A human-readable string describing the actual change. The string is rendered prominent in "]
    #[doc = " the user interface."]
    pub label: String,
    #[doc = " A flag which indicates that user confirmation is needed before applying the change."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "needsConfirmation")]
    pub needs_confirmation: Option<bool>,
}
#[doc = " An identifier referring to a change annotation managed by a workspace edit."]
pub type ChangeAnnotationIdentifier = String;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ClientCapabilitiesGeneral {
    #[doc = " Client capabilities specific to the client's markdown parser."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownClientCapabilities>,
    #[doc = " Client capabilities specific to regular expressions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "regularExpressions")]
    pub regular_expressions: Option<RegularExpressionsClientCapabilities>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ClientCapabilitiesWindow {
    #[doc = " Client capabilities for the show document request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showDocument")]
    pub show_document: Option<ShowDocumentClientCapabilities>,
    #[doc = " Capabilities specific to the showMessage request"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showMessage")]
    pub show_message: Option<ShowMessageRequestClientCapabilities>,
    #[doc = " Whether client supports handling progress notifications. If set servers are allowed to "]
    #[doc = " report in `workDoneProgress` property in the request specific server capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ClientCapabilitiesWorkspaceFileOperations {
    #[doc = " The client has support for sending didCreateFiles notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didCreate")]
    pub did_create: Option<bool>,
    #[doc = " The client has support for sending didDeleteFiles notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didDelete")]
    pub did_delete: Option<bool>,
    #[doc = " The client has support for sending didRenameFiles notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didRename")]
    pub did_rename: Option<bool>,
    #[doc = " Whether the client supports dynamic registration for file requests/notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client has support for sending willCreateFiles requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willCreate")]
    pub will_create: Option<bool>,
    #[doc = " The client has support for sending willDeleteFiles requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willDelete")]
    pub will_delete: Option<bool>,
    #[doc = " The client has support for sending willRenameFiles requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willRename")]
    pub will_rename: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ClientCapabilitiesWorkspace {
    #[doc = " The client supports applying batch edits to the workspace by supporting the request "]
    #[doc = " 'workspace/applyEdit'"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "applyEdit")]
    pub apply_edit: Option<bool>,
    #[doc = " Capabilities specific to the code lens requests scoped to the workspace."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeLens")]
    pub code_lens: Option<CodeLensWorkspaceClientCapabilities>,
    #[doc = " The client supports `workspace/configuration` requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<bool>,
    #[doc = " Capabilities specific to the `workspace/didChangeConfiguration` notification."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didChangeConfiguration")]
    pub did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,
    #[doc = " Capabilities specific to the `workspace/didChangeWatchedFiles` notification."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didChangeWatchedFiles")]
    pub did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,
    #[doc = " Capabilities specific to the `workspace/executeCommand` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executeCommand")]
    pub execute_command: Option<ExecuteCommandClientCapabilities>,
    #[doc = " The client has support for file requests/notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileOperations")]
    pub file_operations: Option<ClientCapabilitiesWorkspaceFileOperations>,
    #[doc = " Capabilities specific to the semantic token requests scoped to the workspace."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticTokens")]
    pub semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,
    #[doc = " Capabilities specific to the `workspace/symbol` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<WorkspaceSymbolClientCapabilities>,
    #[doc = " Capabilities specific to `WorkspaceEdit`s"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspaceEdit")]
    pub workspace_edit: Option<WorkspaceEditClientCapabilities>,
    #[doc = " The client has support for workspace folders."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspaceFolders")]
    pub workspace_folders: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ClientCapabilities {
    #[doc = " Experimental client capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
    #[doc = " General client capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<ClientCapabilitiesGeneral>,
    #[doc = " Text document specific client capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textDocument")]
    pub text_document: Option<TextDocumentClientCapabilities>,
    #[doc = " Window specific client capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<ClientCapabilitiesWindow>,
    #[doc = " Workspace specific client capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<ClientCapabilitiesWorkspace>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionDisabled {
    #[doc = " Human readable description of why the code action is currently disabled."]
    #[doc = " "]
    #[doc = " This is displayed in the code actions UI."]
    pub reason: String,
}
#[doc = " A code action represents a change that can be performed in code, e.g. to fix a problem or to "]
#[doc = " refactor code."]
#[doc = " "]
#[doc = " A CodeAction must set either `edit` and/or a `command`. If both are supplied, the `edit` is "]
#[doc = " applied first, then the `command` is executed."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeAction {
    #[doc = " A command this code action executes. If a code action provides an edit and a command, first "]
    #[doc = " the edit is executed and then the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[doc = " A data entry field that is preserved on a code action between a `textDocument/codeAction` "]
    #[doc = " and a `codeAction/resolve` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " The diagnostics that this code action resolves."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Vec<Diagnostic>>,
    #[doc = " Marks that the code action cannot currently be applied."]
    #[doc = " "]
    #[doc = " Clients should follow the following guidelines regarding disabled code actions:"]
    #[doc = " "]
    #[doc = " - Disabled code actions are not shown in automatic lightbulbs code   action menus."]
    #[doc = " "]
    #[doc = " - Disabled actions are shown as faded out in the code action menu when   the user request a "]
    #[doc = " more specific type of code action, such as   refactorings."]
    #[doc = " "]
    #[doc = " - If the user has a keybinding that auto applies a code action and only   a disabled code "]
    #[doc = " actions are returned, the client should show the user   an error message with `reason` in "]
    #[doc = " the editor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<CodeActionDisabled>,
    #[doc = " The workspace edit this code action performs."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<WorkspaceEdit>,
    #[doc = " Marks this as a preferred action. Preferred actions are used by the `auto fix` command and "]
    #[doc = " can be targeted by keybindings."]
    #[doc = " "]
    #[doc = " A quick fix should be marked preferred if it properly addresses the underlying error. A "]
    #[doc = " refactoring should be marked preferred if it is the most reasonable choice of actions to "]
    #[doc = " take."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isPreferred")]
    pub is_preferred: Option<bool>,
    #[doc = " The kind of the code action."]
    #[doc = " "]
    #[doc = " Used to filter code actions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CodeActionKind>,
    #[doc = " A short, human-readable, title for this code action."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionClientCapabilitiesCodeActionLiteralSupportCodeActionKind {
    #[doc = " The code action kind values the client supports. When this property exists the client also "]
    #[doc = " guarantees that it will handle values outside its set gracefully and falls back to a "]
    #[doc = " default value when unknown."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<CodeActionKind>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionClientCapabilitiesCodeActionLiteralSupport {
    #[doc = " The code action kind is supported with the following value set."]
    #[serde(rename = "codeActionKind")]
    pub code_action_kind: CodeActionClientCapabilitiesCodeActionLiteralSupportCodeActionKind,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionClientCapabilitiesResolveSupport {
    #[doc = " The properties that a client can resolve lazily."]
    pub properties: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CodeActionClientCapabilities {
    #[doc = " The client supports code action literals as a valid response of the "]
    #[doc = " `textDocument/codeAction` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeActionLiteralSupport")]
    pub code_action_literal_support: Option<CodeActionClientCapabilitiesCodeActionLiteralSupport>,
    #[doc = " Whether code action supports the `data` property which is preserved between a "]
    #[doc = " `textDocument/codeAction` and a `codeAction/resolve` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataSupport")]
    pub data_support: Option<bool>,
    #[doc = " Whether code action supports the `disabled` property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disabledSupport")]
    pub disabled_support: Option<bool>,
    #[doc = " Whether code action supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " Whether the client honors the change annotations in text edits and resource operations "]
    #[doc = " returned via the `CodeAction#edit` property by for example presenting the workspace edit in "]
    #[doc = " the user interface and asking for confirmation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "honorsChangeAnnotations")]
    pub honors_change_annotations: Option<bool>,
    #[doc = " Whether code action supports the `isPreferred` property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isPreferredSupport")]
    pub is_preferred_support: Option<bool>,
    #[doc = " Whether the client supports resolving additional code action properties via a separate "]
    #[doc = " `codeAction/resolve` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveSupport")]
    pub resolve_support: Option<CodeActionClientCapabilitiesResolveSupport>,
}
#[doc = " Contains additional diagnostic information about the context in which a code action is run."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionContext {
    #[doc = " An array of diagnostics known on the client side overlapping the range provided to the "]
    #[doc = " `textDocument/codeAction` request. They are provided so that the server knows which errors "]
    #[doc = " are currently presented to the user for the given range. There is no guarantee that these "]
    #[doc = " accurately reflect the error state of the resource. The primary parameter to compute code "]
    #[doc = " actions is the provided range."]
    pub diagnostics: Vec<Diagnostic>,
    #[doc = " Requested kind of actions to return."]
    #[doc = " "]
    #[doc = " Actions not of this kind are filtered out by the client before being shown. So servers can "]
    #[doc = " omit computing them."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<CodeActionKind>>,
}
#[doc = " The kind of a code action."]
#[doc = " "]
#[doc = " Kinds are a hierarchical list of identifiers separated by `.`, e.g. "]
#[doc = " `\"refactor.extract.function\"`."]
#[doc = " "]
#[doc = " The set of kinds is open and client needs to announce the kinds it supports to the server "]
#[doc = " during initialization. "]
#[doc = "  A set of predefined code action kinds."]
pub type CodeActionKind = String;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CodeActionOptions {
    #[doc = " CodeActionKinds that this server may return."]
    #[doc = " "]
    #[doc = " The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server may list "]
    #[doc = " out every specific kind they provide."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeActionKinds")]
    pub code_action_kinds: Option<Vec<CodeActionKind>>,
    #[doc = " The server provides support to resolve additional information for a code action."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " Params for the CodeActionRequest"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionParams {
    #[doc = " Context carrying additional information."]
    pub context: CodeActionContext,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The range for which the command was invoked."]
    pub range: Range,
    #[doc = " The document in which the command was invoked."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionRegistrationOptions {
    #[doc = " CodeActionKinds that this server may return."]
    #[doc = " "]
    #[doc = " The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server may list "]
    #[doc = " out every specific kind they provide."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeActionKinds")]
    pub code_action_kinds: Option<Vec<CodeActionKind>>,
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " The server provides support to resolve additional information for a code action."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " Structure to capture a description for an error code."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeDescription {
    #[doc = " An URI to open with more information about the diagnostic error."]
    pub href: Uri,
}
#[doc = " A code lens represents a command that should be shown along with source text, like the number "]
#[doc = " of references, a way to run tests, etc."]
#[doc = " "]
#[doc = " A code lens is _unresolved_ when no command is associated to it. For performance reasons the "]
#[doc = " creation of a code lens and resolving should be done in two stages."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeLens {
    #[doc = " The command this code lens represents."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[doc = " A data entry field that is preserved on a code lens item between a code lens and a code "]
    #[doc = " lens resolve request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " The range in which this code lens is valid. Should only span a single line."]
    pub range: Range,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CodeLensClientCapabilities {
    #[doc = " Whether code lens supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CodeLensOptions {
    #[doc = " Code lens has a resolve provider as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeLensParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The document to request code lens for."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeLensRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " Code lens has a resolve provider as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CodeLensWorkspaceClientCapabilities {
    #[doc = " Whether the client implementation supports a refresh request sent from the server to the "]
    #[doc = " client."]
    #[doc = " "]
    #[doc = " Note that this event is global and will force the client to refresh all code lenses "]
    #[doc = " currently shown. It should be used with absolute care and is useful for situation where a "]
    #[doc = " server for example detect a project wide change that requires such a calculation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refreshSupport")]
    pub refresh_support: Option<bool>,
}
#[doc = " Represents a color in RGBA space."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Color {
    #[doc = " The alpha component of this color in the range [0-1]."]
    pub alpha: Decimal,
    #[doc = " The blue component of this color in the range [0-1]."]
    pub blue: Decimal,
    #[doc = " The green component of this color in the range [0-1]."]
    pub green: Decimal,
    #[doc = " The red component of this color in the range [0-1]."]
    pub red: Decimal,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ColorInformation {
    #[doc = " The actual color value for this color range."]
    pub color: Color,
    #[doc = " The range in the document where this color appears."]
    pub range: Range,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ColorPresentation {
    #[doc = " An optional array of additional [text edits](#TextEdit) that are applied when selecting "]
    #[doc = " this color presentation. Edits must not overlap with the main "]
    #[doc = " [edit](#ColorPresentation.textEdit) nor with themselves."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalTextEdits")]
    pub additional_text_edits: Option<Vec<TextEdit>>,
    #[doc = " The label of this color presentation. It will be shown on the color picker header. By "]
    #[doc = " default this is also the text that is inserted when selecting this color presentation."]
    pub label: String,
    #[doc = " An [edit](#TextEdit) which is applied to a document when selecting this presentation for "]
    #[doc = " the color. When `falsy` the [label](#ColorPresentation.label) is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textEdit")]
    pub text_edit: Option<TextEdit>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ColorPresentationParams {
    #[doc = " The color information to request presentations for."]
    pub color: Color,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The range where the color would be inserted. Serves as a context."]
    pub range: Range,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Command {
    #[doc = " Arguments that the command handler should be invoked with."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
    #[doc = " The identifier of the actual command handler."]
    pub command: String,
    #[doc = " Title of the command, like `save`."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionClientCapabilitiesCompletionItemInsertTextModeSupport {
    #[serde(rename = "valueSet")]
    pub value_set: Vec<InsertTextMode>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionClientCapabilitiesCompletionItemResolveSupport {
    #[doc = " The properties that a client can resolve lazily."]
    pub properties: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionClientCapabilitiesCompletionItemTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<CompletionItemTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CompletionClientCapabilitiesCompletionItem {
    #[doc = " Client supports commit characters on a completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "commitCharactersSupport")]
    pub commit_characters_support: Option<bool>,
    #[doc = " Client supports the deprecated property on a completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deprecatedSupport")]
    pub deprecated_support: Option<bool>,
    #[doc = " Client supports the following content formats for the documentation property. The order "]
    #[doc = " describes the preferred format of the client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentationFormat")]
    pub documentation_format: Option<Vec<MarkupKind>>,
    #[doc = " Client supports insert replace edit to control different behavior if a completion item is "]
    #[doc = " inserted in the text or should replace text."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertReplaceSupport")]
    pub insert_replace_support: Option<bool>,
    #[doc = " The client supports the `insertTextMode` property on a completion item to override the "]
    #[doc = " whitespace handling mode as defined by the client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertTextModeSupport")]
    pub insert_text_mode_support:
        Option<CompletionClientCapabilitiesCompletionItemInsertTextModeSupport>,
    #[doc = " Client supports the preselect property on a completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preselectSupport")]
    pub preselect_support: Option<bool>,
    #[doc = " Indicates which properties a client can resolve lazily on a completion item. Before version "]
    #[doc = " 3.16.0 only the predefined properties `documentation` and `detail` could be resolved "]
    #[doc = " lazily."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveSupport")]
    pub resolve_support: Option<CompletionClientCapabilitiesCompletionItemResolveSupport>,
    #[doc = " Client supports snippets as insert text."]
    #[doc = " "]
    #[doc = " A snippet can define tab stops and placeholders with `$1`, `$2` and `${3:foo}`. `$0` "]
    #[doc = " defines the final tab stop, it defaults to the end of the snippet. Placeholders with equal "]
    #[doc = " identifiers are linked, that is typing in one will update others too."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snippetSupport")]
    pub snippet_support: Option<bool>,
    #[doc = " Client supports the tag property on a completion item. Clients supporting tags have to "]
    #[doc = " handle unknown tags gracefully. Clients especially need to preserve unknown tags when "]
    #[doc = " sending a completion item back to the server in a resolve call."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagSupport")]
    pub tag_support: Option<CompletionClientCapabilitiesCompletionItemTagSupport>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CompletionClientCapabilitiesCompletionItemKind {
    #[doc = " The completion item kind values the client supports. When this property exists the client "]
    #[doc = " also guarantees that it will handle values outside its set gracefully and falls back to a "]
    #[doc = " default value when unknown."]
    #[doc = " "]
    #[doc = " If this property is not present the client only supports the completion items kinds from "]
    #[doc = " `Text` to `Reference` as defined in the initial version of the protocol."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueSet")]
    pub value_set: Option<Vec<CompletionItemKind>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CompletionClientCapabilities {
    #[doc = " The client supports the following `CompletionItem` specific capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completionItem")]
    pub completion_item: Option<CompletionClientCapabilitiesCompletionItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completionItemKind")]
    pub completion_item_kind: Option<CompletionClientCapabilitiesCompletionItemKind>,
    #[doc = " The client supports to send additional context information for a `textDocument/completion` "]
    #[doc = " request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextSupport")]
    pub context_support: Option<bool>,
    #[doc = " Whether completion supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[doc = " Contains additional information about the context in which a completion request is triggered."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionContext {
    #[doc = " The trigger character (a single character) that has trigger code complete. Is undefined if "]
    #[doc = " `triggerKind !== CompletionTriggerKind.TriggerCharacter`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacter")]
    pub trigger_character: Option<String>,
    #[doc = " How the completion was triggered."]
    #[serde(rename = "triggerKind")]
    pub trigger_kind: CompletionTriggerKind,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionItem {
    #[doc = " An optional array of additional text edits that are applied when selecting this completion. "]
    #[doc = " Edits must not overlap (including the same insert position) with the main edit nor with "]
    #[doc = " themselves."]
    #[doc = " "]
    #[doc = " Additional text edits should be used to change text unrelated to the current cursor "]
    #[doc = " position (for example adding an import statement at the top of the file if the completion "]
    #[doc = " item will insert an unqualified type)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalTextEdits")]
    pub additional_text_edits: Option<Vec<TextEdit>>,
    #[doc = " An optional command that is executed *after* inserting this completion."]
    #[doc = " *Note* that additional modifications to the current document should be described with the "]
    #[doc = " additionalTextEdits-property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[doc = " An optional set of characters that when pressed while this completion is active will accept "]
    #[doc = " it first and then type that character. *Note* that all commit characters should have "]
    #[doc = " `length=1` and that superfluous characters will be ignored."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "commitCharacters")]
    pub commit_characters: Option<Vec<String>>,
    #[doc = " A data entry field that is preserved on a completion item between a completion and a "]
    #[doc = " completion resolve request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " Indicates if this item is deprecated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = " A human-readable string with additional information about this item, like type or symbol "]
    #[doc = " information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = " A human-readable string that represents a doc-comment."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<OneOf<String, MarkupContent>>,
    #[doc = " A string that should be used when filtering a set of completion items. When `falsy` the "]
    #[doc = " label is used as the filter text for this item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filterText")]
    pub filter_text: Option<String>,
    #[doc = " A string that should be inserted into a document when selecting this completion. When "]
    #[doc = " `falsy` the label is used as the insert text for this item."]
    #[doc = " "]
    #[doc = " The `insertText` is subject to interpretation by the client side. Some tools might not take "]
    #[doc = " the string literally. For example VS Code when code complete is requested in this example "]
    #[doc = " `con<cursor position>` and a completion item with an `insertText` of `console` is provided "]
    #[doc = " it will only insert `sole`. Therefore it is recommended to use `textEdit` instead since it "]
    #[doc = " avoids additional client side interpretation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertText")]
    pub insert_text: Option<String>,
    #[doc = " The format of the insert text. The format applies to both the `insertText` property and the "]
    #[doc = " `newText` property of a provided `textEdit`. If omitted defaults to "]
    #[doc = " `InsertTextFormat.PlainText`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertTextFormat")]
    pub insert_text_format: Option<InsertTextFormat>,
    #[doc = " How whitespace and indentation is handled during completion item insertion. If not provided "]
    #[doc = " the client's default value is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertTextMode")]
    pub insert_text_mode: Option<InsertTextMode>,
    #[doc = " The kind of this completion item. Based of the kind an icon is chosen by the editor. The "]
    #[doc = " standardized set of available values is defined in `CompletionItemKind`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CompletionItemKind>,
    #[doc = " The label of this completion item. By default also the text that is inserted when selecting "]
    #[doc = " this completion."]
    pub label: String,
    #[doc = " Select this item when showing."]
    #[doc = " "]
    #[doc = " *Note* that only one completion item can be selected and that the tool / client decides "]
    #[doc = " which item that is. The rule is that the *first* item of those that match best is selected."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preselect: Option<bool>,
    #[doc = " A string that should be used when comparing this item with other items. When `falsy` the "]
    #[doc = " label is used as the sort text for this item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortText")]
    pub sort_text: Option<String>,
    #[doc = " Tags for this completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CompletionItemTag>>,
    #[doc = " An edit which is applied to a document when selecting this completion. When an edit is "]
    #[doc = " provided the value of `insertText` is ignored."]
    #[doc = " "]
    #[doc = " *Note:* The range of the edit must be a single line range and it must contain the position "]
    #[doc = " at which completion has been requested."]
    #[doc = " "]
    #[doc = " Most editors support two different operations when accepting a completion item. One is to "]
    #[doc = " insert a completion text and the other is to replace an existing text with a completion "]
    #[doc = " text. Since this can usually not be predetermined by a server it can report both ranges. "]
    #[doc = " Clients need to signal support for `InsertReplaceEdit`s via the "]
    #[doc = " `textDocument.completion.insertReplaceSupport` client capability property."]
    #[doc = " "]
    #[doc = " *Note 1:* The text edit's range as well as both ranges from an insert replace edit must be "]
    #[doc = " a [single line] and they must contain the position at which completion has been requested."]
    #[doc = " *Note 2:* If an `InsertReplaceEdit` is returned the edit's insert range must be a prefix of "]
    #[doc = " the edit's replace range, that means it must be contained and starting at the same "]
    #[doc = " position."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textEdit")]
    pub text_edit: Option<OneOf<TextEdit, InsertReplaceEdit>>,
}
#[doc = " The kind of a completion entry."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}
#[doc = " Completion item tags are extra annotations that tweak the rendering of a completion item."]
pub type CompletionItemTag = f64;
#[doc = " Represents a collection of [completion items](#CompletionItem) to be presented in the editor."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionList {
    #[doc = " This list is not complete. Further typing should result in recomputing this list."]
    #[doc = " "]
    #[doc = " Recomputed lists have all their items replaced (not appended) in the incomplete completion "]
    #[doc = " sessions."]
    #[serde(rename = "isIncomplete")]
    pub is_incomplete: bool,
    #[doc = " The completion items."]
    pub items: Vec<CompletionItem>,
}
#[doc = " Completion options."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CompletionOptions {
    #[doc = " The list of all possible characters that commit a completion. This field can be used if "]
    #[doc = " clients don't support individual commit characters per completion item. See client "]
    #[doc = " capability `completion.completionItem.commitCharactersSupport`."]
    #[doc = " "]
    #[doc = " If a server provides both `allCommitCharacters` and commit characters on an individual "]
    #[doc = " completion item the ones on the completion item win."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allCommitCharacters")]
    pub all_commit_characters: Option<Vec<String>>,
    #[doc = " The server provides support to resolve additional information for a completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[doc = " Most tools trigger completion request automatically without explicitly requesting it using "]
    #[doc = " a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user starts to type an "]
    #[doc = " identifier. For example if the user types `c` in a JavaScript file code complete will "]
    #[doc = " automatically pop up present `console` besides others as a completion item. Characters that "]
    #[doc = " make up identifiers don't need to be listed here."]
    #[doc = " "]
    #[doc = " If code complete should automatically be trigger on characters not being valid inside an "]
    #[doc = " identifier (for example `.` in JavaScript) list them in `triggerCharacters`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionParams {
    #[doc = " The completion context. This is only available if the client specifies to send this using "]
    #[doc = " the client capability `completion.contextSupport === true`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<CompletionContext>,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionRegistrationOptions {
    #[doc = " The list of all possible characters that commit a completion. This field can be used if "]
    #[doc = " clients don't support individual commit characters per completion item. See client "]
    #[doc = " capability `completion.completionItem.commitCharactersSupport`."]
    #[doc = " "]
    #[doc = " If a server provides both `allCommitCharacters` and commit characters on an individual "]
    #[doc = " completion item the ones on the completion item win."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allCommitCharacters")]
    pub all_commit_characters: Option<Vec<String>>,
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " The server provides support to resolve additional information for a completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[doc = " Most tools trigger completion request automatically without explicitly requesting it using "]
    #[doc = " a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user starts to type an "]
    #[doc = " identifier. For example if the user types `c` in a JavaScript file code complete will "]
    #[doc = " automatically pop up present `console` besides others as a completion item. Characters that "]
    #[doc = " make up identifiers don't need to be listed here."]
    #[doc = " "]
    #[doc = " If code complete should automatically be trigger on characters not being valid inside an "]
    #[doc = " identifier (for example `.` in JavaScript) list them in `triggerCharacters`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " How a completion was triggered"]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum CompletionTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}
