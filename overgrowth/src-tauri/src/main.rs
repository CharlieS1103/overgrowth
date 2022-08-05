/*
How it works so far:
Upon running, it will execute the mac_find_app_files function, which will find all the files ending with .app in the Applications folder and store their paths in a vector.
We will then iterate through the vector and use the get_app_icns function to get the icns file for each app and store each app as a MacApplication struct in a vector.
Next, we will iterate thorugh the app vector, and iterate through the app's icns vector, and use a function to modify the icns file
And finally we will replace the previous file with the modified version

Once that initial integration is finished, we will need to add the access time handling
We also need to store the original icon file somewhere in the users device, so we should sort that out as well


NOTE: For the purpose of development, i heavily recommend just making a directory that has a singular .app file in it, and then just using that so you don't accidentally modify all your apps to permanently have a weird overlay.
*/




*/

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
/*  TODO: Make this function not loop through the home directory and target directories which would typically house app files 
 * "{homedir}/Applications"
 * "{homedir}/Downloads" 
 * "{homedir}/Documents" 
 * "{homedir}/Desktop")
*/
  let home_path = get_home_dir().unwrap().join("/Applications");
  // "/User/{username}/Applications"
  let app_files = loop_through_dir(&home_path, &".app".to_string()); 
  Ok(app_files.unwrap())
}


/* This function is recursive and will return a vector of all the .app files in the directory and all subdirectories 
 * Parameters:
 * dir_path: The path to the directory to be searched
 * file_extension: The file extension to be searched for
 */ 
fn loop_through_dir(dir_path: &PathBuf, extension_type: &String) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  let mut app_files = vec![];
  for entry in fs::read_dir(dir_path)? {
    if let Ok(entry) = entry {
      // Check if we have permissions to the directory
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
/*  This function will return a MacApplication object which contains the name of the application, the path to the application, and the path to all the icons for the application
  * Parameters:
  *  app_path: The path to the application
  */

fn get_app_icns(path : PathBuf) -> Result<MacApplication, Box<dyn std::error::Error>> {
  let app_icns = loop_through_dir(&path, &".icns".to_string()).unwrap();
  let mut app = MacApplication {
    name: path.file_name().unwrap().to_str().unwrap().to_string(),
    path: path,
    icns: app_icns,
  };
  Ok(app)
}

struct MacApplication{
  name: String,
  path: PathBuf,
  icns: Vec<PathBuf>,
}
impl MacApplication{
  fn new(name: String, path: PathBuf) -> MacApplication{
    MacApplication{
      name,
      path,
      icns: vec![],
    }
  }
}

