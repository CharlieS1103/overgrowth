/* use icns::Image;

use crate::app_structs::mac_app::MacApplication;

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
    // This needs to determine which apps pass comparisons and which do not and then apply the actions to the ones that do
    pub fn interpret(&mut self, mac_apps: Vec<MacApplication>, metadata: ImageMetadata) {
        for app in mac_apps {
            let mut pass = true;
            for field in metadata.fields.iter() {
                match field {
                    MetadataField::Type(t,_) => {
                        if app.app_type != *t {
                            pass = false;
                        }
                    }
                    MetadataField::Date(d,_) => {
                        if app.date != *d {
                            pass = false;
                        }
                    }
                    MetadataField::Size(s,_) => {
                        if app.size != *s {
                            pass = false;
                        }
                    }
                    MetadataField::Name(n,_) => {
                        if app.name != *n {
                            pass = false;
                        }
                    }
                    MetadataField::Version(v,_) => {
                        if app.version != *v {
                            pass = false;
                        }
                    }
                    MetadataField::Author(a, _) => {
                        if app.author != *a {
                            pass = false;
                        }
                    }
                    MetadataField::Description(d) => {
                        if app.description != *d {
                            pass = false;
                        }
                    }
                    MetadataField::Rating(r) => {
                        if app.rating != *r {
                            pass = false;
                        }
                    }
                    MetadataField::Price(p) => {
                        if app.price != *p {
                            pass = false;
                        }
                    }
                    MetadataField::Popularity(p) => {
                        if app.popularity != *p {
                            pass = false;
                        }
                    }
                    MetadataField::Downloads(d) => {
                        if app.downloads != *d {
                            pass = false;
                        }
                    }
                    MetadataField::Category(c) => {
                        if app.category != *c {
                            pass = false;
                        }
                    }
                    MetadataField::Tags(t) => {
                        if app.tags != *t {
                            pass = false;
                        }
                    }
                    MetadataField::Keywords(k) => {
                        if app.keywords != *k {
                            pass = false;
                        }
                    }
                    MetadataField::License(l) => {
                        if app.license != *l {
                            pass = false;
                        }
                    }
                    MetadataField::Website(w) => {
                        if app.website != *w {
                            pass = false;
                        }
                    }
                    MetadataField::Source(s) => {
                        if app.source != *s {
                            pass = false;
                        }
                    }
                    MetadataField::Language(l) => {
                        if app.language != *l {
                            pass = false;
                        }
                    }
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
        interpreter.interpret(vec![], metadata);
        
       
    }
    #[test]
    fn test_date() {
        let mut interpreter = Interpreter::new();
        let metadata = parse(r#"where metadata field Date 12/03/2005 = 12/03/2005{ change color to "blue" }"#).unwrap().0;
        interpreter.interpret(metadata);
        assert_eq!(interpreter.state.get("FIELD:DATE"), Some(&"true".to_string()));
    }
}


*/