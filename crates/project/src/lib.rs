use std::{
    io::Write,
    path::{Path, PathBuf},
};

fn output_dir() -> PathBuf {
    let mut target_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    target_dir.push("..");
    target_dir.push("..");
    target_dir.push("__rtml_build__");
    target_dir
}

/// add file in dist dir
pub fn add_file<P: AsRef<Path>>(path: P, content: &[u8]) -> std::io::Result<()> {
    let dir = output_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    let mut file = std::fs::File::create(dir.join(path))?;
    file.write_all(content)?;
    Ok(())
}

/// return normalized package id("-" is transformed to "_")
pub fn get_pkg_id() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap().replace('-', "_")
}
