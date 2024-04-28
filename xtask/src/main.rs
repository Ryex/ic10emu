use std::process::{Command, ExitStatus};

use clap::{Parser, Subcommand};

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
const VALID_VERSION_TYPE: &[&str] = &["patch", "minor", "major"];

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
        // #[arg(last = true)]
        rest: Vec<std::ffi::OsString>,
    },
    /// Start the server
    ///
    /// This does not build the packages, use `build` first
    Start {},
    /// Runs production page under 'www/dist', Run `build` first.
    Deploy {},
    /// bump the cargo.toml and package,json versions
    Version {
        #[arg(last = true, default_value = "patch", value_parser = clap::builder::PossibleValuesParser::new(VALID_VERSION_TYPE))]
        version: String,
    },
    /// update changelog
    Changelog {},
}

#[derive(thiserror::Error)]
enum Error {
    #[error("building package {0} failed. Command: `{1}` Status code {2}")]
    BuildFailed(String, String, std::process::ExitStatus),
    #[error("failed to run command `{0}`")]
    Command(String, #[source] std::io::Error),
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

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
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
            pnpm_install(&args, &workspace)?;
            eprintln!("Starting server");
            let mut cmd = Command::new(&args.manager);
            cmd.current_dir(&workspace.join("www"));
            cmd.args(["run", "start"]).status().map_err(|e| {
                Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e)
            })?;
        }
        Task::Deploy {} => {
            pnpm_install(&args, &workspace)?;
            eprintln!("Production Build");
            let mut cmd = Command::new(&args.manager);
            cmd.current_dir(&workspace.join("www"));
            cmd.args(["run", "build"]).status().map_err(|e| {
                Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e)
            })?;
        }
        Task::Version { version } => {
            let mut cmd = Command::new("cargo");
            cmd.current_dir(&workspace);
            cmd.args(["set-version", "--bump", &version])
                .status()
                .map_err(|e| {
                    Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e)
                })?;
            let mut cmd = Command::new(&args.manager);
            cmd.current_dir(&workspace.join("www"));
            cmd.args(["version", &version]).status().map_err(|e| {
                Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e)
            })?;
        },
        Task::Changelog { } => {
            let mut cmd = Command::new("git-changelog");
            cmd.current_dir(&workspace);
            cmd.args([
                "-io", "CHANGELOG.md",
                "-t", "path:CHANGELOG.md.jinja",
                "--bump", VERSION.unwrap_or("auto"),
                "--parse-refs",
                "--trailers"
            ]).status().map_err(|e| {
                Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e)
            })?;
        },
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
        eprintln!(
            "Running command: {} build {} {} {}",
            &args.wasm_pack,
            if release { "--release" } else { "--dev" },
            package,
            rest.join(std::ffi::OsStr::new(" ")).to_string_lossy(),
        );
        let mut cmd = Command::new(&args.wasm_pack);
        cmd.current_dir(workspace);
        cmd.arg("build");
        if release {
            cmd.arg("--release");
        } else {
            cmd.arg("--dev");
        }
        cmd.arg(package);
        cmd.args(rest);
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

fn pnpm_install(args: &Args, workspace: &std::path::Path) -> Result<ExitStatus, Error> {
    eprintln!("Running `pnpm install`");
    let mut cmd = Command::new(&args.manager);
    cmd.current_dir(&workspace.join("www"));
    cmd.args(["install"])
        .status()
        .map_err(|e| Error::Command(format!("{}", cmd.get_program().to_string_lossy()), e))
}
