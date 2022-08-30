use icns::{IconFamily};

/* Overlays one image ontop of another image (NEEDS TO BE CONVERTED TO PNG BEFOREHAND)
* Parameters:
*  base_image: The image to be overlaid on top of another image
*  overlay_image: The image to be overlayed on top of the base image
* Returns: 
*  The image with the overlayed image on top of the base image
*/
fn add_overlay(mut base_image : image::DynamicImage, overlay_image : &image::DynamicImage) -> image::DynamicImage {
  image::imageops::overlay(&mut base_image, overlay_image,0 ,0 );
  return base_image;
}
// Make a function to convert a .icns file to a .png file
// TODO: Cleanup this entire function
fn convert_icns_to_png(icns_path: &PathBuf){
  let file = BufReader::new(File::open(icns_path).unwrap());
    let  icon_family = IconFamily::read(file).unwrap();
    let icon_type = icon_family.available_icons(); 
   // Loop thorugh all the available icon types and convert them to png files
    for icon in icon_type {
      // TODO: We need to figure out how to handle Jpeg 2000 icons 
      let image =  
      match icon_family.get_icon_with_type(icon){
        Ok(_) => icon_family.get_icon_with_type(icon).unwrap(),
        Err(_) => continue,
      };
      // Create a direcory based on the icn file name 
      let png_dir = icns_path.with_extension("");
      // check if the directory exists, if not create it
      if !png_dir.exists() {
        fs::create_dir_all(&png_dir).unwrap();
      }
      
      let icon_path = &png_dir.join(format!("{:?}.png", icon));
      let file = File::create(&icon_path).unwrap();
      image.write_png(file).unwrap();
      
    }
}