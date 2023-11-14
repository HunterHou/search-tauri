use reqwest;
use std::fs::write;
use std::path::Path;

#[tauri::command]
pub async fn download(url: &str, folder: &str, name: &str, ext: &str) -> Result<(), String> {
    let response = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    let fname = format!("{}/{}.{}", folder, name, ext);
    let dest = Path::new(&fname);
    let _ = write(dest, response);
    //
    Ok(())
}

// use sysinfo::{DiskExt, System, SystemExt};

// #[tauri::command]
// pub async fn disk_free_size(path: String) -> u64 {
//     let sys = System::new_all();
//     for disk in sys.disks() {
//         if Path::new(&path).starts_with(disk.mount_point()) {
//             return disk.available_space();
//         }
//     }
//     return 0;
// }
