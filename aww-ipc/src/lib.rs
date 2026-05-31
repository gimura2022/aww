use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CliCommand {
    RefeshConfig(PathBuf),
    Kill,
}
