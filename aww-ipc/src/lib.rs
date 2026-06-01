use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CliCommand {
    RefreshConfig(PathBuf),
    Kill
}
