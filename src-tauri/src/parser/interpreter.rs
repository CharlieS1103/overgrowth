use crate::app_structs::mac_app::MacApplication;
use super::parser::{ImageMetadata, MetadataField, Action, parse};
use std::error::Error;
use std::fs;
use std::time::SystemTime;
use image::DynamicImage;

pub struct Interpreter {
    pub processed_apps: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            processed_apps: Vec::new(),
        }
    }

    /// Execute a script on a list of applications
    pub fn execute_script(&mut self, script_content: &str, mac_apps: &[MacApplication]) -> Result<Vec<MacApplication>, Box<dyn Error + '_>> {
        let (metadata, _) = parse(script_content)?;
        let mut matching_apps = Vec::new();

        for app in mac_apps {
            if self.app_matches_conditions(app, &metadata.fields)? {
                self.apply_actions_to_app(app, &metadata.actions)?;
                matching_apps.push(app.clone());
                self.processed_apps.push(app.name.clone());
            }
        }

        Ok(matching_apps)
    }

    /// Check if an app matches all the conditions in the script
    fn app_matches_conditions(&self, app: &MacApplication, fields: &[MetadataField]) -> Result<bool, Box<dyn Error>> {
        for field in fields {
            match field {
                MetadataField::Type(_, expected_type) => {
                    // For now, we'll just check the app name contains the type
                    if !app.name.to_lowercase().contains(&expected_type.to_lowercase()) {
                        return Ok(false);
                    }
                }
                MetadataField::Author(_, expected_author) => {
                    // Check if app bundle info contains author info (simplified)
                    if !app.name.to_lowercase().contains(&expected_author.to_lowercase()) {
                        return Ok(false);
                    }
                }
                MetadataField::Date(_, days_threshold, _op, _comparison) => {
                    // Check app modification time
                    let metadata = fs::metadata(&app.path)?;
                    let modified = metadata.modified()?;
                    let now = SystemTime::now();
                    let duration = now.duration_since(modified)?;
                    let days_old = duration.as_secs() / 86400;
                    
                    if days_old < *days_threshold as u64 {
                        return Ok(false);
                    }
                }
                MetadataField::Other(key, value) => {
                    // Generic field matching
                    println!("Checking other field: {} = {}", key, value);
                }
            }
        }
        Ok(true)
    }

    /// Apply actions to an app that matches the conditions
    fn apply_actions_to_app(&self, app: &MacApplication, actions: &[Action]) -> Result<(), Box<dyn Error>> {
        for action in actions {
            match action {
                Action::ChangeColor(color) => {
                    println!("Would change color of {} to {}", app.name, color);
                    // TODO: Implement color changing logic
                }
                Action::AddOverlay(overlay_file) => {
                    println!("Would add overlay {} to {}", overlay_file, app.name);
                    // TODO: Implement overlay logic
                    self.apply_overlay_to_app(app, overlay_file)?;
                }
            }
        }
        Ok(())
    }

    /// Apply an overlay to an application's icon
    fn apply_overlay_to_app(&self, app: &MacApplication, overlay_file: &str) -> Result<(), Box<dyn Error>> {
        // This is where we'd implement the overlay logic
        // For now, just print what we would do
        println!("Applying overlay '{}' to app: {}", overlay_file, app.name);
        println!("App icon path: {}", app.icn_path.display());
        
        // TODO: 
        // 1. Load the base icon from app.icn_path
        // 2. Load the overlay from assets directory
        // 3. Combine them using add_overlay function
        // 4. Save the result back to the icon path
        
        Ok(())
    }

    /// Load and execute all scripts from the scripts directory
    pub fn load_and_execute_all_scripts(&mut self, scripts_dir: &std::path::Path, mac_apps: &[MacApplication]) -> Result<(), Box<dyn Error>> {
        if !scripts_dir.exists() {
            println!("Scripts directory doesn't exist: {}", scripts_dir.display());
            return Ok(());
        }

        for entry in fs::read_dir(scripts_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "ov") {
                println!("Executing script: {}", path.display());
                let script_content = fs::read_to_string(&path)?;
                
                match self.execute_script(&script_content, mac_apps) {
                    Ok(processed) => {
                        println!("Successfully processed {} apps with script {}", processed.len(), path.file_name().unwrap().to_string_lossy());
                    }
                    Err(e) => {
                        eprintln!("Error executing script {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_interpreter_creation() {
        let interpreter = Interpreter::new();
        assert_eq!(interpreter.processed_apps.len(), 0);
    }

    #[test]
    fn test_script_parsing() {
        let mut interpreter = Interpreter::new();
        let script = r#"where metadata field Type is "Test"{ add overlay file: "test.png" }"#;
        
        // Create a test app
        let test_app = MacApplication {
            path: PathBuf::from("/Applications/TestApp.app"),
            name: "TestApp.app".to_string(),
            icns: "AppIcon.icns".to_string(),
            icn_path: PathBuf::from("/Applications/TestApp.app/Contents/Resources/AppIcon.icns"),
        };

        let result = interpreter.execute_script(script, &[test_app]);
        assert!(result.is_ok());
    }
}