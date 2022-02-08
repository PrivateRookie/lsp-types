use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use comrak::{nodes::AstNode, parse_document, Arena, ComrakOptions};
use schemafy_lib::Expander;

const URL: &str = "https://raw.githubusercontent.com/microsoft/language-server-protocol/main/versions/protocol-2-x.md";

#[derive(Parser, Debug)]
enum Args {
    /// fetch definition from github raw content
    Fetch,
    /// generate rust struct def
    Gen {
        /// json schema file path
        source: PathBuf,
    },
}

fn main() {
    let arg = Args::parse();
    match arg {
        Args::Fetch => {
            let content = fetch_content();
            let ts = extract_ts(&content);
            println!("{}", ts);
        }
        Args::Gen { source } => {
            let mut schema = String::new();
            let mut file = File::open(source).expect("can not open file");
            file.read_to_string(&mut schema)
                .expect("read content failed");
            let schema = serde_json::from_str(&schema).unwrap();
            let mut expander = Expander::new(Some("Lsp"), "::schemafy_core::", &schema);
            let code = expander.expand(&schema);
            println!(
                "use serde::{{Deserialize, Serialize}};\n
                use serde_repr::{{Deserialize_repr, Serialize_repr}};\n\n
                {}",
                code.to_string()
            );
        }
    }
}

pub fn fetch_content() -> String {
    ureq::get(URL)
        .call()
        .expect("failed to download content")
        .into_string()
        .expect("invalid content")
}

fn extract_ts(content: &str) -> String {
    let arena = Arena::new();
    let root = parse_document(&arena, content, &ComrakOptions::default());
    let mut ts = vec![];
    iter_nodes(root, &mut ts);

    String::from_utf8(ts).unwrap()
}

fn iter_nodes<'a>(node: &'a AstNode<'a>, ts: &mut Vec<u8>) {
    match &node.data.borrow().value {
        comrak::nodes::NodeValue::CodeBlock(blk) => {
            if blk.info == "typescript".as_bytes() || blk.info == "ts".as_bytes() {
                ts.extend_from_slice(&blk.literal);
                ts.push('\n' as u8);
            }
        }
        _ => {}
    }
    for c in node.children() {
        iter_nodes(c, ts);
    }
}
