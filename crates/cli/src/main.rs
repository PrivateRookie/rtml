use clap::Parser;
use futures_util::{stream::SplitSink, SinkExt};
use owo_colors::OwoColorize;
use rtml_share::{
    copy_dir_all,
    dev_server::{DEFAULT_HOST, DEFAULT_PORT, WS_URL},
};
use std::{
    io::{Read, Seek, Write},
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    process,
    sync::mpsc::channel,
    time::Duration,
};
use tokio::sync::mpsc::{self, Receiver, Sender};
use warp::ws::{Message, WebSocket};

mod metadata;

/// rtml project command line interface
#[derive(Debug, Clone, Parser)]
pub enum Commands {
    /// create new rtml project
    New {
        /// project path
        path: PathBuf,
        /// project name,
        #[clap(short, long)]
        name: Option<String>,
    },
    /// init rtml project
    Init {
        /// project path
        #[clap(short, long)]
        path: Option<PathBuf>,
    },
    /// start dev server
    Dev {
        /// listen host
        #[clap(long, default_value = DEFAULT_HOST)]
        host: String,
        /// listen port
        #[clap(long, default_value = DEFAULT_PORT)]
        port: u16,
        /// if set, rebuild if package change, unit(millisecond)
        #[clap(short, long)]
        watch: Option<u64>,

        /// package name
        #[clap(short, long)]
        package: Option<String>,
    },
    /// build
    Build {
        /// use release build
        #[clap(short, long)]
        release: bool,

        /// package name
        #[clap(short, long)]
        package: Option<String>,
    },
}

fn main() {
    let cmd = Commands::parse();
    match cmd {
        Commands::New { path, name } => {
            run_cargo_new(&path, name);
            add_lib_rs(&path);
            config_cargo(&path);
            add_build_rs(&path);
        }
        Commands::Init { path } => {
            init_project(&path.unwrap_or_else(|| PathBuf::from(".")));
        }
        Commands::Dev {
            package,
            host,
            port,
            watch,
        } => {
            build_wasm(package.clone(), false);
            let dist = run_wasm_bindgen(package.clone(), false);
            let (tx, rx) = mpsc::channel(1);

            if let Some(interval) = watch {
                let meta = metadata::MetaData::load_current_meta();
                let manifest = match package.as_ref() {
                    Some(package) => {
                        let p = meta.packages.iter().find(|p| p.name == *package).unwrap();
                        p.manifest_path.parent().unwrap().into()
                    }
                    None => meta.workspace_root,
                };
                std::thread::spawn(move || start_watch_thread(interval, package, manifest, tx));
            }
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(start_http_server(dist, host, port, rx))
        }
        Commands::Build { package, release } => {
            build_wasm(package.clone(), release);
            run_wasm_bindgen(package, release);
        }
    }
}

macro_rules! expect {
    ($e:expr, $tpl:literal) => {
        $e.map_err(|e| {
            println!("{}: {:?}", $tpl.red(), e);
        })
        .unwrap()
    };
}

macro_rules! error {
    ($($t:tt),+) => {
        println!("{}", format!($($t),+).red());
    };
}
macro_rules! info {
    ($($t:tt),+) => {
        println!("{}", format!($($t),+).green());
    };
}

fn build_wasm(package: Option<String>, release: bool) {
    let mut cmd = process::Command::new("cargo");
    cmd.arg("build");
    if let Some(package) = package {
        cmd.arg("-p");
        cmd.arg(package);
    }
    if release {
        cmd.arg("--release");
    }
    cmd.arg("--target");
    cmd.arg("wasm32-unknown-unknown");
    match cmd.output() {
        Err(e) => {
            error!("unable to call cargo build {:}", e);
        }
        Ok(output) => {
            if !output.stderr.is_empty() {
                let msg = String::from_utf8_lossy(&output.stderr);
                print!("{}", msg);
            }
            let msg = String::from_utf8_lossy(&output.stdout);
            print!("{}", msg);
        }
    }
}

