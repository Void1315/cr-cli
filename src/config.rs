use std::error::Error;

use cfg::{Config, File};

const CONFIG_FILE: &str = "config.toml";

pub fn init_config() -> Result<Config, Box<dyn Error>>{
    Ok(Config::builder().add_source(File::with_name(CONFIG_FILE)).build()?)
}