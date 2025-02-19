#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use std::fs;
use std::path::PathBuf;
use std::io::{self, Read, Write};
use crate::ch2_setup::channel_exercises::ChannelMonitor;
use lightning::chain::transaction::OutPoint;

pub struct FileStore {
  data_dir: PathBuf
}

pub enum ChannelMonitorUpdateStatus {
  Completed,
  UnrecoverableError
}

/// The primary namespace under which [`ChannelMonitor`]s will be persisted.
pub const CHANNEL_MONITOR_PERSISTENCE_PRIMARY_NAMESPACE: &str = "monitors";
/// The secondary namespace under which [`ChannelMonitor`]s will be persisted.
pub const CHANNEL_MONITOR_PERSISTENCE_SECONDARY_NAMESPACE: &str = "";

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

    pub fn persist_channel(
        &self, funding_txo: OutPoint, monitor: ChannelMonitor,
    ) -> ChannelMonitorUpdateStatus {
        let key = format!("{}_{}", funding_txo.txid.to_string(), funding_txo.index);
        match self.write(
            CHANNEL_MONITOR_PERSISTENCE_PRIMARY_NAMESPACE,
            CHANNEL_MONITOR_PERSISTENCE_SECONDARY_NAMESPACE,
            &key,
            &monitor.encode(),
        ) {
            Ok(()) => ChannelMonitorUpdateStatus::Completed,
            Err(_) => ChannelMonitorUpdateStatus::UnrecoverableError,
        }
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
