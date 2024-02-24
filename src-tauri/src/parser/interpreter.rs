use icns::Image;

use super::parser::image_metadata;
use super::parser::ImageMetadata;
use super::parser::MetadataField;
use super::parser::Action;
use super::parser::parse;
use super::parser::ComparisonOperator::{Equals, LessThan, GreaterThan, LessThanOrEqual, GreaterThanOrEqual, NotEquals};
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
                MetadataField::Date(mut key, value, op, value2) => {
                    key =  "FIELD:".to_owned() + &key.to_uppercase();
                    // Compare the dates, store key and true/false in state
                    // print op
                    println!("Operation: {:?}", op);
                    match op 
                    {
                        Equal => {
                            if value == value2 {
                                self.state.insert(key, "true".to_string());
                                

                            } else {
                                self.state.insert(key, "false".to_string());
                            }
                            
                        }
                        LessThan => {
                            if value < value2 {
                                self.state.insert(key, "true".to_string());
                            } else {
                                self.state.insert(key, "false".to_string());
                            }
                        }
                        GreaterThan => {
                            if value > value2 {
                                self.state.insert(key, "true".to_string());
                            } else {
                                self.state.insert(key, "false".to_string());
                            }
                        }
                        LessThanOrEqual => {
                            if value <= value2 {
                                self.state.insert(key, "true".to_string());
                            } else {
                                self.state.insert(key, "false".to_string());
                            }
                        }
                        GreaterThanOrEqual => {
                            if value >= value2 {
                                self.state.insert(key, "true".to_string());
                            } else {
                                self.state.insert(key, "false".to_string());
                            }
                        }
                        NotEquals => {
                            if value != value2 {
                                self.state.insert(key, "true".to_string());
                            } else {
                                self.state.insert(key, "false".to_string());
                            }
                        }
                        
                    }
                    
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



#[cfg(test)]
mod tests {
    
    
    use super::*;

    #[test]
    fn test_basic() {
        let mut interpreter = Interpreter::new();
        let metadata = parse(r#"where metadata field Type is "Landscape"{ change color to "blue" }"#).unwrap().0;
        interpreter.interpret(metadata);
        // I dont like seeing FIELD:TYPE, need to figure out a hashmap structure for comparison operators and the such
        assert_eq!(interpreter.state.get("FIELD:TYPE"), Some(&"Landscape".to_string()));
       
    }
    #[test]
    fn test_date() {
        let mut interpreter = Interpreter::new();
        let metadata = parse(r#"where metadata field Date 12/03/2005 = 12/03/2005{ change color to "blue" }"#).unwrap().0;
        interpreter.interpret(metadata);
        assert_eq!(interpreter.state.get("FIELD:DATE"), Some(&"true".to_string()));
    }
}
