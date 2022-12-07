use std::{fs::File, path::PathBuf, error::Error, io::{Write, BufReader, Read}, time::SystemTime};
use mac_app::MacApplication;
use toml::{Value, value::Table};

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

// Parses the toml file at the given path and returns a vector of MacApplication structs
fn parse_toml_file(toml_path: &PathBuf) -> Result<Vec<MacApplication>, Box<dyn Error>> {
    // Read the toml file and parse it into a toml value
    let toml_value = parse_toml_string(&mut BufReader::new(File::open(toml_path)?))?;

    // Get the mac_apps table from the toml value
    let mac_apps_table = get_mac_apps_table(&toml_value)?;

    // Convert the mac_apps table to a vector of mac_app_structs
    let mac_app_vec = convert_table_to_vec(&mac_apps_table)?;

    Ok(mac_app_vec)
}

// Parses the given toml string and returns the corresponding toml value
fn parse_toml_string(toml_string: &mut dyn Read) -> Result<Value, Box<dyn Error>> {
    let mut string = String::new();
    toml_string.read_to_string(&mut string)?;
    let toml_value = toml::from_str(&string)?;
    Ok(toml_value)
}

// Gets the mac_apps table from the given toml value
fn get_mac_apps_table(toml_value: &Value) -> Result<&Table, Box<dyn Error>> {
    let mac_apps_table = toml_value.get("mac_apps")
        .ok_or("`mac_apps` not found in toml file")?
        .as_table().ok_or("`mac_apps` is not a table")?;
    Ok(mac_apps_table)
}

// Converts the mac_apps table to a vector of mac_app_structs
fn convert_table_to_vec(mac_apps_table: &Table) -> Result<Vec<MacApplication>, Box<dyn Error>> {
    let mac_app_vec = mac_apps_table.into_iter().map(|(app_name, app_table)| {
        // Parse the values in the app_table and construct a MacApplication instance
        let path = app_table.get("path").ok_or("`path` not found in toml file")?
            .as_str().ok_or("`path` is not a string")?;
        let path = PathBuf::from(path);

        let name = app_table.get("name").ok_or("`name` not found in toml file")?
            .as_str().ok_or("`name` is not a string")?;

        let icns = app_table.get("icns").ok_or("`icns` not found in toml file")?
            .as_array().ok_or("`icns` is not an array")?
            .into_iter().map(|icn| {
                icn.as_str().ok_or("icn is not a string")
            }).collect::<Result<Vec<_>, _>>()?;
        

        // Use the From trait to construct a MacApplication instance from the parsed values
        let mac_app = MacApplication {
            path: path,
            icns: icns.into_iter().map(|icn| PathBuf::from(icn)).collect(),
            name: name.to_string(),
        };
        Ok(mac_app)
    }).collect::<Result<Vec<MacApplication>, _>>()?;

    Ok(mac_app_vec)
}
// Make a struct of the icon_states file

