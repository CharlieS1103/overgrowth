
// Config File generation and parsing functions:

use std::{fs::File, path::PathBuf, io::Write};

use serde::Deserialize;




pub fn generate_config(home_path: &PathBuf){
  // Create a overgrowth directory in the home directory if it doesn't exist
  let overgrowth_path = home_path.join(".overgrowth");
  if !overgrowth_path.exists() {
    std::fs::create_dir_all(overgrowth_path).unwrap();
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
