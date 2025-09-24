
// Config File generation and parsing functions:

use std::{fs::File, path::PathBuf, io::Write};

use serde::Deserialize;




pub fn generate_config(home_path: &PathBuf){
  // Create a overgrowth directory in the home directory if it doesn't exist
  let overgrowth_path = home_path.join(".overgrowth");
  if !overgrowth_path.exists() {
    std::fs::create_dir_all(overgrowth_path).unwrap();
  }
  // Create a scripts directory in the overgrowth directory if it doesn't exist
  let scripts_path = home_path.join(".overgrowth/scripts");
  if !scripts_path.exists() {
    std::fs::create_dir_all(scripts_path).unwrap();
  }
  // Check if the config file exists, if it does exit this function
  let config_path = home_path.join(".overgrowth/config.toml");
  if config_path.exists() {
    println!("Config file already exists");
    return;
  }
  // Create a config file with the default values
  let mut config_file = File::create(config_path).unwrap();
  let config_toml = 
  r#"
  # The path to the directory where the icons will be stored (root is home dir of user, note that changing this will not change the location of the config file, only the location of the icons, if the directory does not exist it will be created)
  icon_dir = ".overgrowth/icons"
"#;
  config_file.write_all(config_toml.as_bytes()).unwrap();
}


// Parse the config file and return a Config object
pub fn parse_config(home_path: &PathBuf) -> Config {
  let config_path = home_path.join(".overgrowth/config.toml");
  let config_toml = std::fs::read_to_string(config_path).unwrap();
  let config_toml : Config = toml::from_str(&config_toml).unwrap();

  config_toml
}

#[derive(Deserialize)] pub struct Config{
  pub icon_dir: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use tempfile::TempDir;

  #[test]
  fn test_generate_config_creates_directories() {
    let temp_dir = TempDir::new().unwrap();
    let home_path = temp_dir.path().to_path_buf();
    
    generate_config(&home_path);
    
    // Check if directories were created
    assert!(home_path.join(".overgrowth").exists());
    assert!(home_path.join(".overgrowth/scripts").exists());
    assert!(home_path.join(".overgrowth/config.toml").exists());
  }

  #[test]
  fn test_generate_config_does_not_overwrite_existing() {
    let temp_dir = TempDir::new().unwrap();
    let home_path = temp_dir.path().to_path_buf();
    
    // Create config first time
    generate_config(&home_path);
    let config_path = home_path.join(".overgrowth/config.toml");
    
    // Modify the file
    fs::write(&config_path, "custom content").unwrap();
    
    // Try to generate again
    generate_config(&home_path);
    
    // Should not overwrite
    let content = fs::read_to_string(&config_path).unwrap();
    assert_eq!(content, "custom content");
  }

  #[test]
  fn test_parse_config_returns_correct_structure() {
    let temp_dir = TempDir::new().unwrap();
    let home_path = temp_dir.path().to_path_buf();
    
    generate_config(&home_path);
    let config = parse_config(&home_path);
    
    assert_eq!(config.icon_dir, ".overgrowth/icons");
  }

  #[test]
  fn test_parse_config_with_custom_values() {
    let temp_dir = TempDir::new().unwrap();
    let home_path = temp_dir.path().to_path_buf();
    let overgrowth_path = home_path.join(".overgrowth");
    fs::create_dir_all(&overgrowth_path).unwrap();
    
    let config_content = r#"
    icon_dir = "custom/icon/path"
    "#;
    
    let config_path = overgrowth_path.join("config.toml");
    fs::write(&config_path, config_content).unwrap();
    
    let config = parse_config(&home_path);
    assert_eq!(config.icon_dir, "custom/icon/path");
  }
}
