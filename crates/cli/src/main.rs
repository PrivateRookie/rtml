use std::{
    io::{Read, Seek, Write},
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    process,
    sync::mpsc::{channel, Sender},
    time::Duration,
};

use clap::Parser;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

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
        #[clap(long, default_value = "127.0.0.1")]
        host: String,
        /// listen port
        #[clap(long, default_value = "9001")]
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

            if let Some(interval) = watch {
                let meta = metadata::MetaData::load_current_meta();
                let manifest = match package.as_ref() {
                    Some(package) => {
                        let p = meta.packages.iter().find(|p| p.name == *package).unwrap();
                        p.manifest_path.parent().unwrap().into()
                    }
                    None => meta.workspace_root,
                };
                std::thread::spawn(move || start_watch_thread(interval, package, manifest));
            }
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(start_http_server(dist, host, port))
        }
        Commands::Build { package, release } => {
            build_wasm(package.clone(), release);
            run_wasm_bindgen(package, release);
        }
    }
}

macro_rules! error {
    ($($arg:tt),*) => {
        use std::io::Write;
        let mut stderr = StandardStream::stderr(termcolor::ColorChoice::Auto);
        if let Err(e) = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))) {
            println!("unable to set term color {:?}", e)
        };
        if let Err(e) = writeln!(&mut stderr, $($arg),*) {
            println!("print msg failed {:?}", e)
        }
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
        cmd.arg("----remove-producers-section");
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

    std::fs::copy(
        &src_wasm,
        target_dir.join(src_wasm.file_name().unwrap_or_default()),
    )
    .expect("copy wasm file failed");

    for entry in std::fs::read_dir(get_out_dir(release)).unwrap().flatten() {
        let src = entry.path();
        let dest: PathBuf = src.components().skip(5).collect();
        std::fs::copy(src, target_dir.join(dest)).unwrap();
    }
    target_dir
}

fn read_package() -> String {
    let mut file = std::fs::File::open("Cargo.toml").expect("no Cargo.toml found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read Cargo.toml");
    let cargo = content
        .parse::<toml_edit::Document>()
        .expect("invalid Cargo.toml");
    cargo["package"]["name"]
        .as_str()
        .expect("empty package name")
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

async fn start_http_server(dist: PathBuf, host: String, port: u16) {
    use warp::Filter;

    let index_html = dist.join("index.html");
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(index_html));
    let other = warp::any().and(warp::fs::dir(dist));
    let url = format!("http://{host}:{port}");
    println!("listening on {url}");
    let mut addr = format!("{host}:{port}")
        .to_socket_addrs()
        .expect("invalid socket addr");
    warp::serve(index.or(other)).run(addr.next().unwrap()).await;
}

fn start_watch_thread(interval: u64, package: Option<String>, manifest_dir: PathBuf) {
    use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

    fn start_watcher(path: &Path, interval: u64, tx: Sender<DebouncedEvent>) {
        let mut watcher =
            watcher(tx, Duration::from_millis(interval)).expect("unable to start watcher");
        watcher
            .watch(path, RecursiveMode::Recursive)
            .expect("unable to start watcher");
    }
    let (tx, rx) = channel();
    println!("watch repo  every {interval} milliseconds");
    start_watcher(&manifest_dir.join("src"), interval, tx.clone());
    start_watcher(&manifest_dir.join("build.rs"), interval, tx.clone());
    start_watcher(&manifest_dir.join("Cargo.toml"), interval, tx.clone());
    let mut count = 1;
    println!("waiting");
    loop {
        match rx.recv() {
            Ok(evt) => {
                dbg!(&evt);
                if matches!(
                    evt,
                    DebouncedEvent::Create(_)
                        | DebouncedEvent::Remove(_)
                        | DebouncedEvent::Rename(_, _)
                        | DebouncedEvent::Write(_)
                ) {
                    println!("🚧 {count} rebuilding...");
                    build_wasm(package.clone(), false);
                    run_wasm_bindgen(package.clone(), false);
                    count += 1;
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
    cmd.output().expect("failed to create project");
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
    file.write_all(r#"use rtml::{attr, tags::*};
use rtml_project::{add_file, get_pkg_id};

fn index() -> Html {
    let pkg = get_pkg_id();
    html((
        attr! {lang="zh-cn"},
        (
            head((
                meta(attr! { charset="utf-8" }),
                meta(attr! { name = "viewport", content="width=device-width, initial-scale=1" }),
                meta(
                    attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" },
                ),
                title(&pkg),
                script((
                    attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
            )),
            body(()),
        ),
    ))
}

fn main() -> std::io::Result<()> {
    add_file("index.html", index().to_string().as_bytes())
}"#.as_bytes()).unwrap();
}

fn add_lib_rs(path: &Path) {
    let file_path = path.join("src").join("build.rs");
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(
        r#"use rtml::tags::*;
use rtml::{attr, mount_body, style};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let page = div((
        style! {text-align: "center"},
        (
            h1("hello World!"),
            br(()),
            strong(a((
                attr! {href="http://"},
                "Power By https://github.com/PrivateRookie/rtml",
            ))),
        ),
    ));

    mount_body(page).unwrap();
}"#
        .as_bytes(),
    )
    .unwrap();
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
    let mut cargo: Document = content.parse().expect("invalid Cargo.toml file");
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
        deps.insert("rtml", Item::Table(rtml_dep()));
    }
    if !deps.contains_key("wasm-bindgen") {
        deps.insert("wasm-bindgen", Item::Table(wasm_bindgen_dep()));
    }
    if !deps.contains_key("tracing") {
        deps.insert("tracing", Item::Table(tracing_dep()));
    }
    if !deps.contains_key("tracing-wasm") {
        deps.insert("tracing-wasm", Item::Table(tracing_wasm_dep()));
    }
    if !cargo.contains_key("build-dependencies") {
        cargo["build-dependencies"] = table();
    }
    let build_deps = cargo["build-dependencies"].as_table_mut().unwrap();
    if !build_deps.contains_key("rtml") {
        build_deps.insert("rtml", Item::Table(rtml_dep()));
    }
    if !build_deps.contains_key("rtml-project") {
        build_deps.insert("rtml-project", Item::Table(rtml_project_dep()));
    }

    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write_all(cargo.to_string().as_bytes()).unwrap();
}

fn rtml_dep() -> toml_edit::Table {
    use toml_edit::value;
    let mut dep = toml_edit::Table::new();
    dep.insert("git", value("https://github.com/PrivateRookie/rtml"));
    dep
}
fn rtml_project_dep() -> toml_edit::Table {
    use toml_edit::value;
    let mut dep = toml_edit::Table::new();
    dep.insert("git", value("https://github.com/PrivateRookie/rtml"));
    dep
}
fn wasm_bindgen_dep() -> toml_edit::Table {
    use toml_edit::value;
    let mut dep = toml_edit::Table::new();
    dep.insert("version", value("0.2"));
    dep
}
fn tracing_dep() -> toml_edit::Table {
    use toml_edit::value;
    let mut dep = toml_edit::Table::new();
    dep.insert("version", value("0.1"));
    dep
}
fn tracing_wasm_dep() -> toml_edit::Table {
    use toml_edit::value;
    let mut dep = toml_edit::Table::new();
    dep.insert("version", value("0.2"));
    dep
}
