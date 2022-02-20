# lsp-ty

This crate provides types used for LSP, and some helper structures, macro and trait for handling message.

For more information about LSP, see [LSP - overview](https://microsoft.github.io/language-server-protocol/overviews/lsp/overview/).

## demo usage

check examples of [lsp-io](https://github.com/PrivateRookie/lsp-types/tree/main/crates/io/examples)

## add custom request, notification and response

If you want to custom request/response pair. First, define you request params and response result
type, with `Serialize` and `Deserialize` derive.

```rust
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct MyParams {
    pub name: String,
    pub pos: Position
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct MyResult {
    pub data: Vec<String>
}
```

then use `impl_req` helper macro to auto complete `FromReq` trait for you custom types

```rust
// the second param is you custom request method
impl_req!(MyParams, "custom/my", MyResult);
```

impl notification message is similar with above, except that, notification does not need to specify
response type.
