use super::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[doc = " Signature help represents the signature of something callable. There can be multiple signature "]
#[doc = " but only one active and only one active parameter."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct SignatureHelpClientCapabilitiesSignatureInformationParameterInformation {
    #[doc = " The client supports processing label offsets instead of a simple label string."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "labelOffsetSupport")]
    pub label_offset_support: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct SignatureHelpParams {
    #[doc = " The signature help context. This is only available if the client specifies to send this "]
    #[doc = " using the client capability `textDocument.signatureHelp.contextSupport === true`"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<SignatureHelpContext>,
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
    pub documentation: Option<OneOf<String, MarkupContent>>,
    #[doc = " The label of this signature. Will be shown in the UI."]
    pub label: String,
    #[doc = " The parameters of this signature."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterInformation>>,
}
#[doc = " Static registration options to be returned in the initialize request."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct StaticRegistrationOptions {
    #[doc = " The id used to register the request. The id can be used to deregister the request again. "]
    #[doc = " See also Registration#id."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[doc = " Represents information about programming constructs like variables, classes, interfaces etc."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
    pub location: Location,
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
#[doc = " Text document specific client capabilities."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TextDocumentClientCapabilities {
    #[doc = " Capabilities specific to the various call hierarchy requests."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callHierarchy")]
    pub call_hierarchy: Option<CallHierarchyClientCapabilities>,
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
    pub moniker: Option<MonikerClientCapabilities>,
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
    pub semantic_tokens: Option<SemanticTokensClientCapabilities>,
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[doc = " An event describing a change to a text document. If range and rangeLength are omitted the new "]
#[doc = " text is considered to be the full content of the document."]
pub enum TextDocumentContentChangeEvent {
    Simple {
        #[doc = "The new text of the whole document."]
        text: String,
    },
    Complex {
        #[doc = "The range of the document that changed."]
        range: Range,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "rangeLength")]
        #[doc = "The optional length of the range that got replaced."]
        #[doc = "@deprecated use range instead."]
        range_length: Option<Uinteger>,
        #[doc = "The new text of the whole document."]
        text: String,
    },
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentEdit {
    #[doc = " The edits to be applied."]
    pub edits: Vec<OneOf<TextEdit, AnnotatedTextEdit>>,
    #[doc = " The text document to change."]
    #[serde(rename = "textDocument")]
    pub text_document: OptionalVersionedTextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TextDocumentIdentifier {
    #[doc = " The text document's URI."]
    pub uri: DocumentUri,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TextDocumentItem {
    #[doc = " The text document's language identifier."]
    #[serde(rename = "languageId")]
    pub language_id: String,
    #[doc = " The content of the opened text document."]
    pub text: String,
    #[doc = " The text document's URI."]
    pub uri: DocumentUri,
    #[doc = " The version number of this document (it will increase after each change, including "]
    #[doc = " undo/redo)."]
    pub version: Integer,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentPositionParams {
    #[doc = " The position inside the text document."]
    pub position: Position,
    #[doc = " The text document."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[doc = " General text document registration options."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextDocumentRegistrationOptions {
    #[doc = " A document selector to identify the scope of the registration. If set to null the document "]
    #[doc = " selector provided on the client side will be used."]
    #[serde(rename = "documentSelector")]
    pub document_selector: Option<DocumentSelector>,
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

impl Default for TextDocumentSyncKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
    #[doc = "If present will save notifications are sent to the server. If omitted"]
    #[doc = "the notification should not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willSave")]
    pub will_save: Option<bool>,
    #[doc = "If present will save notifications are sent to the server. If omitted"]
    #[doc = "the notification should not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "willSaveWaitUntil")]
    pub will_save_until: Option<bool>,
    #[doc = "If present save notifications are sent to the server. If omitted the"]
    #[doc = "notification should not be sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<OneOf<bool, SaveOptions>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TextEdit {
    #[doc = " The string to be inserted. For delete operations use an empty string."]
    #[serde(rename = "newText")]
    pub new_text: String,
    #[doc = " The range of the text document to be manipulated. To insert text into a document create a "]
    #[doc = " range where start === end."]
    pub range: Range,
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

impl Default for TraceValue {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct TypeDefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TypeDefinitionParams {
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
pub type Uri = String;
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
pub struct Unregistration {
    #[doc = " The id used to unregister the request or notification. Usually an id provided during the "]
    #[doc = " register request."]
    pub id: String,
    #[doc = " The method / capability to unregister for."]
    pub method: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct UnregistrationParams {
    pub unregisterations: Vec<Unregistration>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct VersionedTextDocumentIdentifier {
    #[doc = " The text document's URI."]
    pub uri: DocumentUri,
    #[doc = " The version number of this document."]
    #[doc = " "]
    #[doc = " The version number of a document will increase after each change, including undo/redo. The "]
    #[doc = " number doesn't need to be consecutive."]
    pub version: Integer,
}
#[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum WatchKind {
    Create = 1,
    Change = 2,
    Delete = 4,
}
#[doc = " The parameters send in a will save text document notification."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WillSaveTextDocumentParams {
    #[doc = " The 'TextDocumentSaveReason'."]
    pub reason: TextDocumentSaveReason,
    #[doc = " The document that will be saved."]
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
pub struct WorkDoneProgressCancelParams {
    #[doc = " The token to be used to report progress."]
    pub token: ProgressToken,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WorkDoneProgressCreateParams {
    #[doc = " The token to be used to report progress."]
    pub token: ProgressToken,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WorkDoneProgressEnd {
    pub kind: String,
    #[doc = " Optional, a final message indicating to for example indicate the outcome of the operation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkDoneProgressOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkDoneProgressParams {
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
    pub changes: Option<::std::collections::BTreeMap<String, Vec<TextEdit>>>,
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
    pub document_changes: Option<OneOf<Vec<TextDocumentEdit>, DocumentChange>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum DocumentChange {
    Edit(TextDocumentEdit),
    Create(CreateFile),
    Rename(RenameFile),
    Delete(DeleteFile),
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceEditClientCapabilitiesChangeAnnotationSupport {
    #[doc = " Whether the client groups edits with equal labels into tree nodes, for instance all edits "]
    #[doc = " labelled with \"Changes in Strings\" would be a tree node."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupsOnLabel")]
    pub groups_on_label: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
pub struct WorkspaceFolder {
    #[doc = " The name of the workspace folder. Used to refer to this workspace folder in the user "]
    #[doc = " interface."]
    pub name: String,
    #[doc = " The associated URI for this workspace folder."]
    pub uri: DocumentUri,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WorkspaceFolderParams {}
#[doc = " The workspace folder change event."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceFoldersChangeEvent {
    #[doc = " The array of added workspace folders"]
    pub added: Vec<WorkspaceFolder>,
    #[doc = " The array of the removed workspace folders"]
    pub removed: Vec<WorkspaceFolder>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceFoldersServerCapabilities {
    #[doc = " Whether the server wants to receive workspace folder change notifications."]
    #[doc = " "]
    #[doc = " If a string is provided, the string is treated as an ID under which the notification is "]
    #[doc = " registered on the client side. The ID can be used to unregister for these events using the "]
    #[doc = " `client/unregisterCapability` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changeNotifications")]
    pub change_notifications: Option<OneOf<bool, String>>,
    #[doc = " The server has support for workspace folders"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceSymbolClientCapabilitiesSymbolKind {
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
pub struct WorkspaceSymbolClientCapabilitiesTagSupport {
    #[doc = " The tags supported by the client."]
    #[serde(rename = "valueSet")]
    pub value_set: Vec<SymbolTag>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceSymbolClientCapabilities {
    #[doc = " Symbol request supports dynamic registration."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicRegistration")]
    pub dynamic_registration: Option<bool>,
    #[doc = " Specific capabilities for the `SymbolKind` in the `workspace/symbol` request."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "symbolKind")]
    pub symbol_kind: Option<WorkspaceSymbolClientCapabilitiesSymbolKind>,
    #[doc = " The client supports tags on `SymbolInformation`. Clients supporting tags have to handle "]
    #[doc = " unknown tags gracefully."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagSupport")]
    pub tag_support: Option<WorkspaceSymbolClientCapabilitiesTagSupport>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " The parameters of a Workspace Symbol Request."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WorkspaceSymbolParams {
    #[doc = " An optional token that a server can use to report partial results (e.g. streaming) to the "]
    #[doc = " client."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partialResultToken")]
    pub partial_result_token: Option<ProgressToken>,
    #[doc = " A query string to filter symbols by. Clients may send an empty string here to request all "]
    #[doc = " symbols."]
    pub query: String,
    #[doc = " An optional token that a server can use to report work done progress."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneToken")]
    pub work_done_token: Option<ProgressToken>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WorkspaceSymbolRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}
#[doc = " Defines a decimal number. Since decimal numbers are very rare in the language server "]
#[doc = " specification we denote the exact range with every decimal using the mathematics interval "]
#[doc = " notation (e.g. [0, 1] denotes all decimals d with 0 <= d <= 1."]
pub type Decimal = f64;
#[doc = " Defines an integer number in the range of -2^31 to 2^31 - 1."]
pub type Integer = i32;
#[doc = " Defines an unsigned integer number in the range of 0 to 2^31 - 1."]
pub type Uinteger = u32;
