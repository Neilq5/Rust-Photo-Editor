use image::DynamicImage;

pub fn load_image(path: &str) -> Result<DynamicImage, image::ImageError> {
    image::open(path)
}
