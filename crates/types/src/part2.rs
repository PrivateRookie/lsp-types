use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use super::*;

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ConfigurationItem {
    #[doc = " The scope to get the configuration section for."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopeUri")]
    pub scope_uri: Option<DocumentUri>,
    #[doc = " The configuration section asked for."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConfigurationParams {
    pub items: Vec<ConfigurationItem>,
}
#[doc = " Create file operation"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
    pub uri: DocumentUri,
}
#[doc = " Options to create a file."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct CreateFilesParams {
    #[doc = " An array of all files/folders created in this operation."]
    pub files: Vec<FileCreate>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct DeclarationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DeclarationParams {
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
pub struct DefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DefinitionParams {
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
    pub uri: DocumentUri,
}
#[doc = " Delete file options"]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct DeleteFilesParams {
    #[doc = " An array of all files/folders deleted in this operation."]
    pub files: Vec<FileDelete>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
    pub range: Range,
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
#[doc = " Represents a related message and source code location for a diagnostic. This should be used to "]
#[doc = " point to code locations that cause or are related to a diagnostics, e.g when duplicating a "]
#[doc = " symbol in a scope."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DiagnosticRelatedInformation {
    #[doc = " The location of this related diagnostic information."]
    pub location: Location,
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
pub struct DidChangeConfigurationClientCapabilities {
    #[doc = " Did change configuration notification supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeConfigurationParams {
    #[doc = " The actual changed settings"]
    pub settings: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeTextDocumentParams {
    #[doc = " The actual content changes. The content changes describe single state changes to the "]
    #[doc = " document. So if there are two content changes c1 (at array index 0) and c2 (at array index "]
    #[doc = " 1) for a document in state S then c1 moves the document from S to S' and c2 from S' to S''. "]
    #[doc = " So c1 is computed on the state S and c2 is computed on the state S'."]
    #[doc = " "]
    #[doc = " To mirror the content of a document using change events use the following approach:"]
    #[doc = " - start with the same initial content"]
    #[doc = " - apply the 'textDocument/didChange' notifications in the order you   receive them."]
    #[doc = " - apply the `TextDocumentContentChangeEvent`s in a single notification   in the order you "]
    #[doc = " receive them."]
    #[serde(rename = "contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
    #[doc = " The document that did change. The version number points to the version after all provided "]
    #[doc = " content changes have been applied."]
    #[serde(rename = "textDocument")]
    pub text_document: VersionedTextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DidChangeWatchedFilesClientCapabilities {
    #[doc = " Did change watched files notification supports dynamic registration. Please note that the "]
    #[doc = " current protocol doesn't support static configuration for file changes from the server "]
    #[doc = " side."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeWatchedFilesParams {
    #[doc = " The actual file events."]
    pub changes: Vec<FileEvent>,
}
#[doc = " Describe options to be used when registering for file system change events."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeWatchedFilesRegistrationOptions {
    #[doc = " The watchers to register."]
    pub watchers: Vec<FileSystemWatcher>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeWorkspaceFoldersParams {
    #[doc = " The actual workspace folder change event."]
    pub event: WorkspaceFoldersChangeEvent,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidCloseTextDocumentParams {
    #[doc = " The document that was closed."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidOpenTextDocumentParams {
    #[doc = " The document that was opened."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentItem,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidSaveTextDocumentParams {
    #[doc = " Optional the content when saved. Depends on the includeText value when the save "]
    #[doc = " notification was requested."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = " The document that was saved."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DocumentColorClientCapabilities {
    #[doc = " Whether document color supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DocumentColorOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentColorParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct DocumentFormattingClientCapabilities {
    #[doc = " Whether formatting supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DocumentFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentFormattingParams {
    #[doc = " The format options."]
    pub options: FormattingOptions,
    #[doc = " The document to format."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentFormattingRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " A document highlight is a range inside a text document which deserves special attention. "]
#[doc = " Usually a document highlight is visualized by changing the background color of its range."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentHighlight {
    #[doc = " The highlight kind, default is DocumentHighlightKind.Text."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<DocumentHighlightKind>,
    #[doc = " The range this highlight applies to."]
    pub range: Range,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct DocumentHighlightOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentHighlightParams {
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
pub struct DocumentHighlightRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " A document link is a range in a text document that links to an internal or external resource, "]
#[doc = " like another text document or a web site."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentLink {
    #[doc = " A data entry field that is preserved on a document link between a DocumentLinkRequest and a "]
    #[doc = " DocumentLinkResolveRequest."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " The range this link applies to."]
    pub range: Range,
    #[doc = " The uri this link points to. If missing a resolve request is sent later."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DocumentUri>,
    #[doc = " The tooltip text when you hover over this link."]
    #[doc = " "]
    #[doc = " If a tooltip is provided, is will be displayed in a string that includes instructions on "]
    #[doc = " how to trigger the link, such as `{0} (ctrl + click)`. The specific instructions vary "]
    #[doc = " depending on OS, user settings, and localization."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct DocumentLinkParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The document to provide document links for."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct DocumentOnTypeFormattingClientCapabilities {
    #[doc = " Whether on type formatting supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct DocumentOnTypeFormattingParams {
    #[doc = " The character that has been typed."]
    pub ch: String,
    #[doc = " The format options."]
    pub options: FormattingOptions,
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct DocumentRangeFormattingClientCapabilities {
    #[doc = " Whether formatting supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DocumentRangeFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentRangeFormattingParams {
    #[doc = " The format options"]
    pub options: FormattingOptions,
    #[doc = " The range to format"]
    pub range: Range,
    #[doc = " The document to format."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
#[doc = " Represents programming constructs like variables, classes, interfaces etc. that appear in a "]
#[doc = " document. Document symbols can be hierarchical and they have two ranges: one that encloses its "]
#[doc = " definition and one that points to its most interesting range, e.g. the range of an identifier."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
    pub range: Range,
    #[doc = " The range that should be selected and revealed when this symbol is being picked, e.g. the "]
    #[doc = " name of a function. Must be contained by the `range`."]
    #[serde(rename = "selectionRange")]
    pub selection_range: Range,
    #[doc = " Tags for this document symbol."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SymbolTag>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct DocumentSymbolClientCapabilitiesTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<SymbolTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct DocumentSymbolParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub type DocumentUri = String;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ExecuteCommandClientCapabilities {
    #[doc = " Execute command supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExecuteCommandOptions {
    #[doc = " The commands to be executed on the server"]
    pub commands: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExecuteCommandParams {
    #[doc = " Arguments that the command should be invoked with."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
    #[doc = " The identifier of the actual command handler."]
    pub command: String,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[doc = " Execute command registration options."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
#[doc = " The file event type."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum FileChangeType {
    Created = 1,
    Changed = 2,
    Deleted = 3,
}
#[doc = " Represents information on a file/folder create."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileCreate {
    #[doc = " A file:// URI for the location of the file/folder being created."]
    pub uri: String,
}
#[doc = " Represents information on a file/folder delete."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileDelete {
    #[doc = " A file:// URI for the location of the file/folder being deleted."]
    pub uri: String,
}
#[doc = " An event describing a file change."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileEvent {
    #[doc = " The change type."]
    #[serde(rename = "type")]
    pub type_: Uinteger,
    #[doc = " The file's URI."]
    pub uri: DocumentUri,
}
#[doc = " A filter to describe in which file operation requests or notifications the server is interested "]
#[doc = " in."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileOperationFilter {
    #[doc = " The actual file operation pattern."]
    pub pattern: FileOperationPattern,
    #[doc = " A Uri like `file` or `untitled`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}
#[doc = " A pattern to describe in which file operation requests or notifications the server is "]
#[doc = " interested in."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileOperationPattern {
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
pub struct FileOperationPatternOptions {
    #[doc = " The pattern should be matched ignoring casing."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreCase")]
    pub ignore_case: Option<bool>,
}
#[doc = " The options to register for file operations."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileOperationRegistrationOptions {
    #[doc = " The actual filters."]
    pub filters: Vec<FileOperationFilter>,
}
#[doc = " Represents information on a file/folder rename."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileRename {
    #[doc = " A file:// URI for the new location of the file/folder being renamed."]
    #[serde(rename = "newUri")]
    pub new_uri: String,
    #[doc = " A file:// URI for the original location of the file/folder being renamed."]
    #[serde(rename = "oldUri")]
    pub old_uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct FoldingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FoldingRangeParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
#[doc = " Value-object describing what options formatting should use."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FormattingOptions {
    #[doc = " Insert a newline character at the end of the file if one does not exist."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertFinalNewline")]
    pub insert_final_newline: Option<bool>,
    #[doc = " Prefer spaces over tabs."]
    #[serde(rename = "insertSpaces")]
    pub insert_spaces: bool,
    #[doc = " Size of a tab in spaces."]
    #[serde(rename = "tabSize")]
    pub tab_size: Uinteger,
    #[doc = " Trim all newlines after the final newline at the end of the file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trimFinalNewlines")]
    pub trim_final_newlines: Option<bool>,
    #[doc = " Trim trailing whitespace on a line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trimTrailingWhitespace")]
    pub trim_trailing_whitespace: Option<bool>,
}
#[doc = " The result of a hover request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Hover {
    #[doc = " The hover's content"]
    #[serde(default)]
    #[serde(with = "::schemafy_core::one_or_many")]
    pub contents: Vec<MarkedString>,
    #[doc = " An optional range is a range inside a text document that is used to visualize a hover, e.g. "]
    #[doc = " by changing the background color."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct HoverOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct HoverParams {
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
pub struct ImplementationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ImplementationParams {
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
pub struct InitializeError {
    #[doc = " Indicates whether the client execute the following retry logic: (1) show the message "]
    #[doc = " provided by the ResponseError to the user (2) user selects retry or cancel (3) if user "]
    #[doc = " selected retry the initialize method is sent again."]
    pub retry: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeParamsClientInfo {
    #[doc = " The name of the client as defined by the client."]
    pub name: String,
    #[doc = " The client's version as defined by the client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeParams {
    #[doc = " The capabilities provided by the client (editor or tool)"]
    pub capabilities: ClientCapabilities,
    #[doc = " Information about the client"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientInfo")]
    pub client_info: Option<InitializeParamsClientInfo>,
    #[doc = " User provided initialization options."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initializationOptions")]
    pub initialization_options: Option<serde_json::Value>,
    #[doc = " The locale the client is currently showing the user interface in. This must not necessarily "]
    #[doc = " be the locale of the operating system."]
    #[doc = " "]
    #[doc = " Uses IETF language tags as the value's syntax (See"]
    #[doc = "<https://en.wikipedia.org/wiki/IETF_language_tag>)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = " The process Id of the parent process that started the server. Is null if the process has "]
    #[doc = " not been started by another process. If the parent process is not alive then the server "]
    #[doc = " should exit (see exit notification) its process."]
    #[serde(rename = "processId")]
    pub process_id: serde_json::Value,
    #[doc = " The rootPath of the workspace. Is null if no folder is open."]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rootPath")]
    pub root_path: Option<String>,
    #[doc = " The rootUri of the workspace. Is null if no folder is open. If both `rootPath` and "]
    #[doc = " `rootUri` are set `rootUri` wins."]
    #[serde(rename = "rootUri")]
    pub root_uri: serde_json::Value,
    #[doc = " The initial trace setting. If omitted trace is disabled ('off')."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<TraceValue>,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
    #[doc = " The workspace folders configured in the client when the server starts. This property is "]
    #[doc = " only available if the client supports workspace folders. It can be `null` if the client "]
    #[doc = " supports workspace folders but none are configured."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspaceFolders")]
    pub workspace_folders: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeResultServerInfo {
    #[doc = " The name of the server as defined by the server."]
    pub name: String,
    #[doc = " The server's version as defined by the server."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeResult {
    #[doc = " The capabilities the language server provides."]
    pub capabilities: ServerCapabilities,
    #[doc = " Information about the server."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverInfo")]
    pub server_info: Option<InitializeResultServerInfo>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct InitializedParams {}
#[doc = " A special text edit to provide an insert and a replace operation."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InsertReplaceEdit {
    #[doc = " The range if the insert is requested"]
    pub insert: Range,
    #[doc = " The string to be inserted."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range if the replace is requested."]
    pub replace: Range,
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
pub struct LinkedEditingRangeClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct LinkedEditingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LinkedEditingRangeParams {
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
pub struct LinkedEditingRanges {
    #[doc = " A list of ranges that can be renamed together. The ranges must have identical length and "]
    #[doc = " contain identical text content. The ranges cannot overlap."]
    pub ranges: Vec<Range>,
    #[doc = " An optional word pattern (regular expression) that describes valid contents for the given "]
    #[doc = " ranges. If no pattern is provided, the client configuration's word pattern will be used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wordPattern")]
    pub word_pattern: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Location {
    pub range: Range,
    pub uri: DocumentUri,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LocationLink {
    #[doc = " Span of the origin of this link."]
    #[doc = " "]
    #[doc = " Used as the underlined span for mouse interaction. Defaults to the word range at the mouse "]
    #[doc = " position."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originSelectionRange")]
    pub origin_selection_range: Option<Range>,
    #[doc = " The full target range of this link. If the target for example is a symbol then target range "]
    #[doc = " is the range enclosing this symbol not including leading/trailing whitespace but everything "]
    #[doc = " else like comments. This information is typically used to highlight the range in the "]
    #[doc = " editor."]
    #[serde(rename = "targetRange")]
    pub target_range: Range,
    #[doc = " The range that should be selected and revealed when this link is being followed, e.g the "]
    #[doc = " name of a function. Must be contained by the the `targetRange`. See also "]
    #[doc = " `DocumentSymbol#range`"]
    #[serde(rename = "targetSelectionRange")]
    pub target_selection_range: Range,
    #[doc = " The target resource identifier of this link."]
    #[serde(rename = "targetUri")]
    pub target_uri: DocumentUri,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LogMessageParams {
    #[doc = " The actual message"]
    pub message: String,
    #[doc = " The message type. See  {@link  MessageType }"]
    #[serde(rename = "type")]
    pub type_: MessageType,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LogTraceParams {
    #[doc = " The message to be logged."]
    pub message: String,
    #[doc = " Additional information that can be computed if the `trace` configuration is set to "]
    #[doc = " `'verbose'`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<String>,
}
#[doc = " Client capabilities specific to the used markdown parser."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MarkdownClientCapabilities {
    #[doc = " The name of the parser."]
    pub parser: String,
    #[doc = " The version of the parser."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[doc = " MarkedString can be used to render human readable text. It is either a markdown string or a "]
#[doc = " code-block that provides a language and a code snippet. The language identifier is semantically "]
#[doc = " equal to the optional language identifier in fenced code blocks in GitHub issues."]
#[doc = " "]
#[doc = " The pair of a language and a value is an equivalent to markdown: ```${language} ${value} ```"]
#[doc = " "]
#[doc = " Note that markdown strings will be sanitized - that means html will be escaped."]
pub type MarkedString = serde_json::Value;
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Message {
    pub jsonrpc: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MessageActionItem {
    #[doc = " A short title like 'Retry', 'Open Log' etc."]
    pub title: String,
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct MonikerClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration. If this is set to `true` the client "]
    #[doc = " supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
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
pub struct MonikerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MonikerParams {
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
pub struct MonikerRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NotificationMessage {
    pub jsonrpc: String,
    #[doc = " The method to be invoked."]
    pub method: String,
    #[doc = " The notification's params."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OptionalVersionedTextDocumentIdentifier {
    #[doc = " The text document's URI."]
    pub uri: DocumentUri,
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
#[doc = " Represents a parameter of a callable-signature. A parameter can have a label and a doc-comment."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct PartialResultParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Position {
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
pub struct PrepareRenameParams {
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
pub type PrepareSupportDefaultBehavior = f64;
pub type ProgressToken = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PublishDiagnosticsClientCapabilitiesTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<DiagnosticTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PublishDiagnosticsParams {
    #[doc = " An array of diagnostic information items."]
    pub diagnostics: Vec<Diagnostic>,
    #[doc = " The URI for which diagnostic information is reported."]
    pub uri: DocumentUri,
    #[doc = " Optional the version number of the document the diagnostics are published for."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Integer>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Range {
    #[doc = " The range's end position."]
    pub end: Position,
    #[doc = " The range's start position."]
    pub start: Position,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ReferenceClientCapabilities {
    #[doc = " Whether references supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReferenceContext {
    #[doc = " Include the declaration of the current symbol."]
    #[serde(rename = "includeDeclaration")]
    pub include_declaration: bool,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ReferenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReferenceParams {
    pub context: ReferenceContext,
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
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}
#[doc = " Client capabilities specific to regular expressions."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RegularExpressionsClientCapabilities {
    #[doc = " The engine's name."]
    pub engine: String,
    #[doc = " The engine's version."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct RenameFile {
    #[doc = " An optional annotation identifer describing the operation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annotationId")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
    #[doc = " A rename"]
    pub kind: String,
    #[doc = " The new location."]
    #[serde(rename = "newUri")]
    pub new_uri: DocumentUri,
    #[doc = " The old (existing) location."]
    #[serde(rename = "oldUri")]
    pub old_uri: DocumentUri,
    #[doc = " Rename options."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<RenameFileOptions>,
}
#[doc = " Rename file options"]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct RenameFilesParams {
    #[doc = " An array of all files/folders renamed in this operation. When a folder is renamed, only the "]
    #[doc = " folder will be included, and not its children."]
    pub files: Vec<FileRename>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct RenameParams {
    #[doc = " The new name of the symbol. If the given name is not valid the request must return a "]
    #[doc = " [ResponseError](#ResponseError) with an appropriate message set."]
    #[serde(rename = "newName")]
    pub new_name: String,
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RequestMessage {
    #[doc = " The request id."]
    pub id: serde_json::Value,
    pub jsonrpc: String,
    #[doc = " The method to be invoked."]
    pub method: String,
    #[doc = " The method's params."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ResponseError {
    #[doc = " A number indicating the error type that occurred."]
    pub code: Integer,
    #[doc = " A primitive or structured value that contains additional information about the error. Can "]
    #[doc = " be omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " A string providing a short description of the error."]
    pub message: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ResponseMessage {
    #[doc = " The error object in case a request fails."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    #[doc = " The request id."]
    pub id: serde_json::Value,
    pub jsonrpc: String,
    #[doc = " The result of a request. This member is REQUIRED on success. This member MUST NOT exist if "]
    #[doc = " there was an error invoking the method."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SaveOptions {
    #[doc = " The client is supposed to include the content on save."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeText")]
    pub include_text: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SelectionRange {
    #[doc = " The parent selection range containing this range. Therefore `parent.range` must contain "]
    #[doc = " `this.range`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<SelectionRange>>,
    #[doc = " The [range](#Range) of this selection range."]
    pub range: Range,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SelectionRangeClientCapabilities {
    #[doc = " Whether implementation supports dynamic registration for selection range providers. If this "]
    #[doc = " is set to `true` the client supports the new `SelectionRangeRegistrationOptions` return "]
    #[doc = " value for the corresponding server capability as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SelectionRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SelectionRangeParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The positions inside the text document."]
    pub positions: Vec<Position>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SemanticTokensClientCapabilitiesRequests {
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
pub struct SemanticTokensClientCapabilities {
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
    pub requests: SemanticTokensClientCapabilitiesRequests,
    #[doc = " The token modifiers that the client supports."]
    #[serde(rename = "tokenModifiers")]
    pub token_modifiers: Vec<String>,
    #[doc = " The token types that the client supports."]
    #[serde(rename = "tokenTypes")]
    pub token_types: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SemanticTokensDelta {
    #[doc = " The semantic token edits to transform a previous result into a new result."]
    pub edits: Vec<SemanticTokensEdit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultId")]
    pub result_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SemanticTokensDeltaParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The result id of a previous response. The result Id can either point to a full response or "]
    #[doc = " a delta response depending on what was received last."]
    #[serde(rename = "previousResultId")]
    pub previous_result_id: String,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SemanticTokensDeltaPartialResult {
    pub edits: Vec<SemanticTokensEdit>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct SemanticTokensLegend {
    #[doc = " The token modifiers a server uses."]
    #[serde(rename = "tokenModifiers")]
    pub token_modifiers: Vec<String>,
    #[doc = " The token types a server uses."]
    #[serde(rename = "tokenTypes")]
    pub token_types: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct SemanticTokensParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SemanticTokensPartialResult {
    pub data: Vec<Uinteger>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SemanticTokensRangeParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " The range the semantic tokens are requested for."]
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ServerCapabilitiesWorkspaceFileOperations {
    #[doc = " The server is interested in receiving didCreateFiles notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didCreate")]
    pub did_create: Option<FileOperationRegistrationOptions>,
    #[doc = " The server is interested in receiving didDeleteFiles file notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didDelete")]
    pub did_delete: Option<FileOperationRegistrationOptions>,
    #[doc = " The server is interested in receiving didRenameFiles notifications."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didRename")]
    pub did_rename: Option<FileOperationRegistrationOptions>,
    #[doc = " The server is interested in receiving willCreateFiles requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willCreate")]
    pub will_create: Option<FileOperationRegistrationOptions>,
    #[doc = " The server is interested in receiving willDeleteFiles file requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willDelete")]
    pub will_delete: Option<FileOperationRegistrationOptions>,
    #[doc = " The server is interested in receiving willRenameFiles requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willRename")]
    pub will_rename: Option<FileOperationRegistrationOptions>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ServerCapabilitiesWorkspace {
    #[doc = " The server is interested in file notifications/requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileOperations")]
    pub file_operations: Option<ServerCapabilitiesWorkspaceFileOperations>,
    #[doc = " The server supports workspace folder."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspaceFolders")]
    pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ServerCapabilities {
    #[doc = " The server provides call hierarchy support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callHierarchyProvider")]
    pub call_hierarchy_provider: Option<serde_json::Value>,
    #[doc = " The server provides code actions. The `CodeActionOptions` return type is only valid if the "]
    #[doc = " client signals code action literal support via the property "]
    #[doc = " `textDocument.codeAction.codeActionLiteralSupport`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeActionProvider")]
    pub code_action_provider: Option<serde_json::Value>,
    #[doc = " The server provides code lens."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeLensProvider")]
    pub code_lens_provider: Option<CodeLensOptions>,
    #[doc = " The server provides color provider support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "colorProvider")]
    pub color_provider: Option<serde_json::Value>,
    #[doc = " The server provides completion support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completionProvider")]
    pub completion_provider: Option<CompletionOptions>,
    #[doc = " The server provides go to declaration support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "declarationProvider")]
    pub declaration_provider: Option<serde_json::Value>,
    #[doc = " The server provides goto definition support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "definitionProvider")]
    pub definition_provider: Option<serde_json::Value>,
    #[doc = " The server provides document formatting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentFormattingProvider")]
    pub document_formatting_provider: Option<serde_json::Value>,
    #[doc = " The server provides document highlight support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentHighlightProvider")]
    pub document_highlight_provider: Option<serde_json::Value>,
    #[doc = " The server provides document link support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentLinkProvider")]
    pub document_link_provider: Option<DocumentLinkOptions>,
    #[doc = " The server provides document formatting on typing."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentOnTypeFormattingProvider")]
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    #[doc = " The server provides document range formatting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentRangeFormattingProvider")]
    pub document_range_formatting_provider: Option<serde_json::Value>,
    #[doc = " The server provides document symbol support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentSymbolProvider")]
    pub document_symbol_provider: Option<serde_json::Value>,
    #[doc = " The server provides execute command support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executeCommandProvider")]
    pub execute_command_provider: Option<ExecuteCommandOptions>,
    #[doc = " Experimental server capabilities."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
    #[doc = " The server provides folding provider support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "foldingRangeProvider")]
    pub folding_range_provider: Option<serde_json::Value>,
    #[doc = " The server provides hover support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hoverProvider")]
    pub hover_provider: Option<serde_json::Value>,
    #[doc = " The server provides goto implementation support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "implementationProvider")]
    pub implementation_provider: Option<serde_json::Value>,
    #[doc = " The server provides linked editing range support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkedEditingRangeProvider")]
    pub linked_editing_range_provider: Option<serde_json::Value>,
    #[doc = " Whether server provides moniker support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "monikerProvider")]
    pub moniker_provider: Option<serde_json::Value>,
    #[doc = " The server provides find references support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referencesProvider")]
    pub references_provider: Option<serde_json::Value>,
    #[doc = " The server provides rename support. RenameOptions may only be specified if the client "]
    #[doc = " states that it supports `prepareSupport` in its initial `initialize` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "renameProvider")]
    pub rename_provider: Option<serde_json::Value>,
    #[doc = " The server provides selection range support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectionRangeProvider")]
    pub selection_range_provider: Option<serde_json::Value>,
    #[doc = " The server provides semantic tokens support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticTokensProvider")]
    pub semantic_tokens_provider: Option<serde_json::Value>,
    #[doc = " The server provides signature help support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "signatureHelpProvider")]
    pub signature_help_provider: Option<SignatureHelpOptions>,
    #[doc = " Defines how text documents are synced. Is either a detailed structure defining each "]
    #[doc = " notification or for backwards compatibility the TextDocumentSyncKind number. If omitted it "]
    #[doc = " defaults to `TextDocumentSyncKind.None`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: Option<serde_json::Value>,
    #[doc = " The server provides goto type definition support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeDefinitionProvider")]
    pub type_definition_provider: Option<serde_json::Value>,
    #[doc = " Workspace specific server capabilities"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<ServerCapabilitiesWorkspace>,
    #[doc = " The server provides workspace symbol support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspaceSymbolProvider")]
    pub workspace_symbol_provider: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SetTraceParams {
    #[doc = " The new value that should be assigned to the trace setting."]
    pub value: TraceValue,
}
#[doc = " Client capabilities for the show document request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowDocumentClientCapabilities {
    #[doc = " The client has support for the show document request."]
    pub support: bool,
}
#[doc = " Params to show a document."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowDocumentParams {
    #[doc = " Indicates to show the resource in an external program. To show for example "]
    #[doc = " `https://code.visualstudio.com/` in the default WEB browser set `external` to `true`."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[doc = " An optional selection range if the document is a text document. Clients might ignore the "]
    #[doc = " property if an external program is started or the file is not a text file."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<Range>,
    #[doc = " An optional property to indicate whether the editor showing the document should take focus "]
    #[doc = " or not. Clients might ignore this property if an external program is started."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "takeFocus")]
    pub take_focus: Option<bool>,
    #[doc = " The document uri to show."]
    pub uri: Uri,
}
#[doc = " The result of an show document request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowDocumentResult {
    #[doc = " A boolean indicating if the show was successful."]
    pub success: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowMessageParams {
    #[doc = " The actual message."]
    pub message: String,
    #[doc = " The message type. See  {@link  MessageType } ."]
    #[serde(rename = "type")]
    pub type_: MessageType,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ShowMessageRequestClientCapabilitiesMessageActionItem {
    #[doc = " Whether the client supports additional attributes which are preserved and sent back to the "]
    #[doc = " server in the request's response."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropertiesSupport")]
    pub additional_properties_support: Option<bool>,
}
#[doc = " Show message request client capabilities"]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ShowMessageRequestClientCapabilities {
    #[doc = " Capabilities specific to the `MessageActionItem` type."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageActionItem")]
    pub message_action_item: Option<ShowMessageRequestClientCapabilitiesMessageActionItem>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowMessageRequestParams {
    #[doc = " The message action items to present."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<MessageActionItem>>,
    #[doc = " The actual message"]
    pub message: String,
    #[doc = " The message type. See  {@link  MessageType }"]
    #[serde(rename = "type")]
    pub type_: MessageType,
}