use std::path::Path;
use image::{ImageBuffer, ImageError, Rgba};

pub fn load_texture<P: AsRef<Path>>(p: P) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    Ok(image::open(p)?.to_rgba8())
}