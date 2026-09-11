use std::{fs, io, path::{Path, PathBuf}};

#[derive(Clone, Debug)]
pub struct FileStore {
    root: PathBuf,
}

impl FileStore {
    pub fn open(root: impl AsRef<Path>) -> io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn put(&self, key: &str, value: &[u8]) -> io::Result<()> {
        fs::write(self.root.join(key), value)
    }

    pub fn get(&self, key: &str) -> io::Result<Vec<u8>> {
        fs::read(self.root.join(key))
    }
}
