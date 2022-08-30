use std::path::PathBuf;
use std::time::SystemTime;
#[derive(Debug)]
pub struct MacApplication{
  pub path: PathBuf,
  pub icns: Vec<PathBuf>,
  pub access_time: SystemTime,
  pub name : String,
}