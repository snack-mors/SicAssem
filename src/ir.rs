#[derive(Debug)]
pub struct Line {
    pub address: i32,
    pub label: Option<String>,
    pub mnemonic: String,
    pub operand: Option<String>,
    pub source_line: usize,
}

impl Line {
    // A standard "constructor" in Rust.
    // We take &str arguments to make the calling code cleaner.
    pub fn new(
        address: i32,
        label: Option<&str>,
        mnemonic: &str,
        operand: Option<&str>,
        source_line: usize
    ) -> Self {
        Line {
            address,
            source_line,
            label: label.map(|s| s.to_string()),
            mnemonic: mnemonic.to_string(),
            operand: operand.map(|s| s.to_string()),
        }
    }
}