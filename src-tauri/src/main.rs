mod app_structs;
mod config;
use std::{path::PathBuf, time::SystemTime, process::Stdio, process::Command, error::Error, fs::{self, File}, io::{BufWriter, BufReader}};
use app_structs::{mac_app::MacApplication};
use config::{parse_config, generate_config};
use icns::{IconFamily};




fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Make a function to convert a .icns file to a .png file
// TODO: Cleanup this entire function
fn convert_icns_to_png(icns_path: &PathBuf){
  let file = BufReader::new(File::open(icns_path).unwrap());
    let  icon_family = IconFamily::read(file).unwrap();
    let icon_type = icon_family.available_icons(); 
   // Loop thorugh all the available icon types and convert them to png files
    for icon in icon_type {
      // TODO: We need to figure out how to handle Jpeg 2000 icons 
      let image =  
      match icon_family.get_icon_with_type(icon){
        Ok(_) => icon_family.get_icon_with_type(icon).unwrap(),
        Err(_) => continue,
      };
      // Create a direcory based on the icn file name 
      let png_dir = icns_path.with_extension("");
      // check if the directory exists, if not create it
      if !png_dir.exists() {
        fs::create_dir_all(&png_dir).unwrap();
      }
      
      let icon_path = &png_dir.join(format!("{:?}.png", icon));
      let file = File::create(&icon_path).unwrap();
      image.write_png(file).unwrap();
      
    }
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

// Handle the MacOS logic
fn mac_logic(){
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
  }
  if mac_store_icns_files(&mac_apps).is_ok() {
    println!("Successfully stored icns files");
  }
  else {
    println!("{:?}", mac_store_icns_files(&mac_apps));
  }
  
  
}

// Convert the access time to the correct Vine State
fn get_vine_state(mac_app : &MacApplication) -> String {
  let config = parse_config(&get_home_dir().unwrap());
  let mut vine_state: String;
  let app_access_time = mac_app.access_time;
  let current_time = SystemTime::now();
  // Get the number of days between the app access time and the current time
  let days_between = current_time.duration_since(app_access_time).unwrap().as_secs() / 86400;
  if days_between > config.stage_one_days{
    vine_state = "1".to_string();
  }
  else if days_between > config.stage_two_days{
    vine_state = "2".to_string();
  }
  else if days_between > config.stage_three_days{
    vine_state = "3".to_string();
  }
  else if days_between > config.stage_four_days{
    vine_state = "4".to_string();
  }
  else if days_between > config.stage_five_days{
    vine_state = "5".to_string();
  }
  else {
    vine_state = "0".to_string();
  }
  return vine_state;
}

// Loop through the MacApplication Vec and store the icns files for each app in the Configs icns-dir
fn mac_store_icns_files(mac_apps :&Vec<MacApplication>) -> Result<(), Box<dyn std::error::Error>> {
  let config = parse_config(&get_home_dir().unwrap());
  let home_dir = get_home_dir().unwrap();
  for app in mac_apps {
    let icns = &app.icns;
    // TODO: Should probably make icon_dir a PathBuf instead of a String from the start, but for now this works.
    for icn in icns {
      // Create an empty icon dir if it doesn't exist
      
      let icn_path = &config.icon_dir;
      
      // Convert icn_path into a PathBuf
      let icn_path = PathBuf::from(icn_path);
      let full_icon_path = home_dir.join(&icn_path);
       if !full_icon_path.exists() {
        fs::create_dir_all(full_icon_path)?;
      }
      // Check to see if the file already exists in the configs icon dir
      if !home_dir.join(icn_path.join(icn.file_name().unwrap())).exists() {
        // If it doesn't exist, copy the file to the configs icon dir
        fs::copy(home_dir.join(icn), home_dir.join(icn_path.join(icn.file_name().unwrap())))?;
        
      }
      else{
        // If it does exist already, do nothing
        println!("{} already exists", icn.display());
      }
    }
  }
  Ok(())
}

/*  This function will return a MacApplication object which contains the name of the application, the path to the application, and the path to all the icons for the application
  * Parameters:
  *  app_path: The path to the application
  */
fn get_mac_app_struct(path : PathBuf) -> Result<MacApplication, Box<dyn std::error::Error>> {
   let last_access_time: SystemTime = fs::metadata(&path)?.accessed().unwrap();
  let app_icns: Result<Vec<PathBuf>, Box<dyn Error>> = loop_through_dir(&path, &".icns".to_string(), true);
  if app_icns.is_ok() {
    let app_icns: Vec<PathBuf> = app_icns.unwrap();
   
    Ok(MacApplication{path : path, icns : app_icns, access_time : last_access_time})
  }
  else {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app icns")))
  }
}

// TODO: Commission an artist to make the 16x16 vine icons with transparency 
/* Overlays one image ontop of another image (NEEDS TO BE CONVERTED TO PNG BEFOREHAND)
* Parameters:
*  base_image: The image to be overlaid on top of another image
*  overlay_image: The image to be overlayed on top of the base image
* Returns: 
*  The image with the overlayed image on top of the base image
*/
fn add_overlay(mut base_image : image::DynamicImage, overlay_image : &image::DynamicImage) -> image::DynamicImage {
  image::imageops::overlay(&mut base_image, overlay_image,0 ,0 );
  return base_image;
}

/*
Essentially just restart the mac dock and clear the cache of the dock icons to make sure the icons are up to date
*/
fn _restart_dock_mac(){
    let echo_child_rm = Command::new("bash")
                .args(["-c", "rm /var/folders/*/*/*/com.apple.dock.iconcache; killall Dock"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child_rm);
}





/* 
// Windows integrations: 


// Windows specific function to find all the .lnk files
fn _win_find_lnk_files(){
  /*  For now only look for .lnk files in the /Desktop directory just for the sake of making development faster
   In the future also scan the startup folder 
   */
  let home_path = get_home_dir().unwrap().join("/Desktop");
  // "/User/{username}/C:\Users\<USERNAME>\Desktop"
  let app_files = loop_through_dir(&home_path, &".lnk".to_string(), false).unwrap(); 
   // Iterate through the vector of lnk files and get the WinApplication struct for each lnk
   let mut win_apps: Vec<WinApplication> = Vec::new();
    for app_file in app_files {
      let app = _get_win_app_struct(app_file).unwrap();
      win_apps.push(app);
      println!("{:?}", win_apps);
  }
}

fn _get_win_app_struct(path: PathBuf) -> Result<WinApplication, Box<dyn std::error::Error>> {
  let app_ico: Result<Vec<PathBuf>, Box<dyn Error>> = loop_through_dir(&path, &".ico".to_string(), false);
  let last_access_time: SystemTime = fs::metadata(&path)?.accessed().unwrap();
  if app_ico.is_ok() {
    let app_ico = app_ico.unwrap();
    Ok(WinApplication{path : path, icos: app_ico, access_time : last_access_time})
  }
  else {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app lnk")))
  }
}

*/




