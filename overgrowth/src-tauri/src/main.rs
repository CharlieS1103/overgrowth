/*
How it works so far:
Upon running, it will execute the mac_find_app_files function, which will find all the files ending with .app in the Applications folder and store their paths in a vector.
We will then iterate through the vector and use the get_app_icns function to get the icns file for each app and store each app as a MacApplication struct in a vector.
Next, we will iterate thorugh the app vector, and iterate through the app's icns vector, and use a function to modify the icns file
And finally we will replace the previous file with the modified version

Once that initial integration is finished, we will need to add the access time handling
We also need to store the original icon file somewhere in the users device, so we should sort that out as well (So we can later restore the icon to the original state if it has been accessed more recently or if the user wants to revert to the original icon)

NOTE: For the purpose of development, i heavily recommend just making a directory that has a singular .app file in it, and then just using that so you don't accidentally modify all your apps to permanently have a weird overlay.
*/

mod app_structs;
use app_structs::mac_app::MacApplication;
use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use std::process::Command;
fn main() {
  mac_find_app_files();
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

/* This function is recursive and will return a vector of all the .app files in the directory and all subdirectories 
 * Parameters:
 * dir_path: The path to the directory to be searched
 * file_extension: The file extension to be searched for
 * check_sub_dir: Whether or not to check subdirectories
 */ 
fn loop_through_dir(dir_path: &PathBuf, extension_type: &String, check_sub_dir: bool) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  let mut found_files = vec![];
  for entry in fs::read_dir(dir_path)? {
    if let Ok(entry) = entry {

      // Check if we have permissions to the directory
      if entry.metadata()?.permissions().readonly(){
        continue;
      }
      if !entry.metadata().is_ok(){
        println!("Could not get metadata for: {}", entry.path().display());
        continue; 
      }
      let path = entry.path();
      if path.to_str().unwrap().ends_with(extension_type) {
          found_files.push(path);

      }
      else if path.is_dir() && check_sub_dir {
        // If it is a directory,  recursively call the function on that directory and append the results to the vector
        let sub_files = loop_through_dir(&path, &extension_type, check_sub_dir);
        if sub_files.is_ok() {
          found_files.append(&mut sub_files.unwrap());
        }
        
      }
    }
  }
  Ok(found_files)
}


// Mac Integrations: 

//Mac OS X specific function to find all the .app files
fn mac_find_app_files(){
  // For now only look for .app files in the /Applications directory just for the sake of making development faster
/*  TODO: Make this function not loop through the home directory and target directories which would typically house app files 
 * "{homedir}/Applications"
 * "{homedir}/Downloads" 
 * "{homedir}/Documents" 
 * "{homedir}/Desktop")
*/
  let home_path = get_home_dir().unwrap().join("/Applications");
  // "/User/{username}/Applications"
  let app_files = loop_through_dir(&home_path, &".app".to_string(), false).unwrap(); 
   // Iterate through the vector of app files and get the MacApplication struct for each app
   let mut mac_apps: Vec<MacApplication> = Vec::new();
    for app_file in app_files {
      let app = get_mac_app_struct(app_file).unwrap();
      mac_apps.push(app);
      println!("{:?}", mac_apps);
  }
  // Next step is to iterate through the mac_apps vector and iterate through the app's icns vector, and use a function to modify the icns file
}


/*  This function will return a MacApplication object which contains the name of the application, the path to the application, and the path to all the icons for the application
  * Parameters:
  *  app_path: The path to the application
  */
fn get_mac_app_struct(path : PathBuf) -> Result<MacApplication, Box<dyn std::error::Error>> {
   let last_access_time = fs::metadata(&path)?.accessed().unwrap();
  let app_icns = loop_through_dir(&path, &".icns".to_string(), true);
  if app_icns.is_ok() {
    let app_icns = app_icns.unwrap();
   
    Ok(MacApplication{path : path, icns : app_icns, access_time : last_access_time})
  }
  else {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app icns")))
  }
}




/*
Essentially just restart the mac dock and clear the cache of the dock icons to make sure the icons are up to date
*/
fn restart_dock_mac(){
    let echo_child_rm = Command::new("bash")
                .args(["-c", "rm /var/folders/*/*/*/com.apple.dock.iconcache; killall Dock"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child_rm);
}






// Windows integrations: 


/* 
// Windows specific function to find all the .lnk files
fn win_find_lnk_files(){
  /*  For now only look for .lnk files in the /Desktop directory just for the sake of making development faster
   In the future also scan the startup folder 
   */
  let home_path = get_home_dir().unwrap().join("/Desktop");
  // "/User/{username}/C:\Users\<USERNAME>\Desktop"
  let app_files = loop_through_dir(&home_path, &".lnk".to_string(), false).unwrap(); 
   // Iterate through the vector of lnk files and get the WinApplication struct for each lnk
   let mut win_apps: Vec<WinApplications> = Vec::new();
    for app_file in app_files {
      let app = get_win_app_struct(app_file).unwrap();
      win_apps.push(app);
      println!("{:?}", win_apps);
  }
}

fn get_win_app_struct(path: PathBuf) -> Result<WinApplications, Box<dyn std::error::Error>> {
  let app_ico = loop_through_dir(&path, &".ico".to_string(), false);
  if app_ico.is_ok() {
    let app_ico = app_ico.unwrap();
    let last_access_time = fs::metadata(path)?.last_access_time();
    Ok(WinApplications{path : path, lnk : app_lnk, access_time : last_access_time})
  }
  else {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app lnk")))
  }
}
*/