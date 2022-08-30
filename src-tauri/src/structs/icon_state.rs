use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IconState {
    // An array of key value pairs where the key is the app path and the value is the icon state
    pub icon_states: Vec<(String, String)>,
}
impl IconState {
 pub fn new() -> Self {
        IconState {
            icon_states: Vec::new(),
        }
    }
    pub fn contains_key(&self, app_path: &str) -> bool {
        for (key, _) in &self.icon_states {
            if key == app_path {
                return true;
            }
        }
        return false;
    }
    pub fn get_icon_state(&self, app_path: &str) -> String {
        for (key, value) in &self.icon_states {
            if key == app_path {
                return value.to_string();
            }
        }
        return "0".to_string();
    }
    pub fn get_mut(&mut self, app_path: &str) -> Option<&mut String> {
        for (key, value) in &mut self.icon_states {
            if key == app_path {
                return Some(value);
            }
        }
        return None;
    }
    pub fn insert(&mut self, app_path: String, icon_state: String) {
        self.icon_states.push((app_path, icon_state));
    }
    pub fn replace(&mut self, app_path: String, icon_state: &String) {
        for (key, value) in &mut self.icon_states {
            if key == &app_path {
                *value = (&icon_state).to_string();
            }
        }
    }
}
