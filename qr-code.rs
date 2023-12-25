extern crate qrcode;
extern crate image;

use qrcode::QrCode;
use qrcode::render::svg;
use image::{GenericImageView, DynamicImage, Rgba};
use std::path::PathBuf;

fn main() {
    // Your data for the QR code
    let data = "Hello, QR Code with Image!";

    // Generate QR code
    let code = match QrCode::new(data) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error generating QR code: {}", e);
            std::process::exit(1);
        }
    };
    let image = code.render::<svg::Color>().build();

    // Load image to be placed in the center
    let center_image_path = PathBuf::from("path/to/your/image.png");
    let center_image = match image::open(&center_image_path) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error loading center image: {}", e);
            std::process::exit(1);
        }
    };

    // Calculate the position to center the image on the QR code
    let x_offset = (image.width() - center_image.width()) / 2;
    let y_offset = (image.height() - center_image.height()) / 2;

    // Combine QR code and image
    let mut final_image = DynamicImage::new_rgba8(image.width(), image.height());
    final_image.copy_from(&image, 0, 0).expect("Error copying QR code image");
    final_image
        .copy_from(&center_image, x_offset as u32, y_offset as u32)
        .expect("Error combining QR code and center image");

    // Save the final image
    let output_path = PathBuf::from("path/to/your/output/image.png");
    final_image.save(&output_path).expect("Error saving final image");
    println!("QR code with center image saved to: {}", output_path.display());
}
