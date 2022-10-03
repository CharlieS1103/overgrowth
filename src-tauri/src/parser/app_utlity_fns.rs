use crate::structs::mac_app::MacApplication;

// Create a function that takes a mac app as input and return the letter that the app starts with
pub fn get_first_letter(mac_app : &MacApplication) -> String{
    let first_letter = mac_app.name.chars().next().unwrap();
    return first_letter.to_string();
}
pub fn letter_to_num(letter : &str) -> i32{
    let mut num = 0;
    for c in letter.chars() {
        num += c as i32;
    }
    return num;
}