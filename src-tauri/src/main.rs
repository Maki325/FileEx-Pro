// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod drive;

use drive::{Drive, DriveType};
use std::ffi::CString;
use tauri::Manager;
use winapi;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::fileapi::{
  GetDiskFreeSpaceA, GetDriveTypeA, GetLogicalDrives, GetVolumeInformationA,
};

unsafe fn get_vol_name(volume_letter: &CString, drive: char) -> String {
  const VOLUME_NAME_SIZE: usize = MAX_PATH + 1;
  let mut volume_name_buf: [u8; VOLUME_NAME_SIZE] = [0; VOLUME_NAME_SIZE];

  let success = GetVolumeInformationA(
    volume_letter.as_ptr(),
    volume_name_buf.as_mut_ptr() as *mut i8,
    VOLUME_NAME_SIZE as u32,
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    0,
  );

  if success == 0 {
    return format!("Local Drive ({}:)", drive);
  }

  if volume_name_buf[0] == 0 {
    return format!("Local Drive ({}:)", drive);
  }

  let end_index = volume_name_buf
    .iter()
    .enumerate()
    .find(|(_, c)| **c == 0)
    .expect("Must exist!")
    .0;

  match CString::from_vec_unchecked(volume_name_buf[0..end_index].to_vec()).into_string() {
    Ok(volume_name) => {
      if volume_name.len() == 0 {
        return format!("Local Drive ({}:)", drive);
      } else {
        return format!("{} ({}:)", volume_name, drive);
      }
    }
    Err(_) => {
      return format!("Local Drive ({}:)", drive);
    }
  }
}

unsafe fn get_win32_drives() -> Vec<Drive> {
  let mut logical_drives: Vec<Drive> = Vec::with_capacity(5);
  let mut bitfield = GetLogicalDrives();
  let mut drive = 'A';

  while bitfield != 0 {
    if bitfield & 1 == 1 {
      let strfulldl = drive.to_string() + ":\\";
      let volume_letter = CString::new(strfulldl.clone()).unwrap();
      let drive_type = GetDriveTypeA(volume_letter.as_ptr()).into();

      match drive_type {
        DriveType::Fixed | DriveType::Removable => {}
        _ => continue,
      };

      let mut drive = Drive::new(get_vol_name(&volume_letter, drive), drive_type);

      GetDiskFreeSpaceA(
        volume_letter.as_ptr(),
        &mut drive.sectors_per_cluster,
        &mut drive.bytes_per_sector,
        &mut drive.number_of_free_clusters,
        &mut drive.number_of_clusters,
      );

      logical_drives.push(drive);
    }
    drive = std::char::from_u32((drive as u32) + 1).unwrap();
    bitfield >>= 1;
  }
  logical_drives
}

#[tauri::command]
fn list_drives() -> Vec<Drive> {
  return unsafe { get_win32_drives() };
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(debug_assertions)] // only include this code on debug builds
      {
        let window = app.get_window("main").unwrap();

        let monitors = window.available_monitors()?;
        let monitor = monitors.get(0).ok_or(tauri::Error::CreateWindow)?;
        let pos = monitor.position();
        window.set_position(tauri::PhysicalPosition { x: pos.x, y: 0 })?;

        window.set_always_on_top(true)?;

        window.open_devtools();
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![list_drives])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
