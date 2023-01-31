extern crate photon_rs;
use photon_rs::multiple::watermark;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;
use std::path::Path;

fn get_center(x: u32, y: u32) -> (u32, u32) {
    (x / 2, y / 2)
}

fn get_img_out_path(img_in_path: &str) -> String {
    let path = Path::new(img_in_path);
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap();
    let extension = path.extension().unwrap();
    let out_path: String = format!(
        "{}/{}_watermarked.{}",
        parent.to_string_lossy(),
        file_stem.to_string_lossy(),
        extension.to_string_lossy()
    );
    out_path
}

pub fn water_mark_centered(
    img_in_path: &str,
    stamp_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    //Open the stamp image
    let stamp: PhotonImage = open_image(stamp_path)?;
    // Open the image (a PhotonImage is returned)
    let mut img: PhotonImage = open_image(&img_in_path)?;
    //Get center of the image
    let (x, y) = get_center(img.get_width(), img.get_height());
    //Get center of the stamp
    let (x_stamp, y_stamp) = get_center(stamp.get_width(), stamp.get_height());
    // Watermark the image
    watermark(&mut img, &stamp, x - x_stamp, y - y_stamp);
    let img_out_pth: &str = &get_img_out_path(img_in_path);
    // Write file to filesystem.
    save_image(img, &img_out_pth);
    Ok(img_out_pth.to_string())
}
