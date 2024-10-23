extern crate serde;

extern crate toml;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MacApplication {
    pub path: PathBuf,
    pub name: String,
    pub icns: String,
    pub icn_path: PathBuf,
}

