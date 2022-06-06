use std::path::Path;

use serde::{Deserialize, Serialize};

pub mod dev_server {
    pub const WS_URL: &str = "__dev__";
    pub const DEFAULT_HOST: &str = "127.0.0.1";
    pub const DEFAULT_PORT: &str = "9001";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevMessage {
    RebuildOk,
    RebuildError(String),
}

pub fn copy_dir_all<P: AsRef<Path>>(src: P, dest: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(&dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let dest_entry = dest.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dest_entry.as_path())?;
        } else {
            std::fs::copy(entry.path(), dest_entry)?;
        }
    }
    Ok(())
}
