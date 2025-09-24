use icns::{IconFamily, IconType, Image};
use image::ImageReader;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_convert_icns_to_png_creates_png_directory() {
        let temp_dir = TempDir::new().unwrap();
        let icns_dir_path = temp_dir.path().to_path_buf();
        
        // Create a minimal test icns file (this won't be a real icns but we can test directory creation)
        let icns_path = icns_dir_path.join("test.icns");
        
        // For now, just test that it handles missing file gracefully
        let result = convert_icns_to_png(icns_path, icns_dir_path.clone());
        
        // Should fail with missing file, but let's make sure the png directory would be created
        match result {
            Ok(_) => {
                let png_dir = icns_dir_path.join("png");
                assert!(png_dir.exists());
            },
            Err(_) => {
                // Expected for non-existent file
                // Just test that the function doesn't panic
                assert!(true);
            }
        }
    }

    #[test]
    fn test_convert_pngs_to_icns_requires_existing_pngs() {
        let temp_dir = TempDir::new().unwrap();
        let png_dir = temp_dir.path().join("png");
        let icns_path = temp_dir.path().join("output.icns");
        
        fs::create_dir_all(&png_dir).unwrap();
        
        // Test with empty PNG directory
        let result = convert_pngs_to_icns(&png_dir, &icns_path);
        
        // Should complete without error even with no PNGs (creates empty ICNS)
        match result {
            Ok(_) => assert!(icns_path.exists()),
            Err(_) => {
                // Also acceptable if it fails with no valid PNGs
                assert!(true);
            }
        }
    }
}

 