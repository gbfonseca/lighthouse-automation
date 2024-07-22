//! # Screen Information
//!
//! `screen` is a collection of functions used to get screen related information.

pub use image::{DynamicImage, GenericImageView, Rgba};
pub use screenshots::DisplayInfo;

/// Returns the width and height of primary screen.
pub fn size() -> (u16, u16) {
    let displays = DisplayInfo::all().expect("Unable to get displays");
    let primary = displays
        .iter()
        .find(|display| display.is_primary == true)
        .expect("Unable to find primary display");
    return (primary.width as u16, primary.height as u16);
}

/// Verifies if specified x & y coordinates are present on primary screen.
pub fn on_screen(x: u16, y: u16) -> bool {
    let display = size();
    return x <= display.0 && y <= display.1;
}

/// Returns screenshot of the primary screen.
pub fn screenshot(buffer: Vec<u8>) -> DynamicImage {
    return image::load_from_memory(&buffer).unwrap();
}

/// Saves the provided screenshot to a path with the specified filename and extension.
pub fn printscreen(screenshot: DynamicImage, path: &str) {
    screenshot
        .save(path)
        .expect("Error saving file to specified path, filename, and/or extension.");
}

/// Locates the first pixel color similar to the one specified and returns its coordinate.
pub fn locate_pixel(image: Vec<u8>, pixel: Rgba<u8>) -> Option<(u16, u16)> {
    let ss = screenshot(image);
    let (x, y) = ss.dimensions();

    let x: u16 = u16::try_from(x).unwrap();
    let y: u16 = u16::try_from(y).unwrap();

    let display = (x, y);

    let text = String::from("Analyze page load");

    for y in 0..display.1 {
        for x in 0..display.0 {
            let bounding_box = (x, y, text.len() as u16 * 10, 20);
            println!("BOUNDY => {:?}", bounding_box);
            return Some(bounding_box);
            // if ss.get_pixel(x.into(), y.into()) == pixel {
            //     return Some((x, y));
            // }
        }
    }

    return None;
}

/// Get the pixel color on x, y coordinate.
pub fn get_pixel(image: Vec<u8>, x: u16, y: u16) -> Rgba<u8> {
    let ss = screenshot(image);
    return ss.get_pixel(x.into(), y.into());
}

// Locates the first object similar to the one provided
// fn locate_img() {}

// Locates all objects similar to the image provided
// fn locate_all_img() {}

// Locates the first object similar to the one provided and returns its center x & y
// fn locate_img_center() {}
