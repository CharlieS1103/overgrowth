extern crate serde;

extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MacApplication {
    pub path: PathBuf,
    pub name: String,
    pub icns: Vec<String>,
}

fn toml_file_to_vec(filepath: &str) -> Vec<MacApplication> {
    let mut file = File::open(filepath).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let toml_data: toml::Value = toml::from_str(&contents).expect("Failed to parse TOML data");
    let mac_apps = toml_data["mac_apps"].as_table().expect("Invalid data format");
    let mut mac_app_vec = Vec::new();
    for (_, app) in mac_apps {
        let app_data = app.as_table().expect("Invalid data format");
        let mac_app = MacApplication {
            path: PathBuf::from(app_data["path"].as_str().expect("Invalid data format").to_string()),
            name: app_data["name"].as_str().expect("Invalid data format").to_string(),
            icns: app_data["icns"].as_array().expect("Invalid data format").iter().map(|x| x.as_str().expect("Invalid data format").to_string()).collect(),
        };
        mac_app_vec.push(mac_app);
    }
    mac_app_vec
}
