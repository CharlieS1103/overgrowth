
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub struct MacApplication{
  pub path: PathBuf,
  pub icns: Vec<PathBuf>,
  pub name : String,
}
// Implement new for MacApplication
impl MacApplication {
  pub fn new(path: PathBuf, icns: Vec<PathBuf>, name : String) -> Self {
    Self {
      path,
      icns,
      name,
    }
  }
}