fn run_wasm_bindgen(package: Option<String>, release: bool) -> PathBuf {
    let src_wasm = get_built_wasm(package, release);

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let mut target_dir = PathBuf::from(&target_dir);
    let dist_dir = std::env::var("RTML_DIST").unwrap_or_else(|_| "dist".to_string());
    target_dir.push(dist_dir);

    let mut cmd = process::Command::new("wasm-bindgen");
    cmd.arg(&src_wasm);
    cmd.arg("--out-dir");
    cmd.arg(&target_dir);
    cmd.arg("--target");
    cmd.arg("web");
    cmd.arg("--no-typescript");
    if release {
        cmd.arg("--remove-name-section");
        cmd.arg("--remove-producers-section");
    } else {
        cmd.arg("--debug");
        cmd.arg("--keep-debug");
    }
    match cmd.output() {
        Err(e) => {
            error!("unable to run wasm-bindgen {:}", e);
        }
        Ok(output) => {
            if !output.stderr.is_empty() {
                let msg = String::from_utf8_lossy(&output.stderr);
                print!("{}", msg);
            }
            let msg = String::from_utf8_lossy(&output.stdout);
            print!("{}", msg);
        }
    }

    expect! {
        std::fs::copy(
            &src_wasm,
            target_dir.join(src_wasm.file_name().unwrap_or_default()),
        ),
        "copy wasm file failed"
    };

    for entry in std::fs::read_dir(get_out_dir(release)).unwrap().flatten() {
        let src = entry.path();
        let dest: PathBuf = src.components().skip(5).collect();
        if src.is_file() {
            std::fs::copy(src, target_dir.join(dest)).unwrap();
        } else {
            copy_dir_all(src, target_dir.join(dest).as_path()).unwrap();
        }
    }
    target_dir
}

fn read_package() -> String {
    let mut file = expect!(std::fs::File::open("Cargo.toml"), "no Cargo.toml found");
    let mut content = String::new();
    expect!(
        file.read_to_string(&mut content),
        "failed to read Cargo.toml"
    );
    let cargo = expect!(content.parse::<toml_edit::Document>(), "invalid Cargo.toml");
    expect!(
        cargo["package"]["name"]
            .as_str()
            .ok_or("empty package name"),
        ""
    )
    .to_string()
}

fn get_built_wasm(package: Option<String>, release: bool) -> PathBuf {
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let mut target_dir = PathBuf::from(&target_dir);
    target_dir.push("wasm32-unknown-unknown");
    if release {
        target_dir.push("release");
    } else {
        target_dir.push("debug");
    }
    let pkg = package.unwrap_or_else(read_package).replace('-', "_");
    target_dir.push(format!("{pkg}.wasm"));
    target_dir
}

fn get_out_dir(release: bool) -> PathBuf {
    let mut out_dir = PathBuf::new();
    out_dir.push("target");
    out_dir.push("wasm32-unknown-unknown");
    if release {
        out_dir.push("release");
    } else {
        out_dir.push("debug");
    };
    out_dir.push("build");
    out_dir.push("__rtml_build__");
    out_dir
}

async fn notify_after_rebuild(
    mut rebuild_rx: Receiver<()>,
    mut ws_rx: Receiver<SplitSink<WebSocket, Message>>,
) {
    let mut clients: Vec<SplitSink<WebSocket, Message>> = vec![];
    loop {
        tokio::select! {
            client = ws_rx.recv() => {
                if let Some(client) = client {
                    clients.push(client)
                }
            },
            _ = rebuild_rx.recv() => {
                let mut to_delete = vec![];
                for (idx, client) in clients.iter_mut().enumerate()    {
                    if client.send(Message::text("reload")).await.is_err() {
                        to_delete.push(idx);
                    };
                }
                for (offset, idx) in to_delete.iter().enumerate() {
                    if clients.remove(idx-offset).close().await.is_err() {};
                }
            }
        };
    }
}

