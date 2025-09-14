use bevy_ecs::prelude::Resource;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Resource)]
pub struct AssetManager {
    base: PathBuf,
}

impl AssetManager {
    pub fn new(base: impl AsRef<Path>) -> Self {
        let full_path = fs::canonicalize(&base).unwrap();
        info!("Initialised asset directory: {:?}", full_path);
        AssetManager {
            base: base.as_ref().to_path_buf(),
        }
    }

    pub fn get(&self, path: impl AsRef<Path>) -> crate::Result<Option<File>> {
        let path = self.base.join(path);
        match File::open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn get_asset() -> crate::Result<()> {
        let dir = TempDir::new().unwrap();
        File::create(dir.path().join("img.jpg"))?;
        let assets = AssetManager::new(dir.path());
        let asset = assets.get("img.jpg")?;
        assert!(asset.is_some());
        Ok(())
    }

    #[test]
    fn asset_not_found() -> crate::Result<()> {
        let dir = TempDir::new().unwrap();
        File::create(dir.path().join("img.jpg"))?;
        let assets = AssetManager::new(dir.path());
        let asset = assets.get("does-not-exist")?;
        assert!(asset.is_none());
        Ok(())
    }
}
