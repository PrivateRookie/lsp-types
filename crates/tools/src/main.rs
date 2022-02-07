use comrak::{nodes::AstNode, parse_document, Arena, ComrakOptions};

const URL: &str = "https://raw.githubusercontent.com/microsoft/language-server-protocol/main/versions/protocol-2-x.md";
fn main() {
    let body = include_str!("./content.md");
    let ts = extract_ts(&body);
    println!("{}", ts);
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
