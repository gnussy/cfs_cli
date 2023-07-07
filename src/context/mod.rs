use std::sync::Arc;

use cfs::partition::CfsPartition;
use tokio::sync::Mutex;

pub struct CFSContext {
  pub current_inode: usize,
  pub cfs: Option<Arc<Mutex<CfsPartition>>>,
}

impl CFSContext {
  pub fn new() -> Self {
    Self {
      current_inode: cfs::ROOT_INODE,
      cfs: None,
    }
  }

  pub fn mkcfs(&mut self, file: std::fs::File, block_size: usize) -> Result<(), String> {
    let mut cfs_partition = cfs::partition::CfsPartition::new(file, block_size as u64)
      .map_err(|_| "Failed to create partition".to_string())?;
    cfs_partition
      .setup_root_dir()
      .map_err(|_| "Failed to setup root directory".to_string())?;

    self.cfs = Some(Arc::new(Mutex::new(cfs_partition)));

    Ok(())
  }

  pub fn cfs(&self) -> Result<Arc<Mutex<CfsPartition>>, String> {
    match &self.cfs {
      Some(cfs) => Ok(cfs.clone()),
      None => Err("No CFS image loaded".to_string()),
    }
  }

  pub fn current_inode(&self) -> usize {
    self.current_inode
  }

  pub fn load_cfs(&mut self, image: std::fs::File) -> Result<(), String> {
    self.cfs = Some(Arc::new(Mutex::new(
      CfsPartition::try_from(image).map_err(|_| "Failed to load CFS image".to_string())?,
    )));

    Ok(())
  }
}

impl Default for CFSContext {
  fn default() -> Self {
    Self::new()
  }
}
