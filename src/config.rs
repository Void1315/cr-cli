use std::{error::Error, fs, io::Read};
use toml::Table;

const CONFIG_FILE: &str = "config.toml";

pub fn init_config() -> Result<Table, Box<dyn Error>> {
    // TODO: 要从exe运行目录下读取配置文件
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap();
    let config_file = exe_dir.join(CONFIG_FILE);
    if !config_file.exists() {
        panic!("配置文件不存在, 请创建配置文件: {}", config_file.display())
    }
    let mut file_handel = fs::File::open(config_file)?;
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
