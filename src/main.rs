extern crate photon_rs;
use photon_rs::multiple::watermark;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;

fn get_center(x: u32, y: u32) -> (u32, u32) {
    (x / 2, y / 2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Open the stamp image
    let stamp: PhotonImage = open_image("samples/stamp.png")?;
    // Open the image (a PhotonImage is returned)
    let mut img: PhotonImage = open_image("samples/1280px-EgliseDeClesles.jpg")?;
    //Get center of the image
    let (x, y) = get_center(img.get_width(), img.get_height());
    //Get center of the stamp
    let (x_stamp, y_stamp) = get_center(stamp.get_width(), stamp.get_height());
    // Watermark the image
    watermark(&mut img, &stamp, x - x_stamp, y - y_stamp);

    // Write file to filesystem.
    save_image(img, "sample/raw-1280px-EgliseDeClesles.jpg");

    Ok(())
}
