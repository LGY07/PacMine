use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::project_manager::Config;
use crate::project_manager::config::{Backup, Event, Java, JavaType, Plugin, PluginManage, Project, Runtime, ServerType, Time};
use crate::project_manager::get_info::{get_info, NotValid};

struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err("The version number must be in x.x.x format".to_string());
        }

        let major = parts[0].parse()
            .map_err(|_| format!("Invalid major version number:{}", parts[0]))?;
        let minor = parts[1].parse()
            .map_err(|_| format!("Invalid minor version number:{}", parts[1]))?;
        let patch = parts[2].parse()
            .map_err(|_| format!("Invalid patch version number:{}", parts[2]))?;

        Ok(Version { major, minor, patch })
    }
    fn to_string(&self) -> String{
        format!("{}.{}.{}",&self.major,&self.minor,&self.patch)
    }
}


fn broken_config(){
    eprintln!("{}","There are files related to NMSL in the current directory, but they may be damaged. Please check the .nmsl directory and the NMSL.toml file. You need to manually delete them to continue creating.".yellow())
}

fn get_input()->String{
        let mut input_buffer = String::new();
        print!(">");
        std::io::Write::flush(&mut std::io::stdout()).expect("ERROR: Failed to print the prompt message");
        match std::io::stdin().read_line(&mut input_buffer) {
            Ok(_) => input_buffer,
            Err(_)=> panic!("Unknown input error!")
        }
}

fn get_config()->Config{

    println!("Enter the name of this project:");
    let project_name = get_input();

    println!("Select the type of the server:");
    println!("1: Minecraft Server(Official)");
    println!("2: PaperMC");
    println!("3: PurpurMC");
    println!("4: SpigotMC");
    println!("5: Minecraft Server for Bedrock Edition(Official)");
    println!("0: Other Server");
    let server_type = loop {
        match get_input().trim().parse::<usize>() {
            Ok(v)=>{
                break loop {
                    match v {
                        1 => break ServerType::OfficialJE,
                        2 => break ServerType::Paper,
                        3 => break ServerType::Purpur,
                        4 => break ServerType::Spigot,
                        5 => break ServerType::OfficialBE,
                        0 => break ServerType::Other,
                        _ => println!("Please select within the range.")
                    }
                }
            },
            Err(_)=>println!("Please enter a number.")
        }
    };

    let server_execute = match server_type {
        ServerType::Other=>{
            println!("Enter the name of the executable file");
            get_input()
        },
        ServerType::OfficialBE=>"server".to_string(),
        _ => "server.jar".to_string()
    };

    println!("Set the game version. The default is the latest version. The format is x.x.x");
    let server_version = loop {
        match Version::from_str(&get_input()) {
            Ok(v)=>break v,
            Err(e)=> println!("{}",e)
        }
    };

    let server_birthday = chrono::Utc::now().to_rfc3339();

    let project = Project{
        name:project_name,
        server_type,
        execute:server_execute,
        version:server_version.to_string(),
        birthday:server_birthday
    };

    let runtime_java = Java{
        version:21,
        edition:JavaType::GraalVM,
        arguments:vec![],
        custom:String::new(),
        xms:0,
        xmx:0
    };

    let runtime = Runtime{
        java:runtime_java
    };

    let backup_time=Time{
        interval:0,
        cron:String::new()
    };

    let backup_event=Event{
        start:false,
        stop:true,
        update:true
    };

    let backup = Backup{
        enable : true,
        world:true,
        other:true,
        time: Option::from(backup_time),
        event: Option::from(backup_event)
    };

    let plugin_manage = PluginManage{
        manage : true,
        plugin:Vec::new()
    };

    Config{
        project,
        runtime,
        backup,
        plugin_manage
    }
}

fn create_project(){
    match get_info() {
        Ok(_)=>println!("{}","The project has been created!".yellow()),
        Err(e) => {
            match e {
                NotValid::ConfigBroken=>broken_config(),
                NotValid::NotConfigured=>{
                    let config = get_config();
                    match config.to_file(Path::new("NMSL.toml")){
                        Ok(_)=>(),
                        Err(_)=>panic!("The configuration file cannot be created!")
                    }
                    match fs::create_dir(".nmsl") {
                        Ok(_)=>(),
                        Err(_)=>panic!("Directory cannot be created!")
                    }
                }
            }
        }
    }
}
