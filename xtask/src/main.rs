use std::process::Command;

use clap::{ArgMatches, Parser, Subcommand};

/// Helper program to start ic10emu and website.
///
/// Can be invoked as `cargo xtask <command>`
#[derive(Debug, Parser)]
#[command(bin_name = "cargo xtask")]
struct Args {
    /// Package manager to use
    #[arg(long, global = true, default_value = "pnpm")]
    manager: String,
    /// wasm-pack executable
    #[arg(long, global = true, default_value = "wasm-pack")]
    wasm_pack: String,
    #[command(subcommand)]
    task: Task,
}

const PACKAGES: &[&str] = &["ic10lsp_wasm", "ic10emu_wasm"];

#[derive(Debug, Subcommand)]
enum Task {
    /// Build the packages
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Packages to build
        #[arg(long, short = 'p', default_values = PACKAGES)]
        packages: Vec<String>,
        /// Additional arguments to pass to wasm-pack, use another `--` to pass to cargo build
        #[arg(last = true, default_values = ["--","-q"])]
        rest: Vec<std::ffi::OsString>,
    },
    /// Start the server
    ///
    /// This does not build the packages, use `build` first
    Start {},
}

#[derive(thiserror::Error)]
enum Error {
    #[error("building package {0} failed. Command: `{1}` Status code {2}")]
    BuildFailed(String, String, std::process::ExitStatus),
    #[error("failed to run command `{0}`")]
    Command(String, #[source] std::io::Error),
    #[error("IO failed {0}")]
    Io(String, #[source] std::io::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::error::Error;
        use std::fmt::*;
        write!(f, "Error: {}", self)?;
        let mut err: &dyn Error = self;
        while let Some(cause) = err.source() {
            write!(f, "\nCaused by: ")?;
            Display::fmt(&cause, f)?;
            err = cause;
        }
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let workspace = {
        let out = Command::new("cargo")
            .arg("metadata")
            .arg("--no-deps")
            .arg("--format-version=1")
            .output()
            .map_err(|e| Error::Command("cargo metadata".to_string(), e))?;
        let s = std::str::from_utf8(&out.stdout).unwrap();
        let Some((_, s)) = s.split_once(r#"workspace_root":""#) else {
            panic!("couldn't find workspace root");
        };
        let Some((path, _)) = s.split_once("\",") else {
            panic!("couldn't find workspace root");
        };
        std::path::PathBuf::from(path)
    };
    match &args.task {
        Task::Build {
            release,
            packages,
            rest,
        } => {
            build(&args, packages, *release, &workspace, rest)?;
        }
        Task::Start {} => {
            eprintln!("Starting server");
            let mut cmd = Command::new(&args.manager);
            cmd.current_dir(&workspace.join("www"));
            cmd.args(["run", "start"]).status().map_err(|e| {
                Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e)
            })?;
        }
    }
    Ok(())
}

fn build<P: AsRef<std::ffi::OsStr> + std::fmt::Debug + std::fmt::Display>(
    args: &Args,
    packages: &[P],
    release: bool,
    workspace: &std::path::Path,
    rest: &[std::ffi::OsString],
) -> Result<(), Error> {
    if packages.is_empty() {
        panic!("no package(s) specified")
    }
    eprintln!("Building packages: {:?}, release: {}", packages, release);
    for package in packages {
        eprintln!("Building package: {}", package);
        let mut cmd = Command::new(&args.wasm_pack);
        cmd.current_dir(workspace);
        cmd.arg("build");
        if release {
            cmd.arg("--release");
        } else {
            cmd.arg("--dev");
        }
        cmd.arg(package);
        let status = cmd
            .status()
            .map_err(|e| Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e))?;
        if status.success() {
            eprintln!("{} built successfully", package);
        } else {
            return Err(Error::BuildFailed(
                package.to_string(),
                format!("{cmd:?}"),
                status,
            ));
        }
    }
    Ok(())
}
