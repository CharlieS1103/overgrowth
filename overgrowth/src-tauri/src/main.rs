use std::fs;
use std::path::PathBuf;

fn main() {
  let files = mac_find_app_files();
  println!("{:?}", files.unwrap());
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
// Return the home_dir of the current user.
fn get_home_dir() -> Result < PathBuf, Box<dyn std::error::Error>> {
  match home::home_dir() {
    Some(path) => Ok(path),
    None => panic!("Could not find home directory"),
  } 
}
fn mac_find_app_files() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {

  // For now only look for .app files in the /Applications directory just for the sake of making development faster
  let home_path = get_home_dir().unwrap().join("/Applications");
  
  // Recursively call the loop_through_dir function and return all the .app files
  let app_files = loop_through_dir(&home_path, &".app".to_string()); 
  Ok(app_files.unwrap())
}


/* This function is recursive and will return a vector of all the .app files in the directory and all subdirectories 
 * dir_path: The path to the directory to be searched
 * file_extension: The file extension to be searched for
 */ 
fn loop_through_dir(dir_path: &PathBuf, extension_type: &String) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  /*  TODO: Make this function not loop through the home directory and target directories which would typically house app files 
 * "{homedir}/Applications"
 * "{homedir}/Downloads" 
 * "{homedir}/Documents" 
 * "{homedir}/Desktop")
*/
  let mut app_files = vec![];
  // Check if we have permissions to the directory
  for entry in fs::read_dir(dir_path)? {
    if let Ok(entry) = entry {
      if entry.metadata()?.permissions().readonly(){
        println!("Permissions denied: {}", entry.path().display());
        continue;
      }
      let path = entry.path();
      if path.to_str().unwrap().ends_with(".app") {
        println!("Pushing application: {:?}", path);
        app_files.push(path);
      }
      //Check if the path is a directory
      
      else if path.is_dir() {
        // If it is, recursively call the function on that directory and append the results to the vector
        let sub_files = loop_through_dir(&path, &extension_type);
        if sub_files.is_ok() {
          app_files.append(&mut sub_files.unwrap());
        }
        
      }
    }
  }
  Ok(app_files)
}