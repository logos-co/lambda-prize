use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};

use crate::EventError;

pub fn atomic_write_bytes(path: &Path, bytes: &[u8]) -> Result<(), EventError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    if !parent.as_os_str().is_empty() {
        fs::create_dir_all(parent).map_err(|e| EventError::Io(e.to_string()))?;
    }

    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("tmp");
    let tmp = path.with_file_name(format!(".{}.tmp", file_name));

    {
        let file = fs::File::create(&tmp).map_err(|e| EventError::Io(e.to_string()))?;
        let mut writer = BufWriter::new(file);
        writer.write_all(bytes).map_err(|e| EventError::Io(e.to_string()))?;
        writer.flush().map_err(|e| EventError::Io(e.to_string()))?;
    }

    fs::rename(&tmp, path).map_err(|e| EventError::Io(e.to_string()))
}

pub fn atomic_write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), EventError> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| EventError::Io(e.to_string()))?;
    atomic_write_bytes(path, &bytes)
}

pub fn read_json_file<T: DeserializeOwned>(path: &Path) -> Result<T, EventError> {
    let raw = fs::read_to_string(path).map_err(|e| EventError::Io(e.to_string()))?;
    serde_json::from_str(&raw).map_err(|e| EventError::Io(e.to_string()))
}
