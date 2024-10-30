mod app_structs;
mod config;
mod parser;
mod utils;
use std::{path::{PathBuf, Path}, error::Error, fs::{self, File, read_dir}, io::{BufReader, Read, BufWriter, Cursor},};
use app_structs::{mac_app::MacApplication, icon_states::generate_toml_file, icon_states::parse_toml_file,};
use config::{parse_config, generate_config};
use plist::Value;
use utils::image_handling::icns_conversion::{convert_icns_to_png, /*convert_pngs_to_icns*/};


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

// TODO: Rename home path to application path for clarity purposes
  let path = get_home_dir().unwrap().join("/Applications");
  // Check if the path.extension is none and if it is 
  const EXTENSION_TYPE: &str = "app";
  // Log path.extension().unwrap()


  let app_files = search_directory(&path, EXTENSION_TYPE, true).unwrap();
   // Iterate through the vector of app files and get the MacApplication struct for each app
   let mut mac_apps: Vec<MacApplication> = Vec::new();
    for app_file in app_files {
      let app = get_mac_app_struct(app_file.clone());
      if app.is_err() {
          continue;
      }
      else if app_file.display().to_string().split(".app").collect::<Vec<&str>>().len() > 2 {
        // This should in all hopes remove any apps which contain additional helper apps inside the directory
        continue;
      }
      else{
        mac_apps.push(app.unwrap());
      }
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

  if mac_store_icns_files(&mac_apps).is_ok() {
    println!("Successfully stored icns files");
  }else{
    println!("Error storing icns files: {}", mac_store_icns_files(&mac_apps).err().unwrap());
  }
  // Store the icns files for each app as a png file in the icons directory as well
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
fn mac_store_icns_files(mac_apps: &Vec<MacApplication>) -> Result<(), Box<dyn Error>> {
  let config = parse_config(&get_home_dir().unwrap());
    let home_dir = get_home_dir().unwrap();
    
    for app in mac_apps {
        // Retrieve the icon file name for the app
        let icon_file_name = match get_icon_file_name(&app.path.to_string_lossy()) {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Error retrieving icon file name for {}: {}", app.path.display(), e);
                continue;
            }
        };
        
        let full_icon_path = home_dir.join(PathBuf::from(&config.icon_dir));

        if !&full_icon_path.exists() {
            fs::create_dir_all(&full_icon_path)?;
        }

                let icn = PathBuf::from(app.icns.clone());
                // Only process the icon file that matches the retrieved icon file name
                if icn.file_stem().unwrap().to_string_lossy() + ".icns" == icon_file_name {
                    let app_icon_dir = app.path.join(&full_icon_path.join(app.path.with_extension("").file_name().unwrap()));
                    if !app_icon_dir.exists() {
                        fs::create_dir_all(app_icon_dir.clone())?;
                    }

                    let icon_path = app_icon_dir.join(&icn);
                    if !icon_path.exists() {
                        fs::copy(home_dir.join(app.icn_path.clone()), icon_path)?;
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
  let app_icns = get_icon_file_name(&path.to_string_lossy())?;

  // Convert app+icons from PathBuf to String
  let app_name = path.file_name().unwrap().to_str().unwrap();
  // Use the icon file name and app name to recursively search for the icns files in the app directory of that name to return the full path to icns and store
  let app_icn_path = search_directory(&path, &app_icns, true).unwrap();
  if app_icn_path.len() == 0 {
    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app icns path")))
  }
  let full_app_icn_path = app_icn_path[0].clone();
  if !app_icns.is_empty() {
    Ok(MacApplication{path : (&path).to_owned(), icns : app_icns, name: (app_name).to_string(), icn_path: full_app_icn_path})
  }
  else {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not get app icns")))
  }
}

// Input an app diretory and it should retrieve the name of the .icns file used by the app via checking the info.plist


fn get_icon_file_name(app_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    let plist_path = Path::new(app_dir).join("Contents").join("Info.plist");

    let mut file = File::open(plist_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    
    let cursor = Cursor::new(contents);
    let plist = Value::from_reader(cursor)?;

    if let Value::Dictionary(dict) = plist {
        if let Some(Value::String(icon_file_name)) = dict.get("CFBundleIconFile") {
          // if value is AppIcon add .icns to the end because for whatever reason plist does not include the extension for the default name :( 
          // or i guess if it doesn't have .icns at the end add it too because favicon is one of those values too ??
          if !icon_file_name.contains(".icns") {
            return Ok(icon_file_name.clone() + ".icns");
          }
            return Ok(icon_file_name.clone());
        }
    }

    Err("Icon file name not found in Info.plist".into())
}

// If a user wants to uninstall overgrowth we should use the backed up icns files in the specified directory to restore the icons to their original state
fn uninstall_overgrowth() -> Result<(), Box<dyn Error>> {
  // THIS FUNCTION IS SUPER SCARY TO TEST!! TODO: WRITE UNIT TESTS FOR THIS FUNCTION
  let config = parse_config(&get_home_dir().unwrap());
  let home_dir = get_home_dir().unwrap();
  let icon_dir = home_dir.join(PathBuf::from(&config.icon_dir));
  let app_dir = home_dir.join("Applications");

  for entry in read_dir(icon_dir)? {
    let entry = entry?;
    let path = entry.path();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let app_name = file_name.split(".icns").collect::<Vec<&str>>()[0];
    let app_path = app_dir.join(app_name);
    let app_icon_path = app_path.join(file_name);
    let original_icon_path = path;

    if app_icon_path.exists() {
      fs::remove_file(app_icon_path.clone())?;
    }

    fs::copy(original_icon_path, app_icon_path)?;
  }

  Ok(())
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
