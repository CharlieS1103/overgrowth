use icns::Image;

use super::parser::image_metadata;
use super::parser::ImageMetadata;

/* 
pub fn apply_actions_to_images(images: &mut Vec<ImageMetadata>, fields: &Vec<(String, String)>, actions: &Vec<(String, String)>) {
    for image in images {
        for field in fields {
            if field.1 == "" || image.get(&field.0) == Some(&field.1) {
                for action in actions {
                    match action.0.as_str() {
                        "ChangeColor" => image.color = action.1.clone(),
                        "AddOverlay" => image.add_overlay(action.1.clone()),
                        // add more actions here as needed
                        _ => (),
                    }
                }
            }
        }
    }
}


#[cfg(test)]

mod tests {
    
    use combine::EasyParser;

    use super::*;

    #[test]
    fn test_apply_actions_to_images() {
        // create some test images using the parsers test prompt
        //let input: &str = r#"where metadata field "Type" is "Landscape"{ change color to "blue" }"#;
        //let result: Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> = image_metadata().easy_parse(input);

        let input = r#"where metadata field "Type" is "Landscape" and metadata field "Author" is "John Doe" { change color to "blue" add overlay file: "logo.png" }"#;
        let result = image_metadata().easy_parse(input);
        // use the result to test the apply_actions_to_images function
        let mut images = vec![result.unwrap().0];
        let fields = vec![("Type".to_string(), "Landscape".to_string()), ("Author".to_string(), "John Doe".to_string())];
        let actions = vec![("ChangeColor".to_string(), "blue".to_string()), ("AddOverlay".to_string(), "logo.png".to_string())];
        apply_actions_to_images(&mut images, &fields, &actions);
        assert_eq!(images[0].color, "blue");
        assert_eq!(images[0].overlays[0], "logo.png");
        
    }
}
*/