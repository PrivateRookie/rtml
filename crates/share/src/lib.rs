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
