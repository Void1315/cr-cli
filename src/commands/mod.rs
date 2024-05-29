use std::collections::HashMap;

use cfg::Value;

pub mod new;

const GLOBAL_TABLE_NAME: &str = "global";

pub fn get_global_filed_map(config_obj: &cfg::Config) -> HashMap<String, Value> {
    match config_obj.get_table(GLOBAL_TABLE_NAME) {
        Ok(table) => table,
        Err(err) => {
            eprintln!("Error 解析{GLOBAL_TABLE_NAME}配置文件错误: {:?}", err);
            std::process::exit(1);
        }
    }
}
