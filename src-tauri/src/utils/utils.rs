use std::{error::Error, fs::{self, read_dir}, path::PathBuf};

use crate::{config::parse_config, get_home_dir};

// If a user wants to uninstall overgrowth we should use the backed up icns files in the specified directory to restore the icons to their original state
pub fn uninstall_overgrowth() -> Result<(), Box<dyn Error>> {
    // THIS FUNCTION IS SUPER SCARY TO TEST!! TODO: WRITE UNIT TESTS FOR THIS FUNCTION
    let config = parse_config(&get_home_dir().unwrap());
    let home_dir = get_home_dir().unwrap();
    let icon_dir = home_dir.join(PathBuf::from(&config.icon_dir));
    let app_dir = home_dir.join("Applications");
  
    for entry in read_dir(icon_dir)? {
      let entry = entry?;
      let path = entry.path();
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let app_name = file_name.split(".icns").collect::<Vec<&str>>()[0];
      let app_path = app_dir.join(app_name);
      let app_icon_path = app_path.join(file_name);
      let original_icon_path = path;
  
      if app_icon_path.exists() {
        fs::remove_file(app_icon_path.clone())?;
      }
  
      fs::copy(original_icon_path, app_icon_path)?;
    }
  
    Ok(())
  }