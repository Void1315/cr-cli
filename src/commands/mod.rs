use toml::Table;

pub mod new;
pub mod zip;
pub mod mail;
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

pub trait MyCommand
where
    Self: IntoIterator<Item = (String, toml::Value)> + Clone + Sized,
    Self: Copy,
{
    fn run(&self, config_obj: &toml::Table);

    fn get_global_filed_map(&self, config_obj: &Table) -> Table;

    fn parse_field(&self, config_obj: &Table) -> Table {
        let mut global_filed_map = self.get_global_filed_map(config_obj);
        let filed_map = self.get_filed_map(config_obj);
        // filed_map中的字段覆盖global_filed_map中的字段
        global_filed_map.extend(filed_map);
        // 遍历self的字段 如果有值则覆盖global_filed_map
        for (key, value) in self.into_iter() {
            global_filed_map.insert(key, value);
        }
        global_filed_map
    }
    fn get_filed_map(&self, config_obj: &Table) -> Table;
}
