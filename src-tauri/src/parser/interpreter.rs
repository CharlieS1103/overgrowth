use icns::Image;

use super::parser::image_metadata;
use super::parser::ImageMetadata;
use super::parser::MetadataField;
use super::parser::Action;
use super::parser::parse;

pub struct Interpreter {
    state: std::collections::HashMap<String, String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            state: std::collections::HashMap::new(),
        }
    }

    pub fn interpret(&mut self, metadata: ImageMetadata) {
    
        for field in metadata.fields {
            
            match field {
                MetadataField::Type(mut key, value) => {
                    key = "FIELD:".to_owned() + &key.to_uppercase();
                    println!("Setting type {} to {}", key, value);
                    self.state.insert(key, value);
                }
                MetadataField::Author(mut key, value) => {
                    key =  "FIELD:".to_owned() + &key.to_uppercase();
                    println!("Setting author {} to {}", key, value);
                    self.state.insert(key, value);
                }
                MetadataField::Other(mut key, value) => {
                    key =  "FIELD:".to_owned() + &key.to_uppercase();
                    println!("Setting other {} to {}", key, value);
                    self.state.insert(key, value);
                }
            }
        }

        for action in metadata.actions {
            match action {
                Action::ChangeColor(color) => {
                    println!("Changing color to {}", color);
                }
                Action::AddOverlay(overlay) => {
                    println!("Adding overlay {}", overlay);
                }
            }
        }
    }
}

// WRite a test for the interpreter
#[cfg(test)]
mod tests {
    use combine::Parser;
    
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut interpreter = Interpreter::new();
        let metadata = image_metadata().parse(r#"where metadata field "Type" is "Landscape"{ change color to "blue" }"#).unwrap().0;
        interpreter.interpret(metadata);
        // I dont like seeing FIELD:TYPE, need to figure out a hashmap structure for comparison operators and the such
        assert_eq!(interpreter.state.get("FIELD:TYPE"), Some(&"Landscape".to_string()));
       
    }
}
