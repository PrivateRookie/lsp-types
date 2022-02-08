use serde::{Deserialize, Serialize};

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CancelParams {
    #[doc = " The request id to cancel."]
    pub id: serde_json::Value,
}
pub type ClientCapabilities = ::std::collections::BTreeMap<String, serde_json::Value>;
#[doc = " Contains additional diagnostic information about the context in which"]
#[doc = " a code action is run."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionContext {
    #[doc = " An array of diagnostics."]
    pub diagnostics: Vec<Diagnostic>,
}
#[doc = " Params for the CodeActionRequest"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeActionParams {
    #[doc = " Context carrying additional information."]
    pub context: CodeActionContext,
    #[doc = " The range for which the command was invoked."]
    pub range: Range,
    #[doc = " The document in which the command was invoked."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[doc = " A code lens represents a command that should be shown along with"]
#[doc = " source text, like the number of references, a way to run tests, etc."]
#[doc = " "]
#[doc = " A code lens is _unresolved_ when no command is associated to it. For performance"]
#[doc = " reasons the creation of a code lens and resolving should be done in two stages."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeLens {
    #[doc = " The command this code lens represents."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[doc = " A data entry field that is preserved on a code lens item between"]
    #[doc = " a code lens and a code lens resolve request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " The range in which this code lens is valid. Should only span a single line."]
    pub range: Range,
}
#[doc = " Code Lens options."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CodeLensOptions {
    #[doc = " Code lens has a resolve provider as well."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CodeLensParams {
    #[doc = " The document to request code lens for."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Command {
    #[doc = " Arguments that the command handler should be"]
    #[doc = " invoked with."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
    #[doc = " The identifier of the actual command handler."]
    pub command: String,
    #[doc = " Title of the command, like `save`."]
    pub title: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionItem {
    #[doc = " An optional array of additional text edits that are applied when"]
    #[doc = " selecting this completion. Edits must not overlap with the main edit"]
    #[doc = " nor with themselves."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalTextEdits")]
    pub additional_text_edits: Option<Vec<TextEdit>>,
    #[doc = " An optional command that is executed *after* inserting this completion. *Note* that"]
    #[doc = " additional modifications to the current document should be described with the"]
    #[doc = " additionalTextEdits-property."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[doc = " An data entry field that is preserved on a completion item between"]
    #[doc = " a completion and a completion resolve request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = " A human-readable string with additional information"]
    #[doc = " about this item, like type or symbol information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = " A human-readable string that represents a doc-comment."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[doc = " A string that should be used when filtering a set of"]
    #[doc = " completion items. When `falsy` the label is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filterText")]
    pub filter_text: Option<String>,
    #[doc = " A string that should be inserted a document when selecting"]
    #[doc = " this completion. When `falsy` the label is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "insertText")]
    pub insert_text: Option<String>,
    #[doc = " The kind of this completion item. Based of the kind"]
    #[doc = " an icon is chosen by the editor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<f64>,
    #[doc = " The label of this completion item. By default"]
    #[doc = " also the text that is inserted when selecting"]
    #[doc = " this completion."]
    pub label: String,
    #[doc = " A string that should be used when comparing this item"]
    #[doc = " with other items. When `falsy` the label is used."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortText")]
    pub sort_text: Option<String>,
    #[doc = " An edit which is applied to a document when selecting"]
    #[doc = " this completion. When an edit is provided the value of"]
    #[doc = " insertText is ignored."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textEdit")]
    pub text_edit: Option<TextEdit>,
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
}
#[doc = " Represents a collection of [completion items](#CompletionItem) to be presented"]
#[doc = " in the editor."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CompletionList {
    #[doc = " This list it not complete. Further typing should result in recomputing"]
    #[doc = " this list."]
    #[serde(rename = "isIncomplete")]
    pub is_incomplete: bool,
    #[doc = " The completion items."]
    pub items: Vec<CompletionItem>,
}
#[doc = " Completion options."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CompletionOptions {
    #[doc = " The server provides support to resolve additional information for a completion item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[doc = " The characters that trigger completion automatically."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Diagnostic {
    #[doc = " The diagnostic's code. Can be omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<serde_json::Value>,
    #[doc = " The diagnostic's message."]
    pub message: String,
    #[doc = " The range at which the message applies."]
    pub range: Range,
    #[doc = " The diagnostic's severity. Can be omitted. If omitted it is up to the"]
    #[doc = " client to interpret diagnostics as error, warning, info or hint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<f64>,
    #[doc = " A human-readable string describing the source of this"]
    #[doc = " diagnostic, e.g. 'typescript' or 'super lint'."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[doc = ""]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeConfigurationParams {
    #[doc = " The actual changed settings"]
    pub settings: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeTextDocumentParams {
    #[doc = " The actual content changes."]
    #[serde(rename = "contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
    #[doc = " The document that did change. The version number points"]
    #[doc = " to the version after all provided content changes have"]
    #[doc = " been applied."]
    #[serde(rename = "textDocument")]
    pub text_document: VersionedTextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DidChangeWatchedFilesParams {
    #[doc = " The actual file events."]
    pub changes: Vec<FileEvent>,
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
    #[doc = " The document that was saved."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentFormattingParams {
    #[doc = " The format options."]
    pub options: FormattingOptions,
    #[doc = " The document to format."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[doc = " A document highlight is a range inside a text document which deserves"]
#[doc = " special attention. Usually a document highlight is visualized by changing"]
#[doc = " the background color of its range."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentHighlight {
    #[doc = " The highlight kind, default is DocumentHighlightKind.Text."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<f64>,
    #[doc = " The range this highlight applies to."]
    pub range: Range,
}
#[doc = "  A document highlight kind."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum DocumentHighlightKind {
    Text = 1,
    Read = 2,
    Write = 3,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentLinkParams {
    #[doc = " The document to provide document links for."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[doc = " Format document on type options"]
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
    #[doc = " The position at which this request was sent."]
    pub position: Position,
    #[doc = " The document to format."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
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
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DocumentSymbolParams {
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum FileChangeType {
    Created = 1,
    Changed = 2,
    Deleted = 3,
}
#[doc = " An event describing a file change."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FileEvent {
    #[doc = " The change type."]
    #[serde(rename = "type")]
    pub type_: f64,
    #[doc = " The file's URI."]
    pub uri: String,
}
#[doc = " Value-object describing what options formatting should use."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FormattingOptions {
    #[doc = " Prefer spaces over tabs."]
    #[serde(rename = "insertSpaces")]
    pub insert_spaces: bool,
    #[doc = " Size of a tab in spaces."]
    #[serde(rename = "tabSize")]
    pub tab_size: f64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HoverContents {
    One(MarkedString),
    Array(Vec<MarkedString>),
}
#[doc = " The result of a hover request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Hover {
    #[doc = " The hover's content"]
    pub contents: HoverContents,
    #[doc = " An optional range is a range inside a text document "]
    #[doc = " that is used to visualize a hover, e.g. by changing the background color."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeError {
    #[doc = " Indicates whether the client should retry to send the"]
    #[doc = " initialize request after showing the message provided"]
    #[doc = " in the ResponseError."]
    pub retry: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeParams {
    #[doc = " The capabilities provided by the client (editor)"]
    pub capabilities: ClientCapabilities,
    #[doc = " User provided initialization options."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initializationOptions")]
    pub initialization_options: Option<serde_json::Value>,
    #[doc = " The process Id of the parent process that started"]
    #[doc = " the server. Is null if the process has not been started by another process."]
    #[doc = " If the parent process is not alive then the server should exit (see exit notification) its "]
    #[doc = " process."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "processId")]
    pub process_id: Option<f64>,
    #[doc = " The rootPath of the workspace. Is null"]
    #[doc = " if no folder is open."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rootPath")]
    pub root_path: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeResult {
    #[doc = " The capabilities the language server provides."]
    pub capabilities: ServerCapabilities,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Location {
    pub range: Range,
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LogMessageParams {
    #[doc = " The actual message"]
    pub message: String,
    #[doc = " The message type. See{@linkMessageType}"]
    #[serde(rename = "type")]
    pub type_: f64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MarkedStringType {
    Str(String),
    Obj { language: String, value: String },
}
#[doc = " The marked string is rendered:"]
#[doc = " - as markdown if it is represented as a string"]
#[doc = " - as code block of the given language if it is represented as a pair of a language and a value"]
#[doc = " "]
#[doc = " The pair of a language and a value is an equivalent to markdown:"]
#[doc = " ```${language}"]
#[doc = " ${value}"]
#[doc = " ```"]
pub type MarkedString = MarkedStringType;
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NotificationMessage {
    pub jsonrpc: String,
    #[doc = " The method to be invoked."]
    pub method: String,
    #[doc = " The notification's params."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
#[doc = " Represents a parameter of a callable-signature. A parameter can"]
#[doc = " have a label and a doc-comment."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ParameterInformation {
    #[doc = " The human-readable doc-comment of this parameter. Will be shown"]
    #[doc = " in the UI but can be omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[doc = " The label of this parameter. Will be shown in"]
    #[doc = " the UI."]
    pub label: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Position {
    #[doc = " Character offset on a line in a document (zero-based)."]
    pub character: f64,
    #[doc = " Line position in a document (zero-based)."]
    pub line: f64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PublishDiagnosticsParams {
    #[doc = " An array of diagnostic information items."]
    pub diagnostics: Vec<Diagnostic>,
    #[doc = " The URI for which diagnostic information is reported."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Range {
    #[doc = " The range's end position."]
    pub end: Position,
    #[doc = " The range's start position."]
    pub start: Position,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReferenceContext {
    #[doc = " Include the declaration of the current symbol."]
    #[serde(rename = "includeDeclaration")]
    pub include_declaration: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReferenceParams {
    pub context: ReferenceContext,
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RenameParams {
    #[doc = " The new name of the symbol. If the given name is not valid the"]
    #[doc = " request must return a [ResponseError](#ResponseError) with an"]
    #[doc = " appropriate message set."]
    #[serde(rename = "newName")]
    pub new_name: String,
    #[doc = " The position at which this request was sent."]
    pub position: Position,
    #[doc = " The document to format."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ResponseError<T: Clone + PartialEq + std::fmt::Debug> {
    #[doc = " A number indicating the error type that occurred."]
    pub code: f64,
    #[doc = " A Primitive or Structured value that contains additional"]
    #[doc = " information about the error. Can be omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[doc = " A string providing a short description of the error."]
    pub message: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ResponseMessage<T: Clone + PartialEq + std::fmt::Debug> {
    #[doc = " The error object in case a request fails."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError<T>>,
    #[doc = " The request id."]
    pub id: serde_json::Value,
    pub jsonrpc: String,
    #[doc = " The result of a request. This can be omitted in"]
    #[doc = " the case of an error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ServerCapabilities {
    #[doc = " The server provides code actions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeActionProvider")]
    pub code_action_provider: Option<bool>,
    #[doc = " The server provides code lens."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeLensProvider")]
    pub code_lens_provider: Option<CodeLensOptions>,
    #[doc = " The server provides completion support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completionProvider")]
    pub completion_provider: Option<CompletionOptions>,
    #[doc = " The server provides goto definition support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "definitionProvider")]
    pub definition_provider: Option<bool>,
    #[doc = " The server provides document formatting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentFormattingProvider")]
    pub document_formatting_provider: Option<bool>,
    #[doc = " The server provides document highlight support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentHighlightProvider")]
    pub document_highlight_provider: Option<bool>,
    #[doc = " The server provides document formatting on typing."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentOnTypeFormattingProvider")]
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    #[doc = " The server provides document range formatting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentRangeFormattingProvider")]
    pub document_range_formatting_provider: Option<bool>,
    #[doc = " The server provides document symbol support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentSymbolProvider")]
    pub document_symbol_provider: Option<bool>,
    #[doc = " The server provides hover support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hoverProvider")]
    pub hover_provider: Option<bool>,
    #[doc = " The server provides find references support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referencesProvider")]
    pub references_provider: Option<bool>,
    #[doc = " The server provides rename support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "renameProvider")]
    pub rename_provider: Option<bool>,
    #[doc = " The server provides signature help support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "signatureHelpProvider")]
    pub signature_help_provider: Option<SignatureHelpOptions>,
    #[doc = " Defines how text documents are synced."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: Option<f64>,
    #[doc = " The server provides workspace symbol support."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspaceSymbolProvider")]
    pub workspace_symbol_provider: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowMessageParams {
    #[doc = " The actual message."]
    pub message: String,
    #[doc = " The message type. See{@linkMessageType}."]
    #[serde(rename = "type")]
    pub type_: f64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ShowMessageRequestParams {
    #[doc = " The message action items to present."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<MessageActionItem>>,
    #[doc = " The actual message"]
    pub message: String,
    #[doc = " The message type. See{@linkMessageType}"]
    #[serde(rename = "type")]
    pub type_: f64,
}
#[doc = " Signature help represents the signature of something"]
#[doc = " callable. There can be multiple signature but only one"]
#[doc = " active and only one active parameter."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SignatureHelp {
    #[doc = " The active parameter of the active signature."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeParameter")]
    pub active_parameter: Option<f64>,
    #[doc = " The active signature."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeSignature")]
    pub active_signature: Option<f64>,
    #[doc = " One or more signatures."]
    pub signatures: Vec<SignatureInformation>,
}
#[doc = " Signature help options."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SignatureHelpOptions {
    #[doc = " The characters that trigger signature help automatically."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
}
#[doc = " Represents the signature of something callable. A signature"]
#[doc = " can have a label, like a function-name, a doc-comment, and"]
#[doc = " a set of parameters."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SignatureInformation {
    #[doc = " The human-readable doc-comment of this signature. Will be shown"]
    #[doc = " in the UI but can be omitted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[doc = " The label of this signature. Will be shown in"]
    #[doc = " the UI."]
    pub label: String,
    #[doc = " The parameters of this signature."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterInformation>>,
}
#[doc = " Represents information about programming constructs like variables, classes,"]
#[doc = " interfaces etc."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SymbolInformation {
    #[doc = " The name of the symbol containing this symbol."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerName")]
    pub container_name: Option<String>,
    #[doc = " The kind of this symbol."]
    pub kind: f64,
    #[doc = " The location of this symbol."]
    pub location: Location,
    #[doc = " The name of this symbol."]
    pub name: String,
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
}
#[doc = " An event describing a change to a text document. If range and rangeLength are omitted"]
#[doc = " the new text is considered to be the full content of the document."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentContentChangeEvent {
    #[doc = " The range of the document that changed."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[doc = " The length of the range that got replaced."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rangeLength")]
    pub range_length: Option<f64>,
    #[doc = " The new text of the document."]
    pub text: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentIdentifier {
    #[doc = " The text document's URI."]
    pub uri: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentItem {
    #[doc = " The text document's language identifier."]
    #[serde(rename = "languageId")]
    pub language_id: String,
    #[doc = " The content of the opened text document."]
    pub text: String,
    #[doc = " The text document's URI."]
    pub uri: String,
    #[doc = " The version number of this document (it will strictly increase after each"]
    #[doc = " change, including undo/redo)."]
    pub version: f64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentPositionParams {
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[doc = " Defines how the host (editor) should sync document changes to the language server."]
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextEdit {
    #[doc = " The string to be inserted. For delete operations use an"]
    #[doc = " empty string."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range of the text document to be manipulated. To insert"]
    #[doc = " text into a document create a range where start === end."]
    pub range: Range,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct VersionedTextDocumentIdentifier {
    #[doc = " The text document's URI."]
    pub uri: String,
    #[doc = " The version number of this document."]
    pub version: f64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WorkspaceEdit {
    #[doc = " Holds changes to existing resources."]
    pub changes: ::std::collections::BTreeMap<String, Vec<TextEdit>>,
}
#[doc = " The parameters of a Workspace Symbol Request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WorkspaceSymbolParams {
    #[doc = " A non-empty query string"]
    pub query: String,
}
pub type Lsp = serde_json::Value;
