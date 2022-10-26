use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VineState {
    // An array of key value pairs where the key is the app path and the value is the vine state
    pub vine_states: Vec<(String, String)>,
}
impl VineState {
 pub fn new() -> Self {
        VineState {
            vine_states: Vec::new(),
        }
    }
    pub fn contains_key(&self, app_path: &str) -> bool {
        for (key, _) in &self.vine_states {
            if key == app_path {
                return true;
            }
        }
        return false;
    }
    pub fn get_vine_state(&self, app_path: &str) -> String {
        for (key, value) in &self.vine_states {
            if key == app_path {
                return value.to_string();
            }
        }
        return "0".to_string();
    }
    pub fn get_mut(&mut self, app_path: &str) -> Option<&mut String> {
        for (key, value) in &mut self.vine_states {
            if key == app_path {
                return Some(value);
            }
        }
        return None;
    }
    pub fn insert(&mut self, app_path: String, vine_state: String) {
        self.vine_states.push((app_path, vine_state));
    }
    pub fn replace(&mut self, app_path: String, vine_state: &String) {
        for (key, value) in &mut self.vine_states {
            if key == &app_path {
                *value = (&vine_state).to_string();
            }
        }
    }
}
