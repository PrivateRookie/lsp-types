# lsp-io

simple wrapper for read/write LSP message.


demo usage from example showing how to handle requests


```rust
// pass context when handle request need;
req.with(Rc::new(RefCell::new(self)))
    // pass handler function, you must specify param type
    // in anonymous handler function argument, other wise
    // you have to use turbo fish symbol
    .then(|ctx, id, _: InitializeParams| {
        let ret = InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    all_commit_characters: None,
                    resolve_provider: None,
                    trigger_characters: Some(vec!["$".to_string()]),
                    work_done_progress: None,
                }),
                ..Default::default()
            },
            server_info: Some(InitializeResultServerInfo {
                name: "yaya-server".to_string(),
                version: Some("0.0.1".to_string()),
            }),
        };
        ctx.borrow_mut().resp(id.ok_resp(ret))
    })
    // use or_else to route to other handler function if
    // method do not match
    .or_else(|ctx, id, _: CompletionParams| {
        let item = CompletionItem {
            label: "demo".to_string(),
            detail: Some("that's ok".to_string()),
            insert_text: Some("yaya".to_string()),
            kind: Some(CompletionItemKind::Keyword),
            ..Default::default()
        };
        ctx.borrow_mut().resp(id.ok_resp(vec![item]))
    })
```

