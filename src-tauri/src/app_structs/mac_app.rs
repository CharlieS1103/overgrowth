extern crate serde;

extern crate toml;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct MacApplication {
    pub path: PathBuf,
    pub name: String,
    pub icns: String,
    pub icn_path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_application_creation() {
        let app = MacApplication {
            path: PathBuf::from("/Applications/TestApp.app"),
            name: "TestApp.app".to_string(),
            icns: "AppIcon.icns".to_string(),
            icn_path: PathBuf::from("/Applications/TestApp.app/Contents/Resources/AppIcon.icns"),
        };

        assert_eq!(app.name, "TestApp.app");
        assert_eq!(app.icns, "AppIcon.icns");
        assert!(app.path.to_string_lossy().contains("TestApp.app"));
    }

    #[test]
    fn test_mac_application_serialization() {
        let app = MacApplication {
            path: PathBuf::from("/Applications/TestApp.app"),
            name: "TestApp.app".to_string(),
            icns: "AppIcon.icns".to_string(),
            icn_path: PathBuf::from("/Applications/TestApp.app/Contents/Resources/AppIcon.icns"),
        };

        // Test serialization to TOML
        let serialized = toml::to_string(&app).unwrap();
        assert!(serialized.contains("TestApp.app"));
        assert!(serialized.contains("AppIcon.icns"));
    }

    #[test]
    fn test_mac_application_deserialization() {
        let toml_str = r#"
path = "/Applications/TestApp.app"
name = "TestApp.app"
icns = "AppIcon.icns"
icn_path = "/Applications/TestApp.app/Contents/Resources/AppIcon.icns"
        "#;

        let app: MacApplication = toml::from_str(toml_str).unwrap();
        assert_eq!(app.name, "TestApp.app");
        assert_eq!(app.icns, "AppIcon.icns");
    }
}

