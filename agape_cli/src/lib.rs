mod error;
pub mod parse;

pub use crate::error::Result;
use crate::parse::CargoMetadata;
use clap::{Parser, Subcommand};
pub use error::CliError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

// TODO print bundle size
#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[command()]
    /// Run your application.
    Run {
        #[arg(short, long)]
        project: Option<String>,
    },
    // TODO: add architecture
    #[command()]
    /// Bundle your application.
    Bundle {
        /// The project to run
        #[arg(short, long)]
        project: Option<String>,
    },
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Run { project } => run_app(&project),
        Command::Bundle { project } => bundle_app(".", project)?,
    }

    Ok(())
}

fn run_app(project: &Option<String>) {
    let mut args = vec!["run"];
    if let Some(project) = project {
        args.push("-p");
        args.push(project);
    }
    std::process::Command::new("cargo")
        .args(&args)
        .status()
        .expect("Failed to run app");
}

pub fn bundle_app(path: impl AsRef<Path>, project: Option<String>) -> Result<()> {
    let metadata = CargoMetadata::from_path(&path)?;
    let mut bin = match project {
        Some(p) => p,
        None => metadata.get_default_bin()?,
    };

    println!("Bundling project");

    let path = path.as_ref();
    let dist = path.join("dist");
    fs::create_dir_all(&dist)?;

    let output = std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .stdout(std::process::Stdio::piped())
        .current_dir(&path)
        .status()?;

    if !output.success() {
        panic!("Failed to build project");
    }

    #[cfg(target_os = "windows")]
    bin.push_str(".exe");
    fs::copy(metadata.get_release_bin(&bin).unwrap(), dist.join(bin))?;

    copy_assets(path.join("assets"), path.join("dist").join("assets"))?;
    println!("Bundled assets");
    Ok(())
}

/// Recursively copy the assets from the src directory into
/// the destination directory.
fn copy_assets<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q) -> Result<()> {
    if !fs::exists(&src)? {
        return Ok(());
    }
    let dest = dest.as_ref();
    fs::create_dir_all(&dest)?;

    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_assets(entry.path(), &dest.join(entry.file_name()))?;
        } else {
            let dist_path = dest.join(entry.file_name());
            fs::copy(entry.path(), dist_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn copy_assets_to_dist() -> Result<()> {
        let temp = tempdir()?;
        let asset_dir = temp.path().join("assets");
        let dist_dir = temp.path().join("dist");

        fs::create_dir(&asset_dir)?;
        fs::write(asset_dir.join("index.html"), "")?;
        fs::write(asset_dir.join("img.png"), "")?;
        copy_assets(asset_dir, &dist_dir)?;

        let entries = fs::read_dir(&dist_dir)?
            .map(|e| e.unwrap())
            .collect::<Vec<_>>();

        assert_eq!(entries.len(), 2);

        Ok(())
    }

    #[test]
    fn copy_assets_recursively() -> Result<()> {
        let temp = tempdir()?;
        let asset_dir = temp.path().join("assets");
        let dist_dir = temp.path().join("dist");

        fs::create_dir_all(&asset_dir.join("images"))?;
        fs::write(asset_dir.join("index.html"), "")?;
        fs::write(asset_dir.join("images").join("img.png"), "")?;
        copy_assets(asset_dir, &dist_dir)?;

        let assets = fs::read_dir(&dist_dir)?
            .map(|e| e.unwrap())
            .collect::<Vec<_>>();

        assert_eq!(assets.len(), 2);
        assert_eq!(assets[0].path(), dist_dir.join("images"));
        assert_eq!(assets[1].path(), dist_dir.join("index.html"));

        let images = fs::read_dir(&assets[0].path())?
            .map(|e| e.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].path(), dist_dir.join("images").join("img.png"));

        Ok(())
    }

    #[test]
    fn skip_non_existent_asset_dir() -> Result<()> {
        let temp = tempdir()?;
        let asset_dir = temp.path().join("assets");
        let dist_dir = temp.path().join("dist");
        copy_assets(asset_dir, dist_dir)?;
        Ok(())
    }
}
