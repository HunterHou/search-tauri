use std::fs::File;
use std::io::copy;
use std::path::Path;
use reqwest;

#[tauri::command]
pub async fn download(
    url: String,
    folder: String,
    name: String,
    ext: String,
) -> Result<(), String> {
    let response = reqwest::get(url);
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = Path::new(&folder).join(name);
        println!("will be located under: '{:?}'", fname);
        File::create(fname).unwrap();
    };
    let content = response.text();
    copy(&mut content.as_bytes(), &mut dest);
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
