use anyhow::Result;
use std::io::{self, Read};

/// Read all of stdin into a `String`.
pub fn read_stdin_all() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
