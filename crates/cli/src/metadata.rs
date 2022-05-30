use std::{path::PathBuf, process};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MetaData {
    pub packages: Vec<Packages>,
    pub target_directory: PathBuf,
    pub workspace_root: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Packages {
    pub name: String,
    pub version: String,
    pub targets: Vec<Target>,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    pub name: String,
    pub src_path: PathBuf,
}

impl MetaData {
    pub fn load_current_meta() -> Self {
        let mut cmd = process::Command::new("cargo");
        cmd.arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps");
        let output = cmd.output().expect("failed cargo metadata");
        let stdout = output.stdout;
        let meta: MetaData = serde_json::from_slice(&stdout).expect("invalid metadata");
        meta
    }
}
