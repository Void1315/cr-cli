use std::{error::Error, fs, io::Read};
use toml::Table;

const CONFIG_FILE: &str = "config.toml";

pub fn init_config() -> Result<Table, Box<dyn Error>> {
    let mut file_handel = fs::File::open(CONFIG_FILE)?;
    let mut content = String::new();
    file_handel.read_to_string(&mut content)?;
    let table: Table = toml::from_str(&content)?;
    Ok(table)
}

pub fn update_config_file(table: &Table) -> Result<(), Box<dyn Error>> {
    let content = toml::to_string(table)?;
    fs::write(CONFIG_FILE, content)?;
    Ok(())
}
