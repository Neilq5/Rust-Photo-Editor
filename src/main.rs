fn main() {
    match photo_editor::load_image("assets/test.jpg") {
        Ok(image) => println!(
            "Loaded {} byte image with dimensions: {}x{}",
            image.as_bytes().len(),
            image.width(),
            image.height()
        ),
        Err(e) => println!("Failed to load image: {}", e),
    }
}
