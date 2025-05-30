mod app_structs;
mod config;
mod parser;
mod utils;
use std::{error::Error, fs::{self, /*read_dir,*/ File}, io::{/*BufReader, BufWriter,*/ Cursor, Read}, path::{Path, PathBuf}};
use app_structs::{mac_app::MacApplication, icon_states::generate_toml_file};
use config::{parse_config, generate_config};
use plist::Value;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, GenericImage, Pixel};
// use utils::utils::uninstall_overgrowth;
use utils::image_handling::icns_conversion::{convert_icns_to_png, /*convert_pngs_to_icns*/};
use utils::image_handling::add_overlay::add_overlay;
use tauri::command;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![mac_logic])
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
#[command]
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
    if vine_demo(&mac_apps).is_ok() {
      println!("Successfully added vine overlays");
  } else {
      println!( "Error adding vine overlays: {}", vine_demo(&mac_apps).err().unwrap());
  }
  }else{
    println!("Error storing icns files: {}", mac_store_icns_files(&mac_apps).err().unwrap());
  }
  // Lets add a demo with just adding vine overlay to all the apps depending on last time they were modified and see if it works
  // Start by adding vines to the pngs 
  
  // Convert pngs to icns and replace the app icon with it
  // Ensure the original icon is stored in the icons directory (for use as a backup)
  // Store the icns files for each app as a png file in the icons directory as well
}


fn vine_demo(mac_apps: &Vec<MacApplication>) -> Result<(), Box<dyn Error>> {
    let home_dir = get_home_dir().unwrap();
    let config = parse_config(&home_dir);
    let icon_dir = home_dir.join(&config.icon_dir);

    // Load vine assets
    let vine_assets = load_vine_assets(&home_dir.join(".overgrowth/assets"))?;

    for app in mac_apps {
        let app_icon_dir = icon_dir.join(app.path.with_extension("").file_name().unwrap());
        let png_files = search_directory(&app_icon_dir, ".png", true)?;
        // check if the app has been used in the last 30 days
        let metadata = fs::metadata(&app.path)?;
        let modified_time = metadata.modified()?;
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(modified_time)?;
        let days = duration.as_secs() / 86400;
        if days > 30 {
          for png_file in png_files {
            let mut image = image::open(&png_file)?;

            for vine in &vine_assets {
                image = add_overlay(image, vine);
            }
            // Save the modified image
            image.save(&png_file)?;

    }
        }

    }

    Ok(())
}

fn load_vine_assets(assets_dir: &PathBuf) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
    let mut vine_assets = Vec::new();
    println!("Loading vine assets from {}", assets_dir.display());
    for entry in fs::read_dir(assets_dir)? {
        let entry = entry?;
        println!("Entry: {}", entry.path().display());
        let path = entry.path();
        // if entry is ds_store, skip it
        if path.file_name().unwrap() == ".DS_Store" {
            continue;
        }
        if path.extension().unwrap() == "png" {
          // print asset path
          println!("Adding {}", path.display());
            let vine = image::open(&path)?;
            vine_assets.push(vine);
        }
    }
    Ok(vine_assets)
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
                   let _ = convert_icns_to_png(app_icon_dir.join(&icn),app_icon_dir );
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




// TODO: Change variable names around for overgrowth clones to have a c underscore prefix just so i can understand the code a tad easier
// Also need to add examples of what each variable looks like just so i can understand my own code





// What i need to do for parsing
// 3. Parse the embedded scripts
// 4. Execute the parsed code 




// 8. For each app in the toml file, loop through the icns files and change the icon of the app to the current icns file
