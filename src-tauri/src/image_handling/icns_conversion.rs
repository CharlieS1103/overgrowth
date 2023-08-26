use icns::{IconFamily, IconType};
use image::{io::Reader as ImageReader, ImageBuffer};
use std::{error::Error, fs::{self, File}, io::BufReader, path::PathBuf};


fn convert_icns_to_png(icns_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Read the icns file
    let file = BufReader::new(File::open(icns_path)?);
    let icon_family = IconFamily::read(file)?;

    // Create a directory based on the icn file name
    let png_dir = icns_path.with_file_name(icns_path.file_name().unwrap());
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



fn convert_pngs_to_icns(png_dir: &PathBuf, icns_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Create a new icon family
    let mut icon_family = IconFamily::new();

    // Loop through all the png files in the directory
    for entry in fs::read_dir(png_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Read the png file
        let file = BufReader::new(File::open(&path)?);
        let image = ImageReader::new(file).decode()?;

        // Get the icon type from the file name
        let icon_type = match path.file_name().unwrap().to_str().unwrap() {
            "icon_16x16.png" => IconType::Icon16x16,
            _ => continue,
        };

        // Add the icon to the icon family
        icon_family.add_icon_with_type(&image.to_rgba(), icon_type)?;
    }

    // Write the icns file
    let file = File::create(icns_path)?;
    icon_family.write(file)?;

    Ok(())
}
      
 