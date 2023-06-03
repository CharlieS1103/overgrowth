use icns::{IconFamily, IconType};
use image::{io::Reader as ImageReader, ImageBuffer};


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



fn convert_pngs_to_icns(png_dir: &Path, icns_path: &Path) /*-> Result<(), Box<dyn Error>>*/ {
      // Placeholder but this function is really important but also hard to figure out
}
 