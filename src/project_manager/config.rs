use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ServerType {
    OfficialJE,
    OfficialBE,
    Paper,
    Spigot,
    Purpur,
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum JavaType {
    GraalVM,
    OracleJDK,
    OpenJDK,
    Custom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub(crate) name: String,
    pub(crate) server_type: ServerType,
    pub(crate) version: String,
    pub(crate) execute: String,
    pub(crate) birthday: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Java {
    pub(crate) version: usize,
    pub(crate) edition: JavaType,
    #[serde(default)]
    pub(crate) custom: String,
    #[serde(default)]
    pub(crate) arguments: Vec<String>,
    pub(crate) xms: usize,
    pub(crate) xmx: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Runtime {
    pub(crate) java: Java,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
    pub(crate) interval: usize,
    #[serde(default)]
    pub(crate) cron: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub(crate) start: bool,
    pub(crate) stop: bool,
    pub(crate) update: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Backup {
    pub(crate) enable: bool,
    pub(crate) world: bool,
    pub(crate) other: bool,
    #[serde(default)]
    pub(crate) time: Option<Time>,
    #[serde(default)]
    pub(crate) event: Option<Event>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Plugin {
    name: String,
    #[serde(default)]
    disable: bool,
    #[serde(default)]
    update: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginManage {
    pub(crate) manage: bool,
    #[serde(default)]
    pub(crate) plugin: Vec<Plugin>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub(crate) project: Project,
    pub(crate) runtime: Runtime,
    pub(crate) backup: Backup,
    pub(crate) plugin_manage: PluginManage,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }


    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    #[cfg(test)]
    pub fn test_config() -> Config {
        Config {
            project: Project {
                name: "MyServer".to_string(),
                server_type: ServerType::OfficialJE,
                execute: "server.jar".to_string(),
                version: "1.21.0".to_string(),
                birthday: "2025-01-01".to_string(),
            },
            runtime: Runtime {
                java: Java {
                    version: 21,
                    edition: JavaType::OpenJDK,
                    custom: String::new(),
                    arguments: vec![
                        "-Xms 1024M".to_string(),
                        "-Xmx 1024M".to_string(),
                    ],
                    xms: 2048,
                    xmx: 4096,
                },
            },
            backup: Backup {
                enable: true,
                world: true,
                other: false,
                time: Some(Time {
                    interval: 3600,
                    cron: "0 */6 * * *".to_string(),
                }),
                event: Some(Event {
                    start: false,
                    stop: true,
                    update: true,
                }),
            },
            plugin_manage: PluginManage {
                manage: true,
                plugin: vec![
                    Plugin {
                        name: "EssentialsX".to_string(),
                        disable: false,
                        update: true,
                    },
                    Plugin {
                        name: "Vault".to_string(),
                        disable: false,
                        update: false,
                    },
                ],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = Config::test_config();
        let test_path = Path::new("./target/test.toml");
        config.to_file(test_path).unwrap();
        let config = Config::from_file(test_path).unwrap();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        println!("{}", toml_str);
    }
}