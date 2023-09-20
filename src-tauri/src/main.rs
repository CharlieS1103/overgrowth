mod app_structs;
mod config;
mod parser;
use std::{path::{PathBuf, Path}, error::Error, fs::{self, File, read_dir}, io::{BufReader, Read, BufWriter}};
use app_structs::{mac_app::MacApplication, icon_states::generate_toml_file, icon_states::parse_toml_file};
use config::{parse_config, generate_config};
use parser::parser::{parse, load_file, ImageMetadata};
use parser::interpreter::{apply_actions_to_images};


fn main() {
  mac_logic();
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


/* This function is recursive and will return a vector of all the files that match the search criteria in the directory and all subdirectories.
 *
 * Parameters:
 * dir_path: The path to the directory to be searched.
 * criteria: A closure that specifies the search criteria. It should take a `PathBuf` and return a `bool`.
 * check_sub_dir: Whether or not to check subdirectories.
 */
fn search_directory(dir_path: &PathBuf, criteria: &str, check_sub_dir: bool) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    // Read the directory entries
    let entries = fs::read_dir(dir_path)?;

    // Filter the entries that match the search criteria
    let matching_entries = entries.filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.to_str().unwrap().ends_with(criteria){
            Some(path)
        } else {
            None
        }
    });

    // Convert the filtered entries into a vector of paths
    let mut matching_paths: Vec<PathBuf> = matching_entries.collect();
    // Print 

    if check_sub_dir {
        // Recursively search the subdirectories
        let subdirs = fs::read_dir(dir_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            });

        for subdir in subdirs {
            let subdir_results = search_directory(&subdir, criteria, check_sub_dir)?;
            matching_paths.extend(subdir_results);
        }
    }
    Ok(matching_paths)
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
// TODO: Rename home path to application path for clarity purposes
  let path = get_home_dir().unwrap().join("/Applications");
  // Check if the path.extension is none and if it is 
  const EXTENSION_TYPE: &str = "app";
  // Log path.extension().unwrap()


  let app_files = search_directory(&path, EXTENSION_TYPE, true).unwrap();
   // Iterate through the vector of app files and get the MacApplication struct for each app
   let mut mac_apps: Vec<MacApplication> = Vec::new();
    for app_file in app_files {
      let app = get_mac_app_struct(app_file).unwrap();
      mac_apps.push(app);
  }
  // Generate the config file
  
  generate_config(&get_home_dir().unwrap());
  // Parse the config file
  parse_config(&get_home_dir().unwrap());
  // Generate the toml file
  match generate_toml_file(&get_home_dir().unwrap(),&mac_apps){
    Ok(_) => println!("Successfully generated toml file"),
    Err(e) => println!("Error generating toml file: {}", e),
  }
 // PArse the toml file and set it to a variable 
  let toml_file = parse_toml_file(&get_home_dir().unwrap().join(".overgrowth/icon_states.toml")).unwrap();
  // Loop through the MacApplication Vec and print the name of each app
  for app in &toml_file {
    println!("Name: {}", app.name);
  }


  if mac_store_icns_files(&mac_apps).is_ok() {
    println!("Successfully stored icns files");
  }
}

/* 




  fn load_scripts() -> Result<Vec<String>, std::io::Error> {
    let script_dir = get_home_dir().unwrap().join(".overgrowth/scripts");
    let mut scripts = Vec::new();
    for entry in read_dir(script_dir)? {
        let path = entry?.path();
        if path.is_file() {
            let contents = load_file(path.to_str().unwrap())?;
            scripts.push(contents);
        }
    }
    Ok(scripts)
}
Need to begin writing an interpreter for the scripts which will take the fields and actions from the parsed script and execute them
*/
// Loop through the MacApplication Vec and store the icns files for each app in the Configs icns-dir
fn mac_store_icns_files(mac_apps :&Vec<MacApplication>) -> Result<(), Box<dyn std::error::Error>> {
  let config = parse_config(&get_home_dir().unwrap());
  let home_dir = get_home_dir().unwrap();
  for app in mac_apps {
    let icns = &app.icns;
    // TODO: Should probably make icon_dir a PathBuf instead of a String from the start, but for now this works.
    let icn_path = &config.icon_dir;
    // icn_path (default): ./overgrowth/icons

      // Convert icn_path into a PathBuf
    let icn_path = PathBuf::from(icn_path);
    let full_icon_path = home_dir.join(&icn_path);
    // full_icon_path (default): /Users/{username}/.overgrowth/icons
       if !&full_icon_path.exists() {
        fs::create_dir_all(&full_icon_path)?;
      }
    for icn in icns {
      // Check to see if the file already exists in the configs icon dir
      let app_icon_dir = &full_icon_path.join(app.path.with_extension("").file_name().unwrap());
      // app_icon_dir (default): Users/{username}/.overgrowth/icons/{app name}/
      //Convert icn to a PathBuf
      let icn = &PathBuf::from(icn);
      // Check if the app icon dir exists and if it doesn't create it
      if !app_icon_dir.exists() {
        fs::create_dir_all(app_icon_dir)?;
      }
      if !app_icon_dir.join(&icn.file_name().unwrap()).exists() {
        // If it doesn't exist, copy the file to the configs icon dir
        // Check if there is another .app file in the icn path, if so, create a new directory for the app
        fs::copy(home_dir.join(icn), app_icon_dir.join(&icn.file_name().unwrap()))?;
        
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
  const EXTENSION_TYPE: &str = "icns";
  let app_icns: Result<Vec<PathBuf>, Box<dyn Error>> = search_directory(&path, EXTENSION_TYPE, true);
  // Convert app+icons from PathBuf to String

  let app_name : String = (&path.file_name().unwrap().to_str().unwrap()).to_string();
  if app_icns.is_ok() {
    let app_icns: Vec<String> = app_icns.unwrap().iter().map(|x| x.to_str().unwrap().to_string()).collect();
   
    Ok(MacApplication{path : (&path).to_owned(), icns : app_icns, name: (app_name).to_string()})
  }
  else {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app icns")))
  }
}

// What i need to do for parsing
// 1. Provide a folder in the app settings directory for the embedded scripts
// 2. Load the embedded scripts 
// 3. Parse the embedded scripts
// 4. Execute the parsed code 






// 1. Get the home directory of the current user
// 2. Search the home directory for .app files
// 3. For each .app file, get the name of the app, the path to the app, and the path to the icns files for the app
// 4. Store the icns files in the configs icon directory
// 5. Generate the toml file
// 6. Parse the toml file
// 7. For each app in the toml file, get the name of the app, the path to the app, and the path to the icns files for the app
// 8. For each app in the toml file, loop through the icns files and change the icon of the app to the current icns file
// 9. Restart the dock
// 10. Clear the dock cache
