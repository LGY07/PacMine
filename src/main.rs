mod project_manager;

use crate::project_manager::{create_project, get_info, print_info};
use clap::{Parser, Subcommand};
use colored::Colorize;
use log::{LevelFilter, error};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the project information of the current location
    Info,
    /// Create a project in a new directory
    New {
        /// The path of the new directory
        path: PathBuf,
    },
    /// Create a project at the current location
    Init,
    /// Install the necessary files to make the project run properly
    Install,
    /// Update the plugins
    Update {
        /// Automatically confirm for update
        #[arg(short, long)]
        yes: bool,
    },
    /// Upgrade the server core
    Upgrade,
    /// Run the daemon process
    Daemon {
        /// Specify the location of the configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
        /// Automatically configure as a systemd service
        #[arg(long)]
        install_systemd: bool,
        /// Automatically configure as a OpenRC service
        #[arg(long)]
        install_openrc: bool,
    },
}

fn main() {
    // 启用日志输出
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    // 解析参数
    let cli = Cli::parse();

    // info 子命令，查看当前项目的信息
    if let Commands::Info = &cli.command {
        print_info()
    }

    // new 子命令，根据传入的地址创建目录并初始化项目
    if let Commands::New { path } = &cli.command {
        // 创建目录
        fs::create_dir(path).unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", "Failed to create the directory!".red());
        });
        // 进入目录
        std::env::set_current_dir(path)
            .unwrap_or_else(|_| panic!("{}", "The directory cannot be opened!".red()));
        // 初始化项目
        create_project()
    }

    // init 子命令，初始化当前目录
    if let Commands::Init = &cli.command {
        // 初始化项目
        create_project()
    }
}
