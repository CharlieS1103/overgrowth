use std::fs;
use std::path::PathBuf;

fn main() {
  let files = mac_find_app_files();
  println!("{:?}", files.unwrap());
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_home_dir() -> Result < PathBuf, Box<dyn std::error::Error>> {
  match home::home_dir() {
    Some(path) => Ok(path),
    None => panic!("Could not find home directory"),
  } 
}
fn mac_find_app_files() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  let home_path = get_home_dir().unwrap();
  println!("{:?}", home_path);
  // Recursively call the loop_through_dir function and return all the .app files
  let app_files = loop_through_dir(&home_path); 
  Ok(app_files.unwrap())
}
// Create a function that takes a directory path as input, and returns a vector of all the .app files in that directory
fn loop_through_dir(dir_path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  let mut app_files = vec![];
  for entry in fs::read_dir(dir_path).unwrap() {
    if let Ok(entry) = entry {
      let path = entry.path();
      if path.to_str().unwrap().ends_with(".app") {
        app_files.push(path);
      }
      //Check if the path is a directory
      else if path.is_dir() {
        // If it is, recursively call the function on that directory
        let sub_files = loop_through_dir(&path);
        app_files.extend(sub_files.unwrap());
      }
    }
  }
  Ok(app_files)
}

