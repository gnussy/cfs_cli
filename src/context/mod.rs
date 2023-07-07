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

  pub fn get_cfs(&self) -> Result<Arc<Mutex<CfsPartition>>, String> {
    match &self.cfs {
      Some(cfs) => Ok(cfs.clone()),
      None => Err("No CFS image loaded".to_string()),
    }
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
