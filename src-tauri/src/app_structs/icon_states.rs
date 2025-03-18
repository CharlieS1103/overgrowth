use std::{fs::{/*self,*/ File}, path::PathBuf, error::Error, io::{Write/* , BufReader, Read*/}/* ,  time::SystemTime, collections::HashMap*/};
use mac_app::MacApplication;
// use toml::{Value, value::Table};
// use serde::Deserialize;

use super::mac_app;


pub fn generate_toml_file(home_dir : &PathBuf, apps : &Vec<MacApplication>) -> Result<(), Box<dyn Error>> {
   // Create a new toml file
    let mut file = File::create(home_dir.join(".overgrowth/icon_states.toml"))?;
    println!("{:?}", home_dir.join(".overgrowth/icon_states.toml"));
    // Write the toml file containing all the mac app icns 
    let mut toml = String::new();
    for app in apps {
        // Helper apps often contain parenthesis in their name SHOULD INDICATE THIS TO USERS ON UI INTERFACE SOMEWHERE
        if app.path.with_extension("").file_name().unwrap().to_str().unwrap().replace(" ", "").contains("(") {
            continue;
        }
        // Get rid of any duplicate apps using path as a unique identifier
        if toml.contains(&app.path.to_str().unwrap()) {
            continue;
        }
        if app.icns.len() == 0 || app.icns == "" {
            continue;
        }
        // Seperate each application  into it's own section
        toml.push_str(&format!("[[apps]]\n"));//, app.path.with_extension("").file_name().unwrap().to_str().unwrap().replace(" ", "")));
        toml.push_str(&format!("path = \"{}\"\n", app.path.to_str().unwrap()));
        toml.push_str(&format!("name = \"{}\"\n", app.name));
        toml.push_str(&format!("icn_path = \"{}\"\n", app.icn_path.to_str().unwrap()));
    
        toml.push_str(&format!("icns = \"{}\"\n", app.icns.as_str()));

    }
    file.write_all(toml.as_bytes())?;
    Ok(())
}

