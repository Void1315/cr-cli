use toml::Table;

pub mod new;

const GLOBAL_TABLE_NAME: &str = "global";

pub fn get_global_filed_map(config_obj: &toml::Table) -> Table {
    match config_obj.get(GLOBAL_TABLE_NAME) {
        Some(table) => {
            let table = table.as_table().unwrap();
            table.clone()
        }
        None => Table::new(),
    }
}
