const URL: &str = "https://raw.githubusercontent.com/microsoft/language-server-protocol/main/versions/protocol-2-x.md";
fn main() {
    let body: String = ureq::get(URL)
        .call()
        .expect("failed to download content")
        .into_string()
        .expect("invalid content");
    dbg!(body);
}
