use crate::project_manager::{CONFIG_FILE, Config, WORK_DIR};
use std::path::Path;
use tracing::error;

#[derive(PartialEq, Debug)]
pub enum ConfigErr {
    NotConfigured,
    ConfigBroken,
}

fn test_exists() -> bool {
    // PacMine.toml 或者 .pacmine 存在
    Path::new(CONFIG_FILE).exists() || Path::new(WORK_DIR).exists()
}

fn read_config() -> Result<Config, ConfigErr> {
    let config_path = Path::new(CONFIG_FILE);

    // 检查目录/文件是否正确
    if !config_path.is_file() || !Path::new(WORK_DIR).is_dir() {
        return Err(ConfigErr::ConfigBroken);
    }

    match Config::from_file(config_path) {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{:?}", e);
            error!(
                "Failed to read the configuration file. Please check if the configuration file is correct."
            );
            Err(ConfigErr::ConfigBroken)
        }
    }
}

pub fn get_info() -> Result<Config, ConfigErr> {
    if !test_exists() {
        return Err(ConfigErr::NotConfigured);
    }

    read_config()
}

pub fn print_info() {
    match get_info() {
        Ok(v) => {
            println!("{}", v)
        }
        Err(e) => {
            error!("{:?}", e)
        }
    }
}
