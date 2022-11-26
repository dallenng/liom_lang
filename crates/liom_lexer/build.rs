use std::env;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use liom_token::TokenKind;
use regex_automata::dense;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output =
        File::create(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("generated.rs"))?;

    output.write_all(
        br"
type StateID = u8;

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
struct Aligned<const N: usize>([u8; N]);
",
    )?;

    let mut const_str_le = format!(
        r#"
#[cfg(target_endian = "little")]
const GENERATED_DFA: [(liom_token::TokenKind, &[u8]); {}] = [
"#,
        TokenKind::KINDS_WITH_REGEX.len()
    );

    let mut const_str_be = format!(
        r#"
#[cfg(target_endian = "big")]
const GENERATED_DFA: [(liom_token::TokenKind, &[u8]); {}] = [
"#,
        TokenKind::KINDS_WITH_REGEX.len()
    );

    for (kind, regex) in TokenKind::KINDS_WITH_REGEX {
        let dfa = dense::Builder::new()
            .anchored(true)
            .minimize(true)
            .longest_match(true)
            .build_with_size::<u8>(regex)?;

        let dfa_bytes_le = dfa.to_bytes_little_endian()?;
        let dfa_bytes_be = dfa.to_bytes_big_endian()?;

        writeln!(
            const_str_le,
            "    (liom_token::TokenKind::{:?}, &Aligned({:?}).0),",
            kind, dfa_bytes_le
        )?;

        writeln!(
            const_str_be,
            "    (liom_token::TokenKind::{:?}, &Aligned({:?}).0),",
            kind, dfa_bytes_be
        )?;
    }

    writeln!(output, "{const_str_le}];")?;
    writeln!(output, "{const_str_be}];")?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
