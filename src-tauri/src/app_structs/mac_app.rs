use std::path::PathBuf;
use std::time::SystemTime;
#[derive(Debug)]
pub struct MacApplication{
  pub path: PathBuf,
  pub icns: Vec<PathBuf>,
  pub access_time: SystemTime,
}
impl MacApplication{
  fn new(path: PathBuf, icns : Vec<PathBuf>, access_time : SystemTime) -> MacApplication{
    MacApplication{
      path,
      icns: vec![],
      access_time,
    }
  }
}