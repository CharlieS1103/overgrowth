use std::path::PathBuf;
#[derive(Debug)]
pub struct WinApplication{
  pub path: PathBuf,
  pub icos: Vec<PathBuf>,
}
impl WinApplication{
  fn new(path: PathBuf, icns : Vec<PathBuf>) -> WinApplication{
    WinApplication{
      path,
      icos: vec![],
    }
  }
}
