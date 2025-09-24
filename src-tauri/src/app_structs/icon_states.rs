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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_generate_toml_file_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().to_path_buf();
        
        // Create .overgrowth directory
        let overgrowth_dir = home_dir.join(".overgrowth");
        fs::create_dir_all(&overgrowth_dir).unwrap();

        let apps = vec![
            MacApplication {
                path: PathBuf::from("/Applications/TestApp1.app"),
                name: "TestApp1.app".to_string(),
                icns: "AppIcon.icns".to_string(),
                icn_path: PathBuf::from("/Applications/TestApp1.app/Contents/Resources/AppIcon.icns"),
            },
            MacApplication {
                path: PathBuf::from("/Applications/TestApp2.app"),
                name: "TestApp2.app".to_string(),
                icns: "Icon.icns".to_string(),
                icn_path: PathBuf::from("/Applications/TestApp2.app/Contents/Resources/Icon.icns"),
            }
        ];

        let result = generate_toml_file(&home_dir, &apps);
        assert!(result.is_ok());

        // Check if file was created
        let toml_path = home_dir.join(".overgrowth/icon_states.toml");
        assert!(toml_path.exists());

        // Check file contents
        let content = fs::read_to_string(&toml_path).unwrap();
        assert!(content.contains("TestApp1.app"));
        assert!(content.contains("TestApp2.app"));
        assert!(content.contains("[[apps]]"));
    }

    #[test]
    fn test_generate_toml_file_filters_helper_apps() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().to_path_buf();
        
        // Create .overgrowth directory
        let overgrowth_dir = home_dir.join(".overgrowth");
        fs::create_dir_all(&overgrowth_dir).unwrap();

        let apps = vec![
            MacApplication {
                path: PathBuf::from("/Applications/TestApp.app"),
                name: "TestApp.app".to_string(),
                icns: "AppIcon.icns".to_string(),
                icn_path: PathBuf::from("/Applications/TestApp.app/Contents/Resources/AppIcon.icns"),
            },
            MacApplication {
                path: PathBuf::from("/Applications/Helper (App).app"),
                name: "Helper (App).app".to_string(),
                icns: "Helper.icns".to_string(),
                icn_path: PathBuf::from("/Applications/Helper (App).app/Contents/Resources/Helper.icns"),
            }
        ];

        let result = generate_toml_file(&home_dir, &apps);
        assert!(result.is_ok());

        let toml_path = home_dir.join(".overgrowth/icon_states.toml");
        let content = fs::read_to_string(&toml_path).unwrap();
        
        // Should contain regular app but not helper app (with parentheses)
        assert!(content.contains("TestApp.app"));
        assert!(!content.contains("Helper (App).app"));
    }

    #[test]
    fn test_generate_toml_file_filters_empty_icns() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().to_path_buf();
        
        // Create .overgrowth directory
        let overgrowth_dir = home_dir.join(".overgrowth");
        fs::create_dir_all(&overgrowth_dir).unwrap();

        let apps = vec![
            MacApplication {
                path: PathBuf::from("/Applications/ValidApp.app"),
                name: "ValidApp.app".to_string(),
                icns: "AppIcon.icns".to_string(),
                icn_path: PathBuf::from("/Applications/ValidApp.app/Contents/Resources/AppIcon.icns"),
            },
            MacApplication {
                path: PathBuf::from("/Applications/InvalidApp.app"),
                name: "InvalidApp.app".to_string(),
                icns: "".to_string(), // Empty icns
                icn_path: PathBuf::from("/Applications/InvalidApp.app/Contents/Resources/"),
            }
        ];

        let result = generate_toml_file(&home_dir, &apps);
        assert!(result.is_ok());

        let toml_path = home_dir.join(".overgrowth/icon_states.toml");
        let content = fs::read_to_string(&toml_path).unwrap();
        
        // Should contain valid app but not invalid app (with empty icns)
        assert!(content.contains("ValidApp.app"));
        assert!(!content.contains("InvalidApp.app"));
    }
}

