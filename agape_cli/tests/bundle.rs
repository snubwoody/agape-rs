use agape_cli::parse::CargoMetadata;
use agape_cli::{Result, bundle_app};
use std::fs;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{TempDir, tempdir};

/// Set up a cargo project for testing.
fn setup_cargo() -> Result<(TempDir, PathBuf)> {
    let dir = tempdir()?;
    let app_path = dir.path().join("test-app");
    let output = Command::new("cargo")
        .args(["new", "test-app"])
        .current_dir(dir.path())
        .output()?;

    if !output.status.success() {
        panic!("Failed to setup rust project: {:?}", output.stderr);
    }

    Ok((dir, app_path))
}

#[test]
fn get_default_bin_name() -> Result<()> {
    let (_dir, app_path) = setup_cargo()?;
    let metadata = CargoMetadata::from_path(&app_path)?;
    let bin = metadata.get_default_bin()?;
    assert_eq!(bin, "test-app");
    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn get_release_bin() -> Result<()> {
    let (_dir, app_path) = setup_cargo()?;
    let metadata = CargoMetadata::from_path(&app_path)?;
    let bin = metadata.get_release_bin(&metadata.get_default_bin()?)?;
    let mut release_name = metadata.target_directory().join("release").join("test-app");
    release_name.set_extension("exe");
    assert_eq!(bin, release_name);
    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))]
fn get_release_bin() -> Result<()> {
    let (_dir, app_path) = setup_cargo()?;
    let metadata = CargoMetadata::from_path(&app_path)?;
    let bin = metadata.get_release_bin(&metadata.get_default_bin()?)?;
    let release_name = metadata.target_directory().join("release").join("test-app");
    assert_eq!(bin, release_name);
    Ok(())
}

// TODO: test workspace
// TODO: test multiple and different bin
#[test]
#[cfg(target_os = "windows")]
fn bundle_release_bin() -> Result<()> {
    let (_dir, app) = setup_cargo()?;
    bundle_app(&app, None)?;
    let mut dist = app.join("dist").join("test-app");
    dist.set_extension("exe");
    assert!(fs::exists(dist)?);
    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))]
fn bundle_release_bin() -> Result<()> {
    let (_dir, app) = setup_cargo()?;
    bundle_app(&app, None)?;
    let dist = app.join("dist").join("test-app");
    assert!(fs::exists(dist)?);
    Ok(())
}

#[test]
fn bundle_assets() -> Result<()> {
    let (_dir, app) = setup_cargo()?;
    let asset_dir = app.join("assets");

    fs::create_dir(&asset_dir)?;
    fs::write(asset_dir.join("index.html"), "")?;
    fs::write(asset_dir.join("img.png"), "")?;
    bundle_app(&app, None)?;
    let dist_assets = app.join("dist").join("assets");

    let entries = read_dir(&dist_assets)?
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].path(), dist_assets.join("img.png"));
    assert_eq!(entries[1].path(), dist_assets.join("index.html"));
    Ok(())
}
