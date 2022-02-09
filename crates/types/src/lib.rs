use serde::{Deserialize, Serialize};

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotatedTextEditRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotatedTextEditRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotatedTextEditRange {
    #[doc = " The range's end position."]
    pub end: AnnotatedTextEditRangeEnd,
    #[doc = " The range's start position."]
    pub start: AnnotatedTextEditRangeStart,
}
#[doc = " A special text edit with an additional change annotation."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotatedTextEdit {
    #[doc = " The actual annotation identifier."]
    #[serde(rename = "annotationId")]
    pub annotation_id: ChangeAnnotationIdentifier,
    #[doc = " The string to be inserted. For delete operations use an empty string."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range of the text document to be manipulated. To insert text into a document create a "]
    #[doc = " range where start === end."]
    pub range: AnnotatedTextEditRange,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyWorkspaceEditParams {
    #[doc = " The edits to apply."]
    pub edit: WorkspaceEdit,
    #[doc = " An optional label of the workspace edit. This label is presented in the user interface for "]
    #[doc = " example on an undo stack to undo the workspace edit."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyIncomingCallItemFromRangesEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyIncomingCallItemFromRangesStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyIncomingCallItemFromRanges {
    #[doc = " The range's end position."]
    pub end: CallHierarchyIncomingCallItemFromRangesEnd,
    #[doc = " The range's start position."]
    pub start: CallHierarchyIncomingCallItemFromRangesStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyIncomingCall {
    #[doc = " The item that makes the call."]
    pub from: CallHierarchyItem,
    #[doc = " The ranges at which the calls appear. This is relative to the caller denoted by "]
    #[doc = " [`this.from`](#CallHierarchyIncomingCall.from)."]
    #[serde(rename = "fromRanges")]
    pub from_ranges: Vec<CallHierarchyIncomingCallItemFromRanges>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyIncomingCallsParams {
    pub item: CallHierarchyItem,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyItemRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyItemRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyItemRange {
    #[doc = " The range's end position."]
    pub end: CallHierarchyItemRangeEnd,
    #[doc = " The range's start position."]
    pub start: CallHierarchyItemRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyItemSelectionRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyItemSelectionRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyItemSelectionRange {
    #[doc = " The range's end position."]
    pub end: CallHierarchyItemSelectionRangeEnd,
    #[doc = " The range's start position."]
    pub start: CallHierarchyItemSelectionRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
    pub range: CallHierarchyItemRange,
    #[doc = " The range that should be selected and revealed when this symbol is being picked, e.g. the "]
    #[doc = " name of a function. Must be contained by the [`range`](#CallHierarchyItem.range)."]
    #[serde(rename = "selectionRange")]
    pub selection_range: CallHierarchyItemSelectionRange,
    #[doc = " Tags for this item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SymbolTag>>,
    #[doc = " The resource identifier of this item."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyOutgoingCallItemFromRangesEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyOutgoingCallItemFromRangesStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyOutgoingCallItemFromRanges {
    #[doc = " The range's end position."]
    pub end: CallHierarchyOutgoingCallItemFromRangesEnd,
    #[doc = " The range's start position."]
    pub start: CallHierarchyOutgoingCallItemFromRangesStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyOutgoingCall {
    #[doc = " The range at which this item is called. This is the range relative to the caller, e.g the "]
    #[doc = " item passed to `callHierarchy/outgoingCalls` request."]
    #[serde(rename = "fromRanges")]
    pub from_ranges: Vec<CallHierarchyOutgoingCallItemFromRanges>,
    #[doc = " The item that is called."]
    pub to: CallHierarchyItem,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyOutgoingCallsParams {
    pub item: CallHierarchyItem,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyPrepareParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyPrepareParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallHierarchyPrepareParams {
    #[doc = " The position inside the text document."]
    pub position: CallHierarchyPrepareParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: CallHierarchyPrepareParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[doc = " Additional information that describes document changes."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionCommand {
    #[doc = " Arguments that the command handler should be invoked with."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
    #[doc = " The identifier of the actual command handler."]
    pub command: String,
    #[doc = " Title of the command, like `save`."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct CodeAction {
    #[doc = " A command this code action executes. If a code action provides an edit and a command, first "]
    #[doc = " the edit is executed and then the command."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CodeActionCommand>,
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
#[serde(deny_unknown_fields)]
pub struct CodeActionClientCapabilitiesCodeActionLiteralSupportCodeActionKind {
    #[doc = " The code action kind values the client supports. When this property exists the client also "]
    #[doc = " guarantees that it will handle values outside its set gracefully and falls back to a "]
    #[doc = " default value when unknown."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<CodeActionKind>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionClientCapabilitiesCodeActionLiteralSupport {
    #[doc = " The code action kind is supported with the following value set."]
    #[serde(rename = "codeActionKind")]
    pub code_action_kind: CodeActionClientCapabilitiesCodeActionLiteralSupportCodeActionKind,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionClientCapabilitiesResolveSupport {
    #[doc = " The properties that a client can resolve lazily."]
    pub properties: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionParamsRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionParamsRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionParamsRange {
    #[doc = " The range's end position."]
    pub end: CodeActionParamsRangeEnd,
    #[doc = " The range's start position."]
    pub start: CodeActionParamsRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[doc = " Params for the CodeActionRequest"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeActionParams {
    #[doc = " Context carrying additional information."]
    pub context: CodeActionContext,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The range for which the command was invoked."]
    pub range: CodeActionParamsRange,
    #[doc = " The document in which the command was invoked."]
    #[serde(rename = "textDocument")]
    pub text_document: CodeActionParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct CodeDescription {
    #[doc = " An URI to open with more information about the diagnostic error."]
    pub href: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CodeLensClientCapabilities {
    #[doc = " Whether code lens supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionClientCapabilitiesCompletionItemInsertTextModeSupport {
    #[serde(rename = "valueSet")]
    pub value_set: Vec<InsertTextMode>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionClientCapabilitiesCompletionItemResolveSupport {
    #[doc = " The properties that a client can resolve lazily."]
    pub properties: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionClientCapabilitiesCompletionItemTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<CompletionItemTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct CompletionItemItemAdditionalTextEditsRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionItemItemAdditionalTextEditsRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionItemItemAdditionalTextEditsRange {
    #[doc = " The range's end position."]
    pub end: CompletionItemItemAdditionalTextEditsRangeEnd,
    #[doc = " The range's start position."]
    pub start: CompletionItemItemAdditionalTextEditsRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionItemItemAdditionalTextEdits {
    #[doc = " The string to be inserted. For delete operations use an empty string."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range of the text document to be manipulated. To insert text into a document create a "]
    #[doc = " range where start === end."]
    pub range: CompletionItemItemAdditionalTextEditsRange,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionItemCommand {
    #[doc = " Arguments that the command handler should be invoked with."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
    #[doc = " The identifier of the actual command handler."]
    pub command: String,
    #[doc = " Title of the command, like `save`."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
    pub additional_text_edits: Option<Vec<CompletionItemItemAdditionalTextEdits>>,
    #[doc = " An optional command that is executed *after* inserting this completion."]
    #[doc = " *Note* that additional modifications to the current document should be described with the "]
    #[doc = " additionalTextEdits-property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CompletionItemCommand>,
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
    pub documentation: Option<serde_json::Value>,
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
    pub text_edit: Option<serde_json::Value>,
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct CompletionParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionParams {
    #[doc = " The completion context. This is only available if the client specifies to send this using "]
    #[doc = " the client capability `completion.contextSupport === true`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<CompletionContext>,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: CompletionParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: CompletionParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationItem {
    #[doc = " The scope to get the configuration section for."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopeUri")]
    pub scope_uri: Option<String>,
    #[doc = " The configuration section asked for."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationParams {
    pub items: Vec<ConfigurationItem>,
}
#[doc = " Create file operation"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateFile {
    #[doc = " An optional annotation identifer describing the operation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annotationId")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
    #[doc = " A create"]
    pub kind: String,
    #[doc = " Additional options"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateFileOptions>,
    #[doc = " The resource to create."]
    pub uri: String,
}
#[doc = " Options to create a file."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateFileOptions {
    #[doc = " Ignore if exists."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreIfExists")]
    pub ignore_if_exists: Option<bool>,
    #[doc = " Overwrite existing file. Overwrite wins over `ignoreIfExists`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}
#[doc = " The parameters sent in notifications/requests for user-initiated creation of files."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateFilesParams {
    #[doc = " An array of all files/folders created in this operation."]
    pub files: Vec<FileCreate>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationClientCapabilities {
    #[doc = " Whether declaration supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `DeclarationRegistrationOptions` return value for the corresponding server "]
    #[doc = " capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports additional metadata in the form of declaration links."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkSupport")]
    pub link_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: DeclarationParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: DeclarationParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationRegistrationOptions {
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionClientCapabilities {
    #[doc = " Whether definition supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports additional metadata in the form of definition links."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkSupport")]
    pub link_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: DefinitionParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: DefinitionParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " Delete file operation"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteFile {
    #[doc = " An optional annotation identifer describing the operation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annotationId")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
    #[doc = " A delete"]
    pub kind: String,
    #[doc = " Delete options."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DeleteFileOptions>,
    #[doc = " The file to delete."]
    pub uri: String,
}
#[doc = " Delete file options"]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteFileOptions {
    #[doc = " Ignore the operation if the file doesn't exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreIfNotExists")]
    pub ignore_if_not_exists: Option<bool>,
    #[doc = " Delete the content recursively if a folder is denoted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}
#[doc = " The parameters sent in notifications/requests for user-initiated deletes of files."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteFilesParams {
    #[doc = " An array of all files/folders deleted in this operation."]
    pub files: Vec<FileDelete>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRange {
    #[doc = " The range's end position."]
    pub end: DiagnosticRangeEnd,
    #[doc = " The range's start position."]
    pub start: DiagnosticRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    #[doc = " The diagnostic's code, which might appear in the user interface."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<serde_json::Value>,
    #[doc = " An optional property to describe the error code."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeDescription")]
    pub code_description: Option<CodeDescription>,
    #[doc = " A data entry field that is preserved between a `textDocument/publishDiagnostics` "]
    #[doc = " notification and `textDocument/codeAction` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " The diagnostic's message."]
    pub message: String,
    #[doc = " The range at which the message applies."]
    pub range: DiagnosticRange,
    #[doc = " An array of related diagnostic information, e.g. when symbol-names within a scope collide "]
    #[doc = " all definitions can be marked via this property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "relatedInformation")]
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
    #[doc = " The diagnostic's severity. Can be omitted. If omitted it is up to the client to interpret "]
    #[doc = " diagnostics as error, warning, info or hint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosticSeverity>,
    #[doc = " A human-readable string describing the source of this diagnostic, e.g. 'typescript' or "]
    #[doc = " 'super lint'."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = " Additional metadata about the diagnostic."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DiagnosticTag>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRelatedInformationLocationRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRelatedInformationLocationRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRelatedInformationLocationRange {
    #[doc = " The range's end position."]
    pub end: DiagnosticRelatedInformationLocationRangeEnd,
    #[doc = " The range's start position."]
    pub start: DiagnosticRelatedInformationLocationRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRelatedInformationLocation {
    pub range: DiagnosticRelatedInformationLocationRange,
    pub uri: String,
}
#[doc = " Represents a related message and source code location for a diagnostic. This should be used to "]
#[doc = " point to code locations that cause or are related to a diagnostics, e.g when duplicating a "]
#[doc = " symbol in a scope."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRelatedInformation {
    #[doc = " The location of this related diagnostic information."]
    pub location: DiagnosticRelatedInformationLocation,
    #[doc = " The message of this related diagnostic information."]
    pub message: String,
}
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}
#[doc = " The diagnostic tags."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum DiagnosticTag {
    Unnecessary = 1,
    Deprecated = 2,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DidChangeConfigurationClientCapabilities {
    #[doc = " Did change configuration notification supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DidChangeWatchedFilesClientCapabilities {
    #[doc = " Did change watched files notification supports dynamic registration. Please note that the "]
    #[doc = " current protocol doesn't support static configuration for file changes from the server "]
    #[doc = " side."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[doc = " Describe options to be used when registering for file system change events."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DidChangeWatchedFilesRegistrationOptions {
    #[doc = " The watchers to register."]
    pub watchers: Vec<FileSystemWatcher>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DidChangeWorkspaceFoldersParams {
    #[doc = " The actual workspace folder change event."]
    pub event: WorkspaceFoldersChangeEvent,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentColorClientCapabilities {
    #[doc = " Whether document color supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentColorOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentColorRegistrationOptions {
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentFilter {
    #[doc = " A language id, like `typescript`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = " A glob pattern, like `*.{ts,js}`."]
    #[doc = " "]
    #[doc = " Glob patterns can have the following syntax:"]
    #[doc = " - `*` to match one or more characters in a path segment"]
    #[doc = " - `?` to match on one character in a path segment"]
    #[doc = " - `**` to match any number of path segments, including none"]
    #[doc = " - `{}` to group sub patterns into an OR expression. (e.g. `**\u{200b}/*.{ts,js}`   matches all "]
    #[doc = " TypeScript and JavaScript files)"]
    #[doc = " - `[]` to declare a range of characters to match in a path segment   (e.g., `example.[0-9]` "]
    #[doc = " to match on `example.0`, `example.1`, …)"]
    #[doc = " - `[!...]` to negate a range of characters to match in a path segment   (e.g., "]
    #[doc = " `example.[!0-9]` to match on `example.a`, `example.b`, but   not `example.0`)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = " A Uri [scheme](#Uri.scheme), like `file` or `untitled`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentFormattingClientCapabilities {
    #[doc = " Whether formatting supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentFormattingRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightRange {
    #[doc = " The range's end position."]
    pub end: DocumentHighlightRangeEnd,
    #[doc = " The range's start position."]
    pub start: DocumentHighlightRangeStart,
}
#[doc = " A document highlight is a range inside a text document which deserves special attention. "]
#[doc = " Usually a document highlight is visualized by changing the background color of its range."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlight {
    #[doc = " The highlight kind, default is DocumentHighlightKind.Text."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<DocumentHighlightKind>,
    #[doc = " The range this highlight applies to."]
    pub range: DocumentHighlightRange,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightClientCapabilities {
    #[doc = " Whether document highlight supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[doc = " A document highlight kind."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum DocumentHighlightKind {
    Text = 1,
    Read = 2,
    Write = 3,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: DocumentHighlightParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: DocumentHighlightParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHighlightRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLinkClientCapabilities {
    #[doc = " Whether document link supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " Whether the client supports the `tooltip` property on `DocumentLink`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tooltipSupport")]
    pub tooltip_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLinkOptions {
    #[doc = " Document links have a resolve provider as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLinkRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " Document links have a resolve provider as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentOnTypeFormattingClientCapabilities {
    #[doc = " Whether on type formatting supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentOnTypeFormattingOptions {
    #[doc = " A character on which formatting should be triggered, like `}`."]
    #[serde(rename = "firstTriggerCharacter")]
    pub first_trigger_character: String,
    #[doc = " More trigger characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moreTriggerCharacter")]
    pub more_trigger_character: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentOnTypeFormattingRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " A character on which formatting should be triggered, like `}`."]
    #[serde(rename = "firstTriggerCharacter")]
    pub first_trigger_character: String,
    #[doc = " More trigger characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moreTriggerCharacter")]
    pub more_trigger_character: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentRangeFormattingClientCapabilities {
    #[doc = " Whether formatting supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentRangeFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentRangeFormattingRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
pub type DocumentSelector = Vec<DocumentFilter>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolRange {
    #[doc = " The range's end position."]
    pub end: DocumentSymbolRangeEnd,
    #[doc = " The range's start position."]
    pub start: DocumentSymbolRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolSelectionRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolSelectionRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolSelectionRange {
    #[doc = " The range's end position."]
    pub end: DocumentSymbolSelectionRangeEnd,
    #[doc = " The range's start position."]
    pub start: DocumentSymbolSelectionRangeStart,
}
#[doc = " Represents programming constructs like variables, classes, interfaces etc. that appear in a "]
#[doc = " document. Document symbols can be hierarchical and they have two ranges: one that encloses its "]
#[doc = " definition and one that points to its most interesting range, e.g. the range of an identifier."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbol {
    #[doc = " Children of this symbol, e.g. properties of a class."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DocumentSymbol>>,
    #[doc = " Indicates if this symbol is deprecated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = " More detail for this symbol, e.g the signature of a function."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = " The kind of this symbol."]
    pub kind: SymbolKind,
    #[doc = " The name of this symbol. Will be displayed in the user interface and therefore must not be "]
    #[doc = " an empty string or a string only consisting of white spaces."]
    pub name: String,
    #[doc = " The range enclosing this symbol not including leading/trailing whitespace but everything "]
    #[doc = " else like comments. This information is typically used to determine if the clients cursor "]
    #[doc = " is inside the symbol to reveal in the symbol in the UI."]
    pub range: DocumentSymbolRange,
    #[doc = " The range that should be selected and revealed when this symbol is being picked, e.g. the "]
    #[doc = " name of a function. Must be contained by the `range`."]
    #[serde(rename = "selectionRange")]
    pub selection_range: DocumentSymbolSelectionRange,
    #[doc = " Tags for this document symbol."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SymbolTag>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolClientCapabilitiesSymbolKind {
    #[doc = " The symbol kind values the client supports. When this property exists the client also "]
    #[doc = " guarantees that it will handle values outside its set gracefully and falls back to a "]
    #[doc = " default value when unknown."]
    #[doc = " "]
    #[doc = " If this property is not present the client only supports the symbol kinds from `File` to "]
    #[doc = " `Array` as defined in the initial version of the protocol."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueSet")]
    pub value_set: Option<Vec<SymbolKind>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolClientCapabilitiesTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<SymbolTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolClientCapabilities {
    #[doc = " Whether document symbol supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports hierarchical document symbols."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hierarchicalDocumentSymbolSupport")]
    pub hierarchical_document_symbol_support: Option<bool>,
    #[doc = " The client supports an additional label presented in the UI when registering a document "]
    #[doc = " symbol provider."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "labelSupport")]
    pub label_support: Option<bool>,
    #[doc = " Specific capabilities for the `SymbolKind` in the `textDocument/documentSymbol` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "symbolKind")]
    pub symbol_kind: Option<DocumentSymbolClientCapabilitiesSymbolKind>,
    #[doc = " The client supports tags on `SymbolInformation`. Tags are supported on `DocumentSymbol` if "]
    #[doc = " `hierarchicalDocumentSymbolSupport` is set to true. Clients supporting tags have to handle "]
    #[doc = " unknown tags gracefully."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagSupport")]
    pub tag_support: Option<DocumentSymbolClientCapabilitiesTagSupport>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolOptions {
    #[doc = " A human-readable string that is shown when multiple outlines trees are shown for the same "]
    #[doc = " document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: DocumentSymbolParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentSymbolRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " A human-readable string that is shown when multiple outlines trees are shown for the same "]
    #[doc = " document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteCommandClientCapabilities {
    #[doc = " Execute command supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteCommandOptions {
    #[doc = " The commands to be executed on the server"]
    pub commands: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteCommandParams {
    #[doc = " Arguments that the command should be invoked with."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
    #[doc = " The identifier of the actual command handler."]
    pub command: String,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[doc = " Execute command registration options."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteCommandRegistrationOptions {
    #[doc = " The commands to be executed on the server"]
    pub commands: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum FailureHandlingKind {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "textOnlyTransactional")]
    TextOnlyTransactional,
}
#[doc = " Represents information on a file/folder create."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileCreate {
    #[doc = " A file:// URI for the location of the file/folder being created."]
    pub uri: String,
}
#[doc = " Represents information on a file/folder delete."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileDelete {
    #[doc = " A file:// URI for the location of the file/folder being deleted."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileOperationFilterPattern {
    #[doc = " The glob pattern to match. Glob patterns can have the following syntax:"]
    #[doc = " - `*` to match one or more characters in a path segment"]
    #[doc = " - `?` to match on one character in a path segment"]
    #[doc = " - `**` to match any number of path segments, including none"]
    #[doc = " - `{}` to group sub patterns into an OR expression. (e.g. `**\u{200b}/*.{ts,js}`   matches all "]
    #[doc = " TypeScript and JavaScript files)"]
    #[doc = " - `[]` to declare a range of characters to match in a path segment   (e.g., `example.[0-9]` "]
    #[doc = " to match on `example.0`, `example.1`, …)"]
    #[doc = " - `[!...]` to negate a range of characters to match in a path segment   (e.g., "]
    #[doc = " `example.[!0-9]` to match on `example.a`, `example.b`, but   not `example.0`)"]
    pub glob: String,
    #[doc = " Whether to match files or folders with this pattern."]
    #[doc = " "]
    #[doc = " Matches both if undefined."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<FileOperationPatternKind>,
    #[doc = " Additional options used during matching."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<FileOperationPatternOptions>,
}
#[doc = " A filter to describe in which file operation requests or notifications the server is interested "]
#[doc = " in."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileOperationFilter {
    #[doc = " The actual file operation pattern."]
    pub pattern: FileOperationFilterPattern,
    #[doc = " A Uri like `file` or `untitled`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
#[doc = " A pattern kind describing if a glob pattern matches a file a folder or both."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum FileOperationPatternKind {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "folder")]
    Folder,
}
#[doc = " Matching options for the file operation pattern."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileOperationPatternOptions {
    #[doc = " The pattern should be matched ignoring casing."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreCase")]
    pub ignore_case: Option<bool>,
}
#[doc = " Represents information on a file/folder rename."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileRename {
    #[doc = " A file:// URI for the new location of the file/folder being renamed."]
    #[serde(rename = "newUri")]
    pub new_uri: String,
    #[doc = " A file:// URI for the original location of the file/folder being renamed."]
    #[serde(rename = "oldUri")]
    pub old_uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileSystemWatcher {
    #[doc = " The glob pattern to watch."]
    #[doc = " "]
    #[doc = " Glob patterns can have the following syntax:"]
    #[doc = " - `*` to match one or more characters in a path segment"]
    #[doc = " - `?` to match on one character in a path segment"]
    #[doc = " - `**` to match any number of path segments, including none"]
    #[doc = " - `{}` to group sub patterns into an OR expression. (e.g. `**\u{200b}/*.{ts,js}`   matches all "]
    #[doc = " TypeScript and JavaScript files)"]
    #[doc = " - `[]` to declare a range of characters to match in a path segment   (e.g., `example.[0-9]` "]
    #[doc = " to match on `example.0`, `example.1`, …)"]
    #[doc = " - `[!...]` to negate a range of characters to match in a path segment   (e.g., "]
    #[doc = " `example.[!0-9]` to match on `example.a`, `example.b`, but not   `example.0`)"]
    #[serde(rename = "globPattern")]
    pub glob_pattern: String,
    #[doc = " The kind of events of interest. If omitted it defaults to WatchKind.Create | "]
    #[doc = " WatchKind.Change | WatchKind.Delete which is 7."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Uinteger>,
}
#[doc = " Represents a folding range. To be valid, start and end line must be bigger than zero and "]
#[doc = " smaller than the number of lines in the document. Clients are free to ignore invalid ranges."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldingRange {
    #[doc = " The zero-based character offset before the folded range ends. If not defined, defaults to "]
    #[doc = " the length of the end line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endCharacter")]
    pub end_character: Option<Uinteger>,
    #[doc = " The zero-based end line of the range to fold. The folded area ends with the line's last "]
    #[doc = " character. To be valid, the end must be zero or larger and smaller than the number of lines "]
    #[doc = " in the document."]
    #[serde(rename = "endLine")]
    pub end_line: Uinteger,
    #[doc = " Describes the kind of the folding range such as `comment` or `region`. The kind is used to "]
    #[doc = " categorize folding ranges and used by commands like 'Fold all comments'. See "]
    #[doc = " [FoldingRangeKind](#FoldingRangeKind) for an enumeration of standardized kinds."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = " The zero-based character offset from where the folded range starts. If not defined, "]
    #[doc = " defaults to the length of the start line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startCharacter")]
    pub start_character: Option<Uinteger>,
    #[doc = " The zero-based start line of the range to fold. The folded area starts after the line's "]
    #[doc = " last character. To be valid, the end must be zero or larger and smaller than the number of "]
    #[doc = " lines in the document."]
    #[serde(rename = "startLine")]
    pub start_line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldingRangeClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration for folding range providers. If this "]
    #[doc = " is set to `true` the client supports the new `FoldingRangeRegistrationOptions` return value "]
    #[doc = " for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " If set, the client signals that it only supports folding complete lines. If set, client "]
    #[doc = " will ignore specified `startCharacter` and `endCharacter` properties in a FoldingRange."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lineFoldingOnly")]
    pub line_folding_only: Option<bool>,
    #[doc = " The maximum number of folding ranges that the client prefers to receive per document. The "]
    #[doc = " value serves as a hint, servers are free to follow the limit."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rangeLimit")]
    pub range_limit: Option<Uinteger>,
}
#[doc = " Enum of known range kinds"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum FoldingRangeKind {
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "imports")]
    Imports,
    #[serde(rename = "region")]
    Region,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldingRangeParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldingRangeParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: FoldingRangeParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldingRangeRegistrationOptions {
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
#[serde(deny_unknown_fields)]
pub struct HoverRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverRange {
    #[doc = " The range's end position."]
    pub end: HoverRangeEnd,
    #[doc = " The range's start position."]
    pub start: HoverRangeStart,
}
#[doc = " The result of a hover request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hover {
    #[doc = " The hover's content"]
    #[serde(default)]
    #[serde(with = "::schemafy_core::one_or_many")]
    pub contents: Vec<serde_json::Value>,
    #[doc = " An optional range is a range inside a text document that is used to visualize a hover, e.g. "]
    #[doc = " by changing the background color."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<HoverRange>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverClientCapabilities {
    #[doc = " Client supports the following content formats if the content property refers to a `literal "]
    #[doc = " of type MarkupContent`. The order describes the preferred format of the client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentFormat")]
    pub content_format: Option<Vec<MarkupKind>>,
    #[doc = " Whether hover supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverParams {
    #[doc = " The position inside the text document."]
    pub position: HoverParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: HoverParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HoverRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImplementationClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `ImplementationRegistrationOptions` return value for the corresponding "]
    #[doc = " server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports additional metadata in the form of definition links."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkSupport")]
    pub link_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImplementationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImplementationParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImplementationParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImplementationParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: ImplementationParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: ImplementationParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImplementationRegistrationOptions {
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
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEditInsertEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEditInsertStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEditInsert {
    #[doc = " The range's end position."]
    pub end: InsertReplaceEditInsertEnd,
    #[doc = " The range's start position."]
    pub start: InsertReplaceEditInsertStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEditReplaceEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEditReplaceStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEditReplace {
    #[doc = " The range's end position."]
    pub end: InsertReplaceEditReplaceEnd,
    #[doc = " The range's start position."]
    pub start: InsertReplaceEditReplaceStart,
}
#[doc = " A special text edit to provide an insert and a replace operation."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReplaceEdit {
    #[doc = " The range if the insert is requested"]
    pub insert: InsertReplaceEditInsert,
    #[doc = " The string to be inserted."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range if the replace is requested."]
    pub replace: InsertReplaceEditReplace,
}
#[doc = " Defines whether the insert text in a completion item should be interpreted as plain text or a "]
#[doc = " snippet."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum InsertTextFormat {
    PlainText = 1,
    Snippet = 2,
}
#[doc = " How whitespace and indentation is handled during completion item insertion."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum InsertTextMode {
    AsIs = 1,
    AdjustIndentation = 2,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangeClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangeParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangeParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangeParams {
    #[doc = " The position inside the text document."]
    pub position: LinkedEditingRangeParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: LinkedEditingRangeParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangeRegistrationOptions {
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
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangesItemRangesEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangesItemRangesStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRangesItemRanges {
    #[doc = " The range's end position."]
    pub end: LinkedEditingRangesItemRangesEnd,
    #[doc = " The range's start position."]
    pub start: LinkedEditingRangesItemRangesStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedEditingRanges {
    #[doc = " A list of ranges that can be renamed together. The ranges must have identical length and "]
    #[doc = " contain identical text content. The ranges cannot overlap."]
    pub ranges: Vec<LinkedEditingRangesItemRanges>,
    #[doc = " An optional word pattern (regular expression) that describes valid contents for the given "]
    #[doc = " ranges. If no pattern is provided, the client configuration's word pattern will be used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wordPattern")]
    pub word_pattern: Option<String>,
}
#[doc = " Client capabilities specific to the used markdown parser."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MarkdownClientCapabilities {
    #[doc = " The name of the parser."]
    pub parser: String,
    #[doc = " The version of the parser."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[doc = " A `MarkupContent` literal represents a string value which content is interpreted base on its "]
#[doc = " kind flag. Currently the protocol supports `plaintext` and `markdown` as markup kinds."]
#[doc = " "]
#[doc = " If the kind is `markdown` then the value can contain fenced code blocks like in GitHub issues."]
#[doc = " "]
#[doc = " Here is an example how such a string can be constructed using JavaScript / TypeScript: "]
#[doc = " ```typescript let markdown: MarkdownContent = {  kind: MarkupKind.Markdown,  value: [   '# "]
#[doc = " Header',   'Some text',   '```typescript',   'someCode();',   '```'  ].join('\\n') }; ```"]
#[doc = " "]
#[doc = " *Please Note* that clients might sanitize the return markdown. A client could decide to remove "]
#[doc = " HTML from the markdown to avoid script execution."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MarkupContent {
    #[doc = " The type of the Markup"]
    pub kind: MarkupKind,
    #[doc = " The content itself"]
    pub value: String,
}
#[doc = " Describes the content type that a client supports in various result literals like `Hover`, "]
#[doc = " `ParameterInfo` or `CompletionItem`."]
#[doc = " "]
#[doc = " Please note that `MarkupKinds` must not start with a `$`. This kinds are reserved for internal "]
#[doc = " usage."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum MarkupKind {
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "markdown")]
    Markdown,
}
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum MessageType {
    Error = 1,
    Warning = 2,
    Info = 3,
    Log = 4,
}
#[doc = " Moniker definition to match LSIF 0.5 moniker definition."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Moniker {
    #[doc = " The identifier of the moniker. The value is opaque in LSIF however schema owners are "]
    #[doc = " allowed to define the structure if they want."]
    pub identifier: String,
    #[doc = " The moniker kind if known."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<MonikerKind>,
    #[doc = " The scheme of the moniker. For example tsc or .Net"]
    pub scheme: String,
    #[doc = " The scope in which the moniker is unique"]
    pub unique: UniquenessLevel,
}
#[doc = " The moniker kind."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum MonikerKind {
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "export")]
    Export,
    #[serde(rename = "local")]
    Local,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MonikerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MonikerParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MonikerParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MonikerParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: MonikerParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: MonikerParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MonikerRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " Represents a parameter of a callable-signature. A parameter can have a label and a doc-comment."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterInformation {
    #[doc = " The human-readable doc-comment of this parameter. Will be shown in the UI but can be "]
    #[doc = " omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<serde_json::Value>,
    #[doc = " The label of this parameter information."]
    #[doc = " "]
    #[doc = " Either a string or an inclusive start and exclusive end offsets within its containing "]
    #[doc = " signature label. (see SignatureInformation.label). The offsets are based on a UTF-16 string "]
    #[doc = " representation as `Position` and `Range` does."]
    #[doc = " "]
    #[doc = " *Note*: a label of type string should be a substring of its containing signature label. Its "]
    #[doc = " intended use case is to highlight the parameter label part in the "]
    #[doc = " `SignatureInformation.label`."]
    pub label: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartialResultParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareRenameParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareRenameParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareRenameParams {
    #[doc = " The position inside the text document."]
    pub position: PrepareRenameParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: PrepareRenameParamsTextDocument,
}
pub type PrepareSupportDefaultBehavior = f64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishDiagnosticsClientCapabilitiesTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<DiagnosticTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishDiagnosticsClientCapabilities {
    #[doc = " Client supports a codeDescription property"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeDescriptionSupport")]
    pub code_description_support: Option<bool>,
    #[doc = " Whether code action supports the `data` property which is preserved between a "]
    #[doc = " `textDocument/publishDiagnostics` and `textDocument/codeAction` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataSupport")]
    pub data_support: Option<bool>,
    #[doc = " Whether the clients accepts diagnostics with related information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "relatedInformation")]
    pub related_information: Option<bool>,
    #[doc = " Client supports the tag property to provide meta data about a diagnostic. Clients "]
    #[doc = " supporting tags have to handle unknown tags gracefully."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagSupport")]
    pub tag_support: Option<PublishDiagnosticsClientCapabilitiesTagSupport>,
    #[doc = " Whether the client interprets the version property of the `textDocument/publishDiagnostics` "]
    #[doc = " notification's parameter."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "versionSupport")]
    pub version_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceClientCapabilities {
    #[doc = " Whether references supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceContext {
    #[doc = " Include the declaration of the current symbol."]
    #[serde(rename = "includeDeclaration")]
    pub include_declaration: bool,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceParams {
    pub context: ReferenceContext,
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: ReferenceParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: ReferenceParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " General parameters to register for a capability."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Registration {
    #[doc = " The id used to register the request. The id can be used to deregister the request again."]
    pub id: String,
    #[doc = " The method / capability to register for."]
    pub method: String,
    #[doc = " Options necessary for the registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "registerOptions")]
    pub register_options: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}
#[doc = " Client capabilities specific to regular expressions."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegularExpressionsClientCapabilities {
    #[doc = " The engine's name."]
    pub engine: String,
    #[doc = " The engine's version."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenameClientCapabilities {
    #[doc = " Whether rename supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " Whether the client honors the change annotations in text edits and resource operations "]
    #[doc = " returned via the rename request's workspace edit by for example presenting the workspace "]
    #[doc = " edit in the user interface and asking for confirmation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "honorsChangeAnnotations")]
    pub honors_change_annotations: Option<bool>,
    #[doc = " Client supports testing for validity of rename operations before execution."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prepareSupport")]
    pub prepare_support: Option<bool>,
    #[doc = " Client supports the default behavior result (`{ defaultBehavior: boolean }`)."]
    #[doc = " "]
    #[doc = " The value indicates the default behavior used by the client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prepareSupportDefaultBehavior")]
    pub prepare_support_default_behavior: Option<PrepareSupportDefaultBehavior>,
}
#[doc = " Rename file operation"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenameFile {
    #[doc = " An optional annotation identifer describing the operation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annotationId")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
    #[doc = " A rename"]
    pub kind: String,
    #[doc = " The new location."]
    #[serde(rename = "newUri")]
    pub new_uri: String,
    #[doc = " The old (existing) location."]
    #[serde(rename = "oldUri")]
    pub old_uri: String,
    #[doc = " Rename options."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<RenameFileOptions>,
}
#[doc = " Rename file options"]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenameFileOptions {
    #[doc = " Ignores if target exists."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreIfExists")]
    pub ignore_if_exists: Option<bool>,
    #[doc = " Overwrite target if existing. Overwrite wins over `ignoreIfExists`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}
#[doc = " The parameters sent in notifications/requests for user-initiated renames of files."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenameFilesParams {
    #[doc = " An array of all files/folders renamed in this operation. When a folder is renamed, only the "]
    #[doc = " folder will be included, and not its children."]
    pub files: Vec<FileRename>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenameOptions {
    #[doc = " Renames should be checked and tested before being executed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prepareProvider")]
    pub prepare_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenameRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " Renames should be checked and tested before being executed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prepareProvider")]
    pub prepare_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " The kind of resource operations supported by the client."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ResourceOperationKind {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "rename")]
    Rename,
    #[serde(rename = "delete")]
    Delete,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SaveOptions {
    #[doc = " The client is supposed to include the content on save."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeText")]
    pub include_text: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeRange {
    #[doc = " The range's end position."]
    pub end: SelectionRangeRangeEnd,
    #[doc = " The range's start position."]
    pub start: SelectionRangeRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRange {
    #[doc = " The parent selection range containing this range. Therefore `parent.range` must contain "]
    #[doc = " `this.range`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<SelectionRange>>,
    #[doc = " The [range](#Range) of this selection range."]
    pub range: SelectionRangeRange,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration for selection range providers. If this "]
    #[doc = " is set to `true` the client supports the new `SelectionRangeRegistrationOptions` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeParamsItemPositions {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The positions inside the text document."]
    pub positions: Vec<SelectionRangeParamsItemPositions>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: SelectionRangeParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRangeRegistrationOptions {
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
pub enum SemanticTokenModifiers {
    #[serde(rename = "declaration")]
    Declaration,
    #[serde(rename = "definition")]
    Definition,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "abstract")]
    Abstract,
    #[serde(rename = "async")]
    Async,
    #[serde(rename = "modification")]
    Modification,
    #[serde(rename = "documentation")]
    Documentation,
    #[serde(rename = "defaultLibrary")]
    DefaultLibrary,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum SemanticTokenTypes {
    #[serde(rename = "namespace")]
    Namespace,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "typeParameter")]
    TypeParameter,
    #[serde(rename = "parameter")]
    Parameter,
    #[serde(rename = "variable")]
    Variable,
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "enumMember")]
    EnumMember,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "macro")]
    Macro,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "modifier")]
    Modifier,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "operator")]
    Operator,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokens {
    #[doc = " The actual tokens."]
    pub data: Vec<Uinteger>,
    #[doc = " An optional result id. If provided and clients support delta updating the client will "]
    #[doc = " include the result id in the next semantic token request. A server can then instead of "]
    #[doc = " computing all semantic tokens again simply send a delta."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultId")]
    pub result_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensDelta {
    #[doc = " The semantic token edits to transform a previous result into a new result."]
    pub edits: Vec<SemanticTokensEdit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultId")]
    pub result_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensDeltaParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensDeltaParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The result id of a previous response. The result Id can either point to a full response or "]
    #[doc = " a delta response depending on what was received last."]
    #[serde(rename = "previousResultId")]
    pub previous_result_id: String,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: SemanticTokensDeltaParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensDeltaPartialResult {
    pub edits: Vec<SemanticTokensEdit>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensEdit {
    #[doc = " The elements to insert."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Uinteger>>,
    #[doc = " The count of elements to remove."]
    #[serde(rename = "deleteCount")]
    pub delete_count: Uinteger,
    #[doc = " The start offset of the edit."]
    pub start: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensLegend {
    #[doc = " The token modifiers a server uses."]
    #[serde(rename = "tokenModifiers")]
    pub token_modifiers: Vec<String>,
    #[doc = " The token types a server uses."]
    #[serde(rename = "tokenTypes")]
    pub token_types: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensOptions {
    #[doc = " Server supports providing semantic tokens for a full document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<serde_json::Value>,
    #[doc = " The legend used by the server"]
    pub legend: SemanticTokensLegend,
    #[doc = " Server supports providing semantic tokens for a specific range of a document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: SemanticTokensParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensPartialResult {
    pub data: Vec<Uinteger>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensRangeParamsRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensRangeParamsRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensRangeParamsRange {
    #[doc = " The range's end position."]
    pub end: SemanticTokensRangeParamsRangeEnd,
    #[doc = " The range's start position."]
    pub start: SemanticTokensRangeParamsRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensRangeParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensRangeParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The range the semantic tokens are requested for."]
    pub range: SemanticTokensRangeParamsRange,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: SemanticTokensRangeParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " Server supports providing semantic tokens for a full document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<serde_json::Value>,
    #[doc = " The id used to register the request. The id can be used to deregister the request again. "]
    #[doc = " See also Registration#id."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = " The legend used by the server"]
    pub legend: SemanticTokensLegend,
    #[doc = " Server supports providing semantic tokens for a specific range of a document."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTokensWorkspaceClientCapabilities {
    #[doc = " Whether the client implementation supports a refresh request sent from the server to the "]
    #[doc = " client."]
    #[doc = " "]
    #[doc = " Note that this event is global and will force the client to refresh all semantic tokens "]
    #[doc = " currently shown. It should be used with absolute care and is useful for situation where a "]
    #[doc = " server for example detect a project wide change that requires such a calculation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refreshSupport")]
    pub refresh_support: Option<bool>,
}
#[doc = " Client capabilities for the show document request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowDocumentClientCapabilities {
    #[doc = " The client has support for the show document request."]
    pub support: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowDocumentParamsSelectionEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowDocumentParamsSelectionStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowDocumentParamsSelection {
    #[doc = " The range's end position."]
    pub end: ShowDocumentParamsSelectionEnd,
    #[doc = " The range's start position."]
    pub start: ShowDocumentParamsSelectionStart,
}
#[doc = " Params to show a document."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowDocumentParams {
    #[doc = " Indicates to show the resource in an external program. To show for example "]
    #[doc = " `https://code.visualstudio.com/` in the default WEB browser set `external` to `true`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[doc = " An optional selection range if the document is a text document. Clients might ignore the "]
    #[doc = " property if an external program is started or the file is not a text file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<ShowDocumentParamsSelection>,
    #[doc = " An optional property to indicate whether the editor showing the document should take focus "]
    #[doc = " or not. Clients might ignore this property if an external program is started."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "takeFocus")]
    pub take_focus: Option<bool>,
    #[doc = " The document uri to show."]
    pub uri: String,
}
#[doc = " The result of an show document request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowDocumentResult {
    #[doc = " A boolean indicating if the show was successful."]
    pub success: bool,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowMessageRequestClientCapabilitiesMessageActionItem {
    #[doc = " Whether the client supports additional attributes which are preserved and sent back to the "]
    #[doc = " server in the request's response."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropertiesSupport")]
    pub additional_properties_support: Option<bool>,
}
#[doc = " Show message request client capabilities"]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowMessageRequestClientCapabilities {
    #[doc = " Capabilities specific to the `MessageActionItem` type."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageActionItem")]
    pub message_action_item: Option<ShowMessageRequestClientCapabilitiesMessageActionItem>,
}
#[doc = " Signature help represents the signature of something callable. There can be multiple signature "]
#[doc = " but only one active and only one active parameter."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelp {
    #[doc = " The active parameter of the active signature. If omitted or the value lies outside the "]
    #[doc = " range of `signatures[activeSignature].parameters` defaults to 0 if the active signature has "]
    #[doc = " parameters. If the active signature has no parameters it is ignored. In future version of "]
    #[doc = " the protocol this property might become mandatory to better express the active parameter if "]
    #[doc = " the active signature does have any."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeParameter")]
    pub active_parameter: Option<Uinteger>,
    #[doc = " The active signature. If omitted or the value lies outside the range of `signatures` the "]
    #[doc = " value defaults to zero or is ignored if the `SignatureHelp` has no signatures."]
    #[doc = " "]
    #[doc = " Whenever possible implementors should make an active decision about the active signature "]
    #[doc = " and shouldn't rely on a default value."]
    #[doc = " "]
    #[doc = " In future version of the protocol this property might become mandatory to better express "]
    #[doc = " this."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeSignature")]
    pub active_signature: Option<Uinteger>,
    #[doc = " One or more signatures. If no signatures are available the signature help request should "]
    #[doc = " return `null`."]
    pub signatures: Vec<SignatureInformation>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpClientCapabilitiesSignatureInformationParameterInformation {
    #[doc = " The client supports processing label offsets instead of a simple label string."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "labelOffsetSupport")]
    pub label_offset_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpClientCapabilitiesSignatureInformation {
    #[doc = " The client supports the `activeParameter` property on `SignatureInformation` literal."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeParameterSupport")]
    pub active_parameter_support: Option<bool>,
    #[doc = " Client supports the following content formats for the documentation property. The order "]
    #[doc = " describes the preferred format of the client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentationFormat")]
    pub documentation_format: Option<Vec<MarkupKind>>,
    #[doc = " Client capabilities specific to parameter information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameterInformation")]
    pub parameter_information:
        Option<SignatureHelpClientCapabilitiesSignatureInformationParameterInformation>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpClientCapabilities {
    #[doc = " The client supports to send additional context information for a "]
    #[doc = " `textDocument/signatureHelp` request. A client that opts into contextSupport will also "]
    #[doc = " support the `retriggerCharacters` on `SignatureHelpOptions`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextSupport")]
    pub context_support: Option<bool>,
    #[doc = " Whether signature help supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports the following `SignatureInformation` specific properties."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "signatureInformation")]
    pub signature_information: Option<SignatureHelpClientCapabilitiesSignatureInformation>,
}
#[doc = " Additional information about the context in which a signature help request was triggered."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpContext {
    #[doc = " The currently active `SignatureHelp`."]
    #[doc = " "]
    #[doc = " The `activeSignatureHelp` has its `SignatureHelp.activeSignature` field updated based on "]
    #[doc = " the user navigating through available signatures."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeSignatureHelp")]
    pub active_signature_help: Option<SignatureHelp>,
    #[doc = " `true` if signature help was already showing when it was triggered."]
    #[doc = " "]
    #[doc = " Retriggers occur when the signature help is already active and can be caused by actions "]
    #[doc = " such as typing a trigger character, a cursor move, or document content changes."]
    #[serde(rename = "isRetrigger")]
    pub is_retrigger: bool,
    #[doc = " Character that caused signature help to be triggered."]
    #[doc = " "]
    #[doc = " This is undefined when triggerKind !== SignatureHelpTriggerKind.TriggerCharacter"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacter")]
    pub trigger_character: Option<String>,
    #[doc = " Action that caused signature help to be triggered."]
    #[serde(rename = "triggerKind")]
    pub trigger_kind: SignatureHelpTriggerKind,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpOptions {
    #[doc = " List of characters that re-trigger signature help."]
    #[doc = " "]
    #[doc = " These trigger characters are only active when signature help is already showing. All "]
    #[doc = " trigger characters are also counted as re-trigger characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retriggerCharacters")]
    pub retrigger_characters: Option<Vec<String>>,
    #[doc = " The characters that trigger signature help automatically."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpParams {
    #[doc = " The signature help context. This is only available if the client specifies to send this "]
    #[doc = " using the client capability `textDocument.signatureHelp.contextSupport === true`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<SignatureHelpContext>,
    #[doc = " The position inside the text document."]
    pub position: SignatureHelpParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: SignatureHelpParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureHelpRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " List of characters that re-trigger signature help."]
    #[doc = " "]
    #[doc = " These trigger characters are only active when signature help is already showing. All "]
    #[doc = " trigger characters are also counted as re-trigger characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retriggerCharacters")]
    pub retrigger_characters: Option<Vec<String>>,
    #[doc = " The characters that trigger signature help automatically."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " How a signature help was triggered."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum SignatureHelpTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    ContentChange = 3,
}
#[doc = " Represents the signature of something callable. A signature can have a label, like a "]
#[doc = " function-name, a doc-comment, and a set of parameters."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureInformation {
    #[doc = " The index of the active parameter."]
    #[doc = " "]
    #[doc = " If provided, this is used in place of `SignatureHelp.activeParameter`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeParameter")]
    pub active_parameter: Option<Uinteger>,
    #[doc = " The human-readable doc-comment of this signature. Will be shown in the UI but can be "]
    #[doc = " omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<serde_json::Value>,
    #[doc = " The label of this signature. Will be shown in the UI."]
    pub label: String,
    #[doc = " The parameters of this signature."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterInformation>>,
}
#[doc = " Static registration options to be returned in the initialize request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StaticRegistrationOptions {
    #[doc = " The id used to register the request. The id can be used to deregister the request again. "]
    #[doc = " See also Registration#id."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolInformationLocationRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolInformationLocationRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolInformationLocationRange {
    #[doc = " The range's end position."]
    pub end: SymbolInformationLocationRangeEnd,
    #[doc = " The range's start position."]
    pub start: SymbolInformationLocationRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolInformationLocation {
    pub range: SymbolInformationLocationRange,
    pub uri: String,
}
#[doc = " Represents information about programming constructs like variables, classes, interfaces etc."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolInformation {
    #[doc = " The name of the symbol containing this symbol. This information is for user interface "]
    #[doc = " purposes (e.g. to render a qualifier in the user interface if necessary). It can't be used "]
    #[doc = " to re-infer a hierarchy for the document symbols."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerName")]
    pub container_name: Option<String>,
    #[doc = " Indicates if this symbol is deprecated."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = " The kind of this symbol."]
    pub kind: SymbolKind,
    #[doc = " The location of this symbol. The location's range is used by a tool to reveal the location "]
    #[doc = " in the editor. If the symbol is selected in the tool the range's start information is used "]
    #[doc = " to position the cursor. So the range usually spans more then the actual symbol's name and "]
    #[doc = " does normally include things like visibility modifiers."]
    #[doc = " "]
    #[doc = " The range doesn't have to denote a node range in the sense of a abstract syntax tree. It "]
    #[doc = " can therefore not be used to re-construct a hierarchy of the symbols."]
    pub location: SymbolInformationLocation,
    #[doc = " The name of this symbol."]
    pub name: String,
    #[doc = " Tags for this symbol."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SymbolTag>>,
}
#[doc = " A symbol kind."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}
#[doc = " Symbol tags are extra annotations that tweak the rendering of a symbol."]
pub type SymbolTag = f64;
#[doc = " Describe options to be used when registering for text document change events."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentChangeRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " How documents are synced to the server. See TextDocumentSyncKind.Full and "]
    #[doc = " TextDocumentSyncKind.Incremental."]
    #[serde(rename = "syncKind")]
    pub sync_kind: TextDocumentSyncKind,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentClientCapabilitiesCallHierarchy {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentClientCapabilitiesMoniker {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentClientCapabilitiesSemanticTokensRequests {
    #[doc = " The client will send the `textDocument/semanticTokens/full` request if the server provides "]
    #[doc = " a corresponding handler."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<serde_json::Value>,
    #[doc = " The client will send the `textDocument/semanticTokens/range` request if the server provides "]
    #[doc = " a corresponding handler."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentClientCapabilitiesSemanticTokens {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The formats the clients supports."]
    pub formats: Vec<TokenFormat>,
    #[doc = " Whether the client supports tokens that can span multiple lines."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "multilineTokenSupport")]
    pub multiline_token_support: Option<bool>,
    #[doc = " Whether the client supports tokens that can overlap each other."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "overlappingTokenSupport")]
    pub overlapping_token_support: Option<bool>,
    #[doc = " Which requests the client supports and might send to the server depending on the server's "]
    #[doc = " capability. Please note that clients might not show semantic tokens or degrade some of the "]
    #[doc = " user experience if a range or full request is advertised by the client but not provided by "]
    #[doc = " the server. If for example the client capability `requests.full` and `request.range` are "]
    #[doc = " both set to true but the server only provides a range provider the client might not render "]
    #[doc = " a minimap correctly or might even decide to not show any semantic tokens at all."]
    pub requests: TextDocumentClientCapabilitiesSemanticTokensRequests,
    #[doc = " The token modifiers that the client supports."]
    #[serde(rename = "tokenModifiers")]
    pub token_modifiers: Vec<String>,
    #[doc = " The token types that the client supports."]
    #[serde(rename = "tokenTypes")]
    pub token_types: Vec<String>,
}
#[doc = " Text document specific client capabilities."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentClientCapabilities {
    #[doc = " Capabilities specific to the various call hierarchy requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callHierarchy")]
    pub call_hierarchy: Option<TextDocumentClientCapabilitiesCallHierarchy>,
    #[doc = " Capabilities specific to the `textDocument/codeAction` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeAction")]
    pub code_action: Option<CodeActionClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/codeLens` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeLens")]
    pub code_lens: Option<CodeLensClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/documentColor` and the "]
    #[doc = " `textDocument/colorPresentation` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "colorProvider")]
    pub color_provider: Option<DocumentColorClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/completion` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CompletionClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/declaration` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration: Option<DeclarationClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/definition` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/documentHighlight` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentHighlight")]
    pub document_highlight: Option<DocumentHighlightClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/documentLink` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentLink")]
    pub document_link: Option<DocumentLinkClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/documentSymbol` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentSymbol")]
    pub document_symbol: Option<DocumentSymbolClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/foldingRange` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "foldingRange")]
    pub folding_range: Option<FoldingRangeClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/formatting` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<DocumentFormattingClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/hover` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/implementation` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation: Option<ImplementationClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/linkedEditingRange` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkedEditingRange")]
    pub linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/moniker` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moniker: Option<TextDocumentClientCapabilitiesMoniker>,
    #[doc = " Capabilities specific to the `textDocument/onTypeFormatting` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onTypeFormatting")]
    pub on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/publishDiagnostics` notification."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publishDiagnostics")]
    pub publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/rangeFormatting` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rangeFormatting")]
    pub range_formatting: Option<DocumentRangeFormattingClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/references` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferenceClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/rename` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<RenameClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/selectionRange` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectionRange")]
    pub selection_range: Option<SelectionRangeClientCapabilities>,
    #[doc = " Capabilities specific to the various semantic token requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticTokens")]
    pub semantic_tokens: Option<TextDocumentClientCapabilitiesSemanticTokens>,
    #[doc = " Capabilities specific to the `textDocument/signatureHelp` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "signatureHelp")]
    pub signature_help: Option<SignatureHelpClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,
    #[doc = " Capabilities specific to the `textDocument/typeDefinition` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeDefinition")]
    pub type_definition: Option<TypeDefinitionClientCapabilities>,
}
#[doc = " An event describing a change to a text document. If range and rangeLength are omitted the new "]
#[doc = " text is considered to be the full content of the document."]
pub type TextDocumentContentChangeEvent = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentEditTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
    #[doc = " The version number of this document. If an optional versioned text document identifier is "]
    #[doc = " sent from the server to the client and the file is not open in the editor (the server has "]
    #[doc = " not received an open notification before) the server can send `null` to indicate that the "]
    #[doc = " version is known and the content on disk is the master (as specified with document content "]
    #[doc = " ownership)."]
    #[doc = " "]
    #[doc = " The version number of a document will increase after each change, including undo/redo. The "]
    #[doc = " number doesn't need to be consecutive."]
    pub version: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentEdit {
    #[doc = " The edits to be applied."]
    pub edits: Vec<serde_json::Value>,
    #[doc = " The text document to change."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentEditTextDocument,
}
#[doc = " General text document registration options."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
}
#[doc = " Represents reasons why a text document is saved."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum TextDocumentSaveReason {
    Manual = 1,
    AfterDelay = 2,
    FocusOut = 3,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentSaveRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[doc = " The client is supposed to include the content on save."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeText")]
    pub include_text: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentSyncClientCapabilities {
    #[doc = " The client supports did save notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didSave")]
    pub did_save: Option<bool>,
    #[doc = " Whether text document synchronization supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports sending will save notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willSave")]
    pub will_save: Option<bool>,
    #[doc = " The client supports sending a will save request and waits for a response providing text "]
    #[doc = " edits which will be applied to the document before it is saved."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willSaveWaitUntil")]
    pub will_save_wait_until: Option<bool>,
}
#[doc = " Defines how the host (editor) should sync document changes to the language server."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextDocumentSyncOptions {
    #[doc = " Change notifications are sent to the server. See TextDocumentSyncKind.None, "]
    #[doc = " TextDocumentSyncKind.Full and TextDocumentSyncKind.Incremental. If omitted it defaults to "]
    #[doc = " TextDocumentSyncKind.None."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<TextDocumentSyncKind>,
    #[doc = " Open and close notifications are sent to the server. If omitted open close notification "]
    #[doc = " should not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openClose")]
    pub open_close: Option<bool>,
    #[doc = " If present save notifications are sent to the server. If omitted the notification should "]
    #[doc = " not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<serde_json::Value>,
    #[doc = " If present will save notifications are sent to the server. If omitted the notification "]
    #[doc = " should not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willSave")]
    pub will_save: Option<bool>,
    #[doc = " If present will save wait until requests are sent to the server. If omitted the request "]
    #[doc = " should not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willSaveWaitUntil")]
    pub will_save_wait_until: Option<bool>,
}
pub type TokenFormat = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum TraceValue {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "messages")]
    Messages,
    #[serde(rename = "verbose")]
    Verbose,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinitionClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `TypeDefinitionRegistrationOptions` return value for the corresponding "]
    #[doc = " server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " The client supports additional metadata in the form of definition links."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkSupport")]
    pub link_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinitionParamsPosition {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinitionParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinitionParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<serde_json::Value>,
    #[doc = " The position inside the text document."]
    pub position: TypeDefinitionParamsPosition,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TypeDefinitionParamsTextDocument,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinitionRegistrationOptions {
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
#[doc = " Moniker uniqueness level to define scope of the moniker."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum UniquenessLevel {
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "global")]
    Global,
}
#[doc = " General parameters to unregister a capability."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Unregistration {
    #[doc = " The id used to unregister the request or notification. Usually an id provided during the "]
    #[doc = " register request."]
    pub id: String,
    #[doc = " The method / capability to unregister for."]
    pub method: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnregistrationParams {
    pub unregisterations: Vec<Unregistration>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WillSaveTextDocumentParamsTextDocument {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[doc = " The parameters send in a will save text document notification."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WillSaveTextDocumentParams {
    #[doc = " The 'TextDocumentSaveReason'."]
    pub reason: TextDocumentSaveReason,
    #[doc = " The document that will be saved."]
    #[serde(rename = "textDocument")]
    pub text_document: WillSaveTextDocumentParamsTextDocument,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressBegin {
    #[doc = " Controls if a cancel button should show to allow the user to cancel the long running "]
    #[doc = " operation. Clients that don't support cancellation are allowed to ignore the setting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    pub kind: String,
    #[doc = " Optional, more detailed associated progress message. Contains complementary information to "]
    #[doc = " the `title`."]
    #[doc = " "]
    #[doc = " Examples: \"3/25 files\", \"project/src/module2\", \"node_modules/some_dep\". If unset, the "]
    #[doc = " previous progress message (if any) is still valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Optional progress percentage to display (value 100 is considered 100%). If not provided "]
    #[doc = " infinite progress is assumed and clients are allowed to ignore the `percentage` value in "]
    #[doc = " subsequent in report notifications."]
    #[doc = " "]
    #[doc = " The value should be steadily rising. Clients are free to ignore values that are not "]
    #[doc = " following this rule. The value range is [0, 100]"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<Uinteger>,
    #[doc = " Mandatory title of the progress operation. Used to briefly inform about the kind of "]
    #[doc = " operation being performed."]
    #[doc = " "]
    #[doc = " Examples: \"Indexing\" or \"Linking dependencies\"."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressCancelParams {
    #[doc = " The token to be used to report progress."]
    pub token: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressCreateParams {
    #[doc = " The token to be used to report progress."]
    pub token: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressEnd {
    pub kind: String,
    #[doc = " Optional, a final message indicating to for example indicate the outcome of the operation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressParams {
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDoneProgressReport {
    #[doc = " Controls enablement state of a cancel button. This property is only valid if a cancel "]
    #[doc = " button got requested in the `WorkDoneProgressBegin` payload."]
    #[doc = " "]
    #[doc = " Clients that don't support cancellation or don't support control the button's enablement "]
    #[doc = " state are allowed to ignore the setting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    pub kind: String,
    #[doc = " Optional, more detailed associated progress message. Contains complementary information to "]
    #[doc = " the `title`."]
    #[doc = " "]
    #[doc = " Examples: \"3/25 files\", \"project/src/module2\", \"node_modules/some_dep\". If unset, the "]
    #[doc = " previous progress message (if any) is still valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = " Optional progress percentage to display (value 100 is considered 100%). If not provided "]
    #[doc = " infinite progress is assumed and clients are allowed to ignore the `percentage` value in "]
    #[doc = " subsequent in report notifications."]
    #[doc = " "]
    #[doc = " The value should be steadily rising. Clients are free to ignore values that are not "]
    #[doc = " following this rule. The value range is [0, 100]"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<Uinteger>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEditItemChangesRangeEnd {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEditItemChangesRangeStart {
    #[doc = " Character offset on a line in a document (zero-based). Assuming that the line is "]
    #[doc = " represented as a string, the `character` value represents the gap between the `character` "]
    #[doc = " and `character + 1`."]
    #[doc = " "]
    #[doc = " If the character value is greater than the line length it defaults back to the line length."]
    pub character: Uinteger,
    #[doc = " Line position in a document (zero-based)."]
    pub line: Uinteger,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEditItemChangesRange {
    #[doc = " The range's end position."]
    pub end: WorkspaceEditItemChangesRangeEnd,
    #[doc = " The range's start position."]
    pub start: WorkspaceEditItemChangesRangeStart,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEditItemChanges {
    #[doc = " The string to be inserted. For delete operations use an empty string."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range of the text document to be manipulated. To insert text into a document create a "]
    #[doc = " range where start === end."]
    pub range: WorkspaceEditItemChangesRange,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEdit {
    #[doc = " A map of change annotations that can be referenced in `AnnotatedTextEdit`s or create, "]
    #[doc = " rename and delete file / folder operations."]
    #[doc = " "]
    #[doc = " Whether clients honor this property depends on the client capability "]
    #[doc = " `workspace.changeAnnotationSupport`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changeAnnotations")]
    pub change_annotations: Option<::std::collections::BTreeMap<String, ChangeAnnotation>>,
    #[doc = " Holds changes to existing resources."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<::std::collections::BTreeMap<String, Vec<WorkspaceEditItemChanges>>>,
    #[doc = " Depending on the client capability `workspace.workspaceEdit.resourceOperations` document "]
    #[doc = " changes are either an array of `TextDocumentEdit`s to express changes to n different text "]
    #[doc = " documents where each text document edit addresses a specific version of a text document. Or "]
    #[doc = " it can contain above `TextDocumentEdit`s mixed with create, rename and delete file / folder "]
    #[doc = " operations."]
    #[doc = " "]
    #[doc = " Whether a client supports versioned document edits is expressed via "]
    #[doc = " `workspace.workspaceEdit.documentChanges` client capability."]
    #[doc = " "]
    #[doc = " If a client neither supports `documentChanges` nor "]
    #[doc = " `workspace.workspaceEdit.resourceOperations` then only plain `TextEdit`s using the "]
    #[doc = " `changes` property are supported."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentChanges")]
    pub document_changes: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEditClientCapabilitiesChangeAnnotationSupport {
    #[doc = " Whether the client groups edits with equal labels into tree nodes, for instance all edits "]
    #[doc = " labelled with \"Changes in Strings\" would be a tree node."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupsOnLabel")]
    pub groups_on_label: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEditClientCapabilities {
    #[doc = " Whether the client in general supports change annotations on text edits, create file, "]
    #[doc = " rename file and delete file changes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changeAnnotationSupport")]
    pub change_annotation_support: Option<WorkspaceEditClientCapabilitiesChangeAnnotationSupport>,
    #[doc = " The client supports versioned document changes in `WorkspaceEdit`s"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentChanges")]
    pub document_changes: Option<bool>,
    #[doc = " The failure handling strategy of a client if applying the workspace edit fails."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failureHandling")]
    pub failure_handling: Option<FailureHandlingKind>,
    #[doc = " Whether the client normalizes line endings to the client specific setting. If set to `true` "]
    #[doc = " the client will normalize line ending characters in a workspace edit to the client specific "]
    #[doc = " new line character(s)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "normalizesLineEndings")]
    pub normalizes_line_endings: Option<bool>,
    #[doc = " The resource operations the client supports. Clients should at least support 'create', "]
    #[doc = " 'rename' and 'delete' files and folders."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceOperations")]
    pub resource_operations: Option<Vec<ResourceOperationKind>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceFolder {
    #[doc = " The name of the workspace folder. Used to refer to this workspace folder in the user "]
    #[doc = " interface."]
    pub name: String,
    #[doc = " The associated URI for this workspace folder."]
    pub uri: String,
}
#[doc = " The workspace folder change event."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceFoldersChangeEvent {
    #[doc = " The array of added workspace folders"]
    pub added: Vec<WorkspaceFolder>,
    #[doc = " The array of the removed workspace folders"]
    pub removed: Vec<WorkspaceFolder>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceFoldersServerCapabilities {
    #[doc = " Whether the server wants to receive workspace folder change notifications."]
    #[doc = " "]
    #[doc = " If a string is provided, the string is treated as an ID under which the notification is "]
    #[doc = " registered on the client side. The ID can be used to unregister for these events using the "]
    #[doc = " `client/unregisterCapability` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changeNotifications")]
    pub change_notifications: Option<serde_json::Value>,
    #[doc = " The server has support for workspace folders"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceSymbolRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
pub type Array = Vec<serde_json::Value>;
#[doc = " Defines a decimal number. Since decimal numbers are very rare in the language server "]
#[doc = " specification we denote the exact range with every decimal using the mathematics interval "]
#[doc = " notation (e.g. [0, 1] denotes all decimals d with 0 <= d <= 1."]
pub type Decimal = f64;
#[doc = " Defines an integer number in the range of -2^31 to 2^31 - 1."]
pub type Integer = i32;
#[doc = " Defines an unsigned integer number in the range of 0 to 2^31 - 1."]
pub type Uinteger = u32;
pub type Lsp = serde_json::Value;
