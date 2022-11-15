use std::{fs::File, path::PathBuf, error::Error, io::Write};
use mac_app::MacApplication;
use super::mac_app;


pub fn generate_toml_file(home_dir : &PathBuf, icns : &Vec<MacApplication>) -> Result<(), Box<dyn Error>> {
   // Create a new toml file
    let mut file = File::create(home_dir.join(".overgrowth/icon_states.toml"))?;
    println!("{:?}", home_dir.join(".overgrowth/icon_states.toml"));
    // Write the toml file containing all the mac app icns 
    let mut toml = String::new();
    toml.push_str("[mac_apps]\n");
    for app in icns {
        // Seperate each application  into it's own section
        toml.push_str(&format!("[[{}]]\n", app.path.with_extension("").file_name().unwrap().to_str().unwrap()));
        toml.push_str(&format!("path = \"{}\"\n", app.path.to_str().unwrap()));
        toml.push_str(&format!("name = \"{}\"\n", app.name));
        toml.push_str(&format!("icns = [\n"));
        for icn in &app.icns {
            toml.push_str(&format!("\"{}\",\n", icn.to_str().unwrap()));
        }
        toml.push_str(&format!("]\n"));
    }
    file.write_all(toml.as_bytes())?;
    Ok(())
}

