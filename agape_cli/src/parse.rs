use crate::{CliError, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct CargoMetadata {
    packages: Vec<CargoPackage>,
    target_directory: PathBuf,
}

// TODO: handle workspace
impl CargoMetadata {
    pub fn target_directory(&self) -> &Path {
        &self.target_directory
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let output = Command::new("cargo")
            .args(["metadata", "--format-version", "1", "--no-deps"])
            .current_dir(&path)
            .output()?;

        let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)?;
        Ok(metadata)
    }

    /// Get the name of the default binary.
    pub fn get_default_bin(&self) -> Result<String> {
        if self.packages.len() > 1 {
            return Err(CliError::generic(
                "Failed to find a default package, please specific which package to use",
            ));
        }

        if self.packages.is_empty() {
            return Err(CliError::generic("No packages were found"));
        }

        let package = self.packages.first().unwrap();
        let default_target = package
            .targets
            .iter()
            .find(|t| t.kind.contains(&"bin".to_string()))
            .unwrap();

        Ok(default_target.name.clone())
    }

    /// Get the path of the release binary.
    pub fn get_release_bin(&self, name: &str) -> Result<PathBuf> {
        let path = self.target_directory.join("release").join(name);
        let path = path.clone();
        if cfg!(target_os = "windows") {
            let mut path = path.clone();
            path.set_extension("exe");
        }

        Ok(path)
    }
}

#[derive(Debug, Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    targets: Vec<CargoTarget>,
}

#[derive(Debug, Deserialize)]
pub struct CargoTarget {
    kind: Vec<String>,
    name: String,
}
