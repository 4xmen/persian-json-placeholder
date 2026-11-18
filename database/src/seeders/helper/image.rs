use anyhow::{Context, Result};
use image::{DynamicImage, imageops::FilterType};
use std::path::Path;

pub fn generate_category_images(
    source: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    slug: &str,
) -> Result<(String, String)> {
    let source = source.as_ref();
    let output_dir = output_dir.as_ref();

    // Create output directories
    let thumb_dir = output_dir.join("thumbs");
    std::fs::create_dir_all(&thumb_dir)?;

    // Output paths
    let image_path = output_dir.join(format!("{slug}.webp"));
    let thumb_path = thumb_dir.join(format!("{slug}.webp"));

    // Skip processing if both images already exist
    if image_path.exists() && thumb_path.exists() {
        // println!("skip:{slug}");
        return Ok((
            image_path.to_string_lossy().into_owned(),
            thumb_path.to_string_lossy().into_owned(),
        ));
    }

    // Load original image
    let original = image::open(source)
        .with_context(|| format!("Failed to load image: {}", source.display()))?;

    // Convert original image to WebP
    save_webp(&original, &image_path, 85.0)?;

    // Crop center and resize to 800x800
    let thumb = original.resize_to_fill(800, 800, FilterType::Lanczos3);

    // Convert thumbnail to WebP
    save_webp(&thumb, &thumb_path, 80.0)?;

    Ok((
        image_path.to_string_lossy().into_owned(),
        thumb_path.to_string_lossy().into_owned(),
    ))
}

fn save_webp(image: &DynamicImage, path: &Path, quality: f32) -> Result<()> {
    let rgba = image.to_rgba8();

    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());

    let encoded = encoder.encode(quality);

    std::fs::write(path, &*encoded)?;

    Ok(())
}
