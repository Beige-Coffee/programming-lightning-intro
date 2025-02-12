#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use std::fs;
use std::path::PathBuf;
use std::io::{self, Read, Write};

pub struct FileStore {
  data_dir: PathBuf
}

impl FileStore {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    fn get_file_path(&self, primary_namespace: &str, secondary_namespace: &str, key: &str) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(primary_namespace);
        if !secondary_namespace.is_empty() {
            path.push(secondary_namespace);
        }
        path.push(key);
        path
    }

    fn read_file(&self, path: PathBuf) -> lightning::io::Result<Vec<u8>> {
        let mut file = fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    fn write_file(&self, path: PathBuf, data: &[u8]) -> lightning::io::Result<()> {
        // assumes directory exists
        let mut file = fs::File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}

impl FileStore {
    pub fn read(&self, primary: &str, secondary: &str, key: &str) -> lightning::io::Result<Vec<u8>> {
        let path = self.get_file_path(primary, secondary, key);
        self.read_file(path)
    }

    pub fn write(&self, primary: &str, secondary: &str, key: &str, data:&[u8]) -> lightning::io::Result<()> {
        let path = self.get_file_path(primary, secondary, key);
        let mut file = fs::File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}
