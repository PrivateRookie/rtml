use rtml_share::{
    copy_dir_all,
    dev_server::{DEFAULT_HOST, DEFAULT_PORT, WS_URL},
};
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

/// auto collect assets file to dist dir
/// **source** should be relative path to root
pub fn add_assets<P: AsRef<Path>>(source: P) -> std::io::Result<()> {
    let dist = output_dir();
    if !dist.exists() {
        std::fs::create_dir_all(&dist)?;
    }
    if source.as_ref().is_file() {
        let dest = dist.join(&source);
        std::fs::copy(&source, dest)?;
    } else {
        let dest = dist.join(source.as_ref().components().skip(1).collect::<PathBuf>());
        copy_dir_all(source, dest.as_path())?;
    }
    Ok(())
}

/// return normalized package id("-" is transformed to "_")
pub fn get_pkg_id() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap().replace('-', "_")
}

/// auto reload page if source change, if you only want to auto
/// reload in debug mode, [`check debug_auto_reload`]
pub fn auto_reload() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let loc = window.location();
    let url = format!(
        "ws://{}:{}/{WS_URL}",
        loc.hostname()
            .map(|x| x.as_str().to_string())
            .unwrap_or_else(|_| DEFAULT_HOST.into()),
        loc.port()
            .map(|x| x.as_str().to_string())
            .unwrap_or_else(|_| DEFAULT_PORT.into())
    );
    let ws = web_sys::WebSocket::new(&url).unwrap();
    let reload = Closure::wrap(Box::new(move || {
        if let Err(e) = loc.reload() {
            tracing::error!("failed to reload {:?}", e)
        }
    }) as Box<dyn Fn()>);
    ws.set_onmessage(Some(reload.as_ref().unchecked_ref()));
    reload.forget();
}

/// auto reload page if source change in debug mode
pub fn debug_auto_reload() {
    #[cfg(debug_assertions)]
    {
        auto_reload()
    }
}
