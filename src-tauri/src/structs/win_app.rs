use std::{path::PathBuf, time::SystemTime};
#[derive(Debug)]
pub struct WinApplication{
  pub path: PathBuf,
  pub icos: Vec<PathBuf>,
  pub access_time: SystemTime,
}