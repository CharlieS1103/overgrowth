use std::path::PathBuf;
#[derive(Debug)]
pub struct MacApplication{
  pub path: PathBuf,
  pub icns: Vec<PathBuf>,
}
impl MacApplication{
  fn new(path: PathBuf, icns : Vec<PathBuf>) -> MacApplication{
    MacApplication{
      path,
      icns: vec![],
    }
  }
}