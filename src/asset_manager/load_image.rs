use image::{ImageBuffer, ImageError, Rgba};
use std::path::Path;

pub fn load_texture<P: AsRef<Path>>(p: P) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    Ok(image::open(p)?.to_rgba8())
}
