use icns::{IconFamily, IconType, Image};
use image::{io::Reader as ImageReader/*, ImageBuffer*/};
use std::{error::Error, fs::{self, File}, io::BufReader, path::PathBuf};


pub fn convert_icns_to_png(icns_path: PathBuf, icns_dir_path : PathBuf) -> Result<(), Box<dyn Error>> {
    // Read the icns file
    let file = BufReader::new(File::open(icns_path)?);
    let icon_family = IconFamily::read(file)?;


    let png_dir = icns_dir_path.join("png");
    fs::create_dir_all(&png_dir)?;
    // Loop through all the available icon types and convert them to png files
    for icon in icon_family.available_icons() {
        let image = match icon_family.get_icon_with_type(icon) {
            Ok(img) => img, 
            Err(_) => continue,
        };

        // Create the png file path
        let png_path = png_dir.join(format!("{:?}.png", icon));

        // Write the png file
        let file = File::create(&png_path)?;
        image.write_png(file)?;
    }

    Ok(())
}



pub fn convert_pngs_to_icns(png_dir: &PathBuf, icns_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Create a new icon family
    let mut icon_family = IconFamily::new();

    // Loop through all the png files in the directory
    for entry in fs::read_dir(png_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Read the png file
        let file = BufReader::new(File::open(&path)?);
        let image = ImageReader::new(file).decode()?;

        let icon_type = match path.file_name().unwrap().to_str().unwrap() {
            "RGB24_16x16.png" => IconType::RGB24_16x16,
            "Mask8_16x16.png" => IconType::Mask8_16x16,
            "RGB24_32x32.png" => IconType::RGB24_32x32,
            "Mask8_32x32.png" => IconType::Mask8_32x32,
            "RGB24_48x48.png" => IconType::RGB24_48x48,
            "Mask8_48x48.png" => IconType::Mask8_48x48,
            "RGB24_128x128.png" => IconType::RGB24_128x128,
            "Mask8_128x128.png" => IconType::Mask8_128x128,
            "RGBA32_16x16.png" => IconType::RGBA32_16x16,
            "RGBA32_16x16_2x.png" => IconType::RGBA32_16x16_2x,
            "RGBA32_32x32.png" => IconType::RGBA32_32x32,
            "RGBA32_32x32_2x.png" => IconType::RGBA32_32x32_2x,
            "RGBA32_64x64.png" => IconType::RGBA32_64x64,
            "RGBA32_128x128.png" => IconType::RGBA32_128x128,
            "RGBA32_128x128_2x.png" => IconType::RGBA32_128x128_2x,
            "RGBA32_256x256.png" => IconType::RGBA32_256x256,
            "RGBA32_256x256_2x.png" => IconType::RGBA32_256x256_2x,
            "RGBA32_512x512.png" => IconType::RGBA32_512x512,
            "RGBA32_512x512_2x.png" => IconType::RGBA32_512x512_2x,
            _ => continue,
        };

        let rgba_image = image.to_rgba8();
        let image = Image::from_data(icns::PixelFormat::RGBA, rgba_image.width(), rgba_image.height(), rgba_image.into_raw())?;
        icon_family.add_icon_with_type(&image, icon_type)?;
    }

    // Write the icns file
    let file = File::create(icns_path)?;
    icon_family.write(file)?;

    Ok(())
}

 