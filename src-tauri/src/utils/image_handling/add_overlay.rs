pub fn add_overlay(mut base_image : image::DynamicImage, overlay_image : &image::DynamicImage) -> image::DynamicImage {
    image::imageops::overlay(&mut base_image, overlay_image,0 ,0 );
    return base_image;
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageBuffer, Rgba};

    #[test]
    fn test_add_overlay_combines_images() {
        // Create a simple 10x10 red base image
        let base_img = DynamicImage::ImageRgba8(
            ImageBuffer::from_fn(10, 10, |_, _| Rgba([255, 0, 0, 255]))
        );

        // Create a simple 5x5 blue overlay
        let overlay_img = DynamicImage::ImageRgba8(
            ImageBuffer::from_fn(5, 5, |_, _| Rgba([0, 0, 255, 255]))
        );

        let result = add_overlay(base_img, &overlay_img);

        // Check that we got an image back
        assert_eq!(result.width(), 10);
        assert_eq!(result.height(), 10);

        // Check that the overlay was applied at position (0,0)
        let result_rgba = result.to_rgba8();
        let pixel_at_0_0 = result_rgba.get_pixel(0, 0);
        // Should be blue (the overlay color) at position 0,0
        assert_eq!(pixel_at_0_0, &Rgba([0, 0, 255, 255]));

        // Check that areas outside overlay remain red
        let pixel_at_9_9 = result_rgba.get_pixel(9, 9);
        assert_eq!(pixel_at_9_9, &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_add_overlay_preserves_base_size() {
        let base_img = DynamicImage::ImageRgba8(
            ImageBuffer::from_fn(100, 50, |_, _| Rgba([255, 255, 255, 255]))
        );

        let overlay_img = DynamicImage::ImageRgba8(
            ImageBuffer::from_fn(20, 20, |_, _| Rgba([0, 0, 0, 255]))
        );

        let result = add_overlay(base_img, &overlay_img);

        // Result should maintain the base image dimensions
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }
}