async fn start_http_server(dist: PathBuf, host: String, port: u16, rebuild_rx: Receiver<()>) {
    use futures_util::StreamExt;
    use warp::Filter;
    let index_html = dist.join("index.html");

    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(notify_after_rebuild(rebuild_rx, rx));
    async fn on_upgrade(websocket: WebSocket, tx: Sender<SplitSink<WebSocket, Message>>) {
        let (ws_tx, _) = websocket.split();
        tx.send(ws_tx).await.unwrap();
    }

    let dev_ws = warp::path(WS_URL)
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone();
            ws.on_upgrade(move |websocket| on_upgrade(websocket, tx))
        });
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(index_html));
    let other = warp::any().and(warp::fs::dir(dist));

    let url = format!("http://{host}:{port}");
    info!("listening on {url}");
    let mut addr = expect!(
        format!("{host}:{port}").to_socket_addrs(),
        "invalid socket addr"
    );
    warp::serve(dev_ws.or(index).or(other))
        .run(addr.next().unwrap())
        .await;
}

fn start_watch_thread(
    interval: u64,
    package: Option<String>,
    manifest_dir: PathBuf,
    rebuild_tx: Sender<()>,
) {
    use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

    macro_rules! monitor {
        ($path:expr, $interval:expr, $tx:expr) => {
            let mut w = expect!(
                watcher($tx, Duration::from_millis($interval)),
                "unable to start watcher"
            );
            expect!(
                w.watch($path, RecursiveMode::Recursive),
                "unable to start watcher"
            );
        };
    }
    let (tx, rx) = channel();
    info!("watch repo every {interval} milliseconds");
    monitor!(&manifest_dir.join("src"), interval, tx.clone());
    monitor!(&manifest_dir.join("build.rs"), interval, tx.clone());
    monitor!(&manifest_dir.join("Cargo.toml"), interval, tx);
    let mut count = 1;
    loop {
        match rx.recv() {
            Ok(evt) => {
                if matches!(
                    evt,
                    DebouncedEvent::Create(_)
                        | DebouncedEvent::Remove(_)
                        | DebouncedEvent::Rename(_, _)
                        | DebouncedEvent::Write(_)
                ) {
                    info!("🚧 {count} rebuilding...");
                    build_wasm(package.clone(), false);
                    run_wasm_bindgen(package.clone(), false);
                    if let Err(e) = rebuild_tx.blocking_send(()) {
                        error!("{:}", e);
                    }
                    count += 1;
                    info!("🚧 {count} rebuild done.");
                }
            }
            Err(e) => {
                error!("watch error: {:?}", e);
                break;
            }
        }
    }
}

fn run_cargo_new(path: &PathBuf, name: Option<String>) {
    let mut cmd = process::Command::new("cargo");
    cmd.arg("new").arg(path).arg("--lib");
    if let Some(name) = name {
        cmd.arg("--name");
        cmd.arg(name);
    }
    expect!(cmd.output(), "failed to create project");
}

fn init_project(path: &Path) {
    config_cargo(path);
    add_build_rs(path);
}

fn add_build_rs(path: &Path) {
    let file_path = path.join("build.rs");
    if file_path.exists() {
        error!("build.rs already exists");
        process::exit(1);
    }

    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(include_bytes!("../build.tpl.rs")).unwrap();
}

fn add_lib_rs(path: &Path) {
    let file_path = path.join("src").join("build.rs");
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(include_bytes!("../lib.tpl.rs")).unwrap();
}

