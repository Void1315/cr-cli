use cfg::Value;
use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;

use crate::config;

// new 命令创建一个工作目录

const TABLE_NAME: &str = "new";

#[derive(Parser, Debug)]
pub struct New {
    #[arg(long)]
    pub course_name: Option<String>,
    #[arg(long)]
    pub courses_number: Option<u32>,
    #[arg(long)]
    pub note_name: Option<String>,
    #[arg(long)]
    pub workspace: Option<String>,
}

// 实现迭代器
impl IntoIterator for &New {
    type Item = (String, Value);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        if let Some(course_name) = &self.course_name {
            vec.push(("course_name".to_string(), Value::from(course_name.clone())));
        }
        if let Some(courses_number) = &self.courses_number {
            vec.push(("courses_number".to_string(), Value::from(*courses_number)));
        }
        if let Some(note_name) = &self.note_name {
            vec.push(("note_name".to_string(), Value::from(note_name.clone())));
        }
        if let Some(workspace) = &self.workspace {
            vec.push(("workspace".to_string(), Value::from(workspace.clone())));
        }
        vec.into_iter()
    }
}

// 通用性操作 TODO: 提出一个trait
impl New {
    pub fn run(&self, config_obj: &cfg::Config) {
        let filed_map = self.parse_field(&config_obj);
        // TODO: 业务逻辑
        self.create_project(&filed_map);
        dbg!(filed_map);
    }

    fn parse_field(&self, config_obj: &cfg::Config) -> HashMap<String, Value> {
        let mut global_filed_map = super::get_global_filed_map(config_obj);

        let filed_map = self.get_filed_map(config_obj);
        // filed_map中的字段覆盖global_filed_map中的字段
        global_filed_map.extend(filed_map);

        // 遍历self的字段 如果有值则覆盖global_filed_map
        for (key, value) in self.into_iter() {
            global_filed_map.insert(key, value);
        }

        global_filed_map
    }

    fn get_filed_map(&self, config_obj: &cfg::Config) -> HashMap<String, Value> {
        match config_obj.get_table(TABLE_NAME) {
            Ok(table) => table,
            Err(err) => {
                eprintln!("Error 解析{TABLE_NAME}配置文件错误: {:?}", err);
                std::process::exit(1);
            }
        }
    }
}

// 业务型操作
#[allow(dead_code)]
impl New {
    /// 在工作目录中创建一个项目
    /// 若工作目录不存在则创建
    fn create_project(&self, field_map: &HashMap<String, Value>) {
        let workspace = field_map.get("workspace").unwrap().clone().into_string().unwrap();
        // 检查工作目录是否存在
        if !std::path::Path::new(&workspace).exists() {
            println!("创建工作目录: {}", &workspace.green());
            match std::fs::create_dir_all(&workspace) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Error 创建工作目录失败: {:?}", err);
                }
            }
        }
        // 创建项目
        let project_path = self.add_project_dir(&workspace, &field_map);
        // 添加笔记
        self.add_note_file(&project_path);
    }

    /// 创建项目文件夹
    fn add_project_dir(&self, workspace: &str, field_map: &HashMap<String, Value>) -> String {
        let project_name = self.get_project_name(&field_map);
        let project_path = format!("{}/{}", workspace, project_name);
        // 判断是否存在项目文件夹
        if std::path::Path::new(&project_path).exists() {
            println!(
                "{} 项目文件夹已存在: {}",
                "Warning".yellow(),
                project_path.yellow()
            );
            return project_path;
        }

        println!("创建项目: {}", project_path.green());
        match std::fs::create_dir_all(&project_path) {
            Ok(_) => {
                // 更新配置中的课程数

            }
            Err(err) => {
                panic!("Error 创建项目失败: {:?}", err);
            }
        }
        project_path
    }

    /// 添加笔记文件
    fn add_note_file(&self, project_path: &str) {
        let note_name = self.note_name.as_ref().unwrap();
        let note_path = format!("{}/{}", project_path, note_name);
        // 判断笔记是否存在
        if std::path::Path::new(&note_path).exists() {
            println!(
                "{} 笔记文件已存在: {}",
                "Warning".yellow(),
                note_path.yellow()
            );
            return;
        }
        println!("创建笔记: {}", note_path.green());
        match std::fs::write(&note_path, "") {
            Ok(_) => {}
            Err(err) => {
                panic!("Error 创建笔记失败: {:?}", err);
            }
        }
    }
    fn update_config_courses_number(field_map: &mut HashMap<String, Value>, config_obj: &cfg::Config){
        let mut courses_number = field_map.get("courses_number").unwrap().clone().into_uint().unwrap();
        courses_number = courses_number + 1;
        field_map.insert("courses_number".to_string(), Value::from(courses_number));
        
        // config_obj.set(TABLE_NAME, field_map);
        // config_obj.
    }

    /// 获取项目文件夹名称
    fn get_project_name(&self, field_map: &HashMap<String, Value>) -> String {
        let course_name = field_map.get("course_name").unwrap().clone().into_string().unwrap();
        let mut courses_number = field_map.get("courses_number").unwrap().clone().into_uint().unwrap();
        courses_number = courses_number + 1;
        format!("{}-{}", courses_number, course_name)
    }
}
