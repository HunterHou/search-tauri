use image::GenericImageView;
use reqwest;
use std::fs::write;
use std::path::Path;

// #[tauri::command]

#[warn(unused_must_use)]
pub fn download(url: &str, folder: &str, name: &str, ext: &str, crop: bool) -> Result<(), String> {
    let response = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let fname = format!("{}/{}.{}", folder, name, ext);
    let dest = Path::new(&fname);
    println!("downloading {} to {}", url, &fname);
    let _ = write(dest, response);
    if crop {
        let mut img = image::open(&fname).unwrap();
        let (width, height) = img.dimensions();
        println!("Image dimensions: {:?}", img.dimensions());
        let cropped = img.crop(width/2+34, 0, width, height);
        let png= format!("{}/{}.{}", folder, name, "png");
        let _ = cropped.save(png);
    }
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
