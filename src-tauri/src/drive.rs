use serde::Serialize;
use winapi;
use winapi::um::winbase::{
  DRIVE_CDROM, DRIVE_FIXED, DRIVE_NO_ROOT_DIR, DRIVE_RAMDISK, DRIVE_REMOTE, DRIVE_REMOVABLE,
  DRIVE_UNKNOWN,
};

#[derive(Debug, Serialize)]
pub enum DriveType {
  Unknown,
  NoRootDir,
  Removable,
  Fixed,
  Remote,
  CDRom,
  RamDisk,
}

impl From<u32> for DriveType {
  fn from(value: u32) -> Self {
    match value {
      DRIVE_UNKNOWN => DriveType::Unknown,
      DRIVE_NO_ROOT_DIR => DriveType::NoRootDir,
      DRIVE_REMOVABLE => DriveType::Removable,
      DRIVE_FIXED => DriveType::Fixed,
      DRIVE_REMOTE => DriveType::Remote,
      DRIVE_CDROM => DriveType::CDRom,
      DRIVE_RAMDISK => DriveType::RamDisk,
      _ => panic!("Unknown drive type: {}", value),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct Drive {
  pub name: String,
  pub drive_type: DriveType,
  pub sectors_per_cluster: u32,
  pub bytes_per_sector: u32,
  pub number_of_free_clusters: u32,
  pub number_of_clusters: u32,
}

impl Drive {
  pub fn new(name: String, drive_type: DriveType) -> Drive {
    Drive {
      name,
      drive_type,
      sectors_per_cluster: 0,
      bytes_per_sector: 0,
      number_of_free_clusters: 0,
      number_of_clusters: 0,
    }
  }
}
