pub fn add_overlay(mut base_image : image::DynamicImage, overlay_image : &image::DynamicImage) -> image::DynamicImage {
    image::imageops::overlay(&mut base_image, overlay_image,0 ,0 );
    return base_image;
  }