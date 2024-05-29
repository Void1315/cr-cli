use clap::{Parser, Subcommand};
use commands::new::New;
use config::init_config;

mod commands;
mod config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    New(New),
}
fn main() {
    run();
}

fn config_init() -> toml::Table {
    // 1. 初始化配置
    let result = init_config();
    if let Ok(config_obj) = result {
        config_obj
    } else {
        // 打印错误
        eprintln!("Error 初始化配置错误: {:?}", result.err().unwrap());
        std::process::exit(1);
    }
}

fn run() {
    let config_obj = config_init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::New(new_obj) => {
            new_obj.run(&config_obj);
        }
    }
}