fn config_cargo(path: &Path) {
    use toml_edit::{table, value, Document, Item};
    let cargo_path = path.join("Cargo.toml");
    if !cargo_path.exists() {
        error!("missing Cargo.toml file");
        process::exit(1);
    }
    let mut file = std::fs::File::options()
        .write(true)
        .read(true)
        .open(cargo_path)
        .unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut cargo: Document = expect!(content.parse(), "invalid Cargo.toml file");
    if !cargo.contains_key("lib") {
        let mut lib = toml_edit::Table::new();
        lib.insert("crate-type", toml_edit::array());
        cargo["lib"] = toml_edit::Item::Table(lib);
    }
    let crate_type = &cargo["lib"]["crate-type"].as_array();
    if let Some(arr) = crate_type {
        if arr.into_iter().any(|item| item.as_str() == Some("bin")) {
            error!("bin type crate is not compatible with wasm target, expect cdylib and rlib");
            process::exit(1);
        }
    }
    let mut arr = toml_edit::Array::new();
    arr.push("cdylib");
    arr.push("rlib");
    cargo["lib"]["crate-type"] = value(arr);
    if cargo["dependencies"].as_table().is_none() {
        cargo["dependencies"] = table();
    }
    let deps = cargo["dependencies"].as_table_mut().unwrap();
    if !deps.contains_key("rtml") {
        deps.insert("rtml", Item::Value(rtml_dep()));
    }
    if !deps.contains_key("rtml-project") {
        deps.insert("rtml-project", Item::Value(rtml_project_dep()));
    }
    if !deps.contains_key("wasm-bindgen") {
        deps.insert("wasm-bindgen", Item::Value(wasm_bindgen_dep()));
    }
    if !deps.contains_key("web-sys") {
        deps.insert("web-sys", Item::Value(web_sys_dep()));
    }
    if !deps.contains_key("js-sys") {
        deps.insert("js-sys", Item::Value(js_sys_dep()));
    }
    if !deps.contains_key("tracing") {
        deps.insert("tracing", Item::Value(tracing_dep()));
    }
    if !deps.contains_key("tracing-wasm") {
        deps.insert("tracing-wasm", Item::Value(tracing_wasm_dep()));
    }
    if !cargo.contains_key("build-dependencies") {
        cargo["build-dependencies"] = table();
    }
    let build_deps = cargo["build-dependencies"].as_table_mut().unwrap();
    if !build_deps.contains_key("rtml") {
        build_deps.insert("rtml", Item::Value(rtml_dep()));
    }
    if !build_deps.contains_key("rtml-project") {
        build_deps.insert("rtml-project", Item::Value(rtml_project_dep()));
    }

    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write_all(cargo.to_string().as_bytes()).unwrap();
}

fn rtml_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "git",
        toml_edit::Value::String(toml_edit::Formatted::new(
            "https://github.com/PrivateRookie/rtml".into(),
        )),
    );
    toml_edit::Value::InlineTable(dep)
}
fn rtml_project_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "git",
        toml_edit::Value::String(toml_edit::Formatted::new(
            "https://github.com/PrivateRookie/rtml".into(),
        )),
    );
    toml_edit::Value::InlineTable(dep)
}
fn web_sys_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "version",
        toml_edit::Value::String(toml_edit::Formatted::new("0.3".into())),
    );
    let mut feats = toml_edit::Array::new();
    feats.push("Document");
    feats.push("Element");
    feats.push("HtmlElement");
    feats.push("Node");
    feats.push("Window");
    dep.insert("features", toml_edit::Value::Array(feats));
    toml_edit::Value::InlineTable(dep)
}

fn js_sys_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "version",
        toml_edit::Value::String(toml_edit::Formatted::new("0.3".into())),
    );
    toml_edit::Value::InlineTable(dep)
}

fn wasm_bindgen_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "version",
        toml_edit::Value::String(toml_edit::Formatted::new("0.2".into())),
    );
    toml_edit::Value::InlineTable(dep)
}
fn tracing_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "version",
        toml_edit::Value::String(toml_edit::Formatted::new("0.1".into())),
    );
    toml_edit::Value::InlineTable(dep)
}
fn tracing_wasm_dep() -> toml_edit::Value {
    let mut dep = toml_edit::InlineTable::new();
    dep.insert(
        "version",
        toml_edit::Value::String(toml_edit::Formatted::new("0.2".into())),
    );
    toml_edit::Value::InlineTable(dep)
}
