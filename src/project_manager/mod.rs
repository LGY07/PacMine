mod config;
mod create;
mod info;
mod run;
pub mod tools;

pub use config::Config;
pub use create::create_project;
pub use info::{get_info, print_info};
pub use run::start_server;

pub const CONFIG_FILE: &str = "PacMine.toml";
pub const WORK_DIR: &str = ".pacmine";
pub const CACHE_DIR: &str = ".pacmine/cache";
pub const BACKUP_DIR: &str = ".pacmine/backup";
pub const RUNTIME_DIR: &str = ".pacmine/runtime";
pub const LOG_DIR: &str = ".pacmine/log";
