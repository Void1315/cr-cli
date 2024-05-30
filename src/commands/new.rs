use std::path::Path;

use clap::Parser;
use colored::Colorize;
use toml::{Table, Value};

use crate::config::update_config_file;

use super::MyCommand;

// new 命令创建一个工作目录

const TABLE_NAME: &str = "new";

#[derive(Parser, Debug)]
/// 将会创建一个新的项目目录。使用以下参数进行定制
pub struct New {
    #[arg(long)]
    /// 课程名称，文件夹名称一部分。例如： 虚基类
    pub course_name: Option<String>,
    #[arg(long)]
    /// 课程序号，文件夹名称的一部分。例如: 12
    pub courses_number: Option<u32>,
    #[arg(long)]
    /// 笔记文件名称，包含文件后缀。例如: 笔记.md
    pub note_name: Option<String>,
    #[arg(long, short = 'w')]
    /// 工作目录文件夹路径，将会在此目录中创建新的项目，例如: /home/username/workspace。
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

impl MyCommand for &New {
    fn run(&self, config_obj: &toml::Table) {
        let filed_map = self.parse_field(config_obj);
        self.create_project(&filed_map, config_obj);
    }
    fn get_global_filed_map(&self, config_obj: &Table) -> Table {
        super::get_global_filed_map(config_obj)
    }

    fn get_filed_map(&self, config_obj: &Table) -> Table {
        match config_obj.get(TABLE_NAME) {
            Some(table) => {
                let table = table.as_table().unwrap();
                table.clone()
            }
            None => Table::new(),
        }
    }
}

// 业务型操作
#[allow(dead_code)]
impl New {
    /// 在工作目录中创建一个项目
    /// 若工作目录不存在则创建
    fn create_project(&self, field_map: &Table, config_obj: &Table) {
        let workspace = field_map.get("workspace").unwrap().as_str().unwrap();

        // 检查工作目录是否存在
        if !Path::new(workspace).exists() {
            println!("创建工作目录: {}", workspace.green());
            match std::fs::create_dir_all(workspace) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Error 创建工作目录失败: {:?}", err);
                }
            }
        }
        // 创建项目
        let project_path = self.add_project_dir(workspace, field_map, config_obj);
        // 添加笔记
        self.add_note_file(&project_path, field_map);
    }

    /// 创建项目文件夹
    fn add_project_dir(&self, workspace: &str, field_map: &Table, config_obj: &Table) -> String {
        let project_name = self.get_project_name(field_map);
        let project_path = Path::new(workspace).join(project_name);
        let project_path_str = project_path.to_str().unwrap();
        // 判断是否存在项目文件夹
        if std::path::Path::new(&project_path).exists() {
            println!(
                "{} 项目文件夹已存在: {}",
                "Warning".yellow(),
                project_path_str.yellow()
            );
            self.update_config_courses_number(field_map, config_obj); // 存在的项目 也要更新配置中的课程数
            return project_path_str.to_string();
        }

        println!("创建项目: {}", project_path_str.green());
        match std::fs::create_dir_all(&project_path) {
            Ok(_) => {
                // 更新配置中的课程数
                self.update_config_courses_number(field_map, config_obj);
            }
            Err(err) => {
                panic!("Error 创建项目失败: {:?}", err);
            }
        }
        project_path_str.to_string()
    }

    /// 添加笔记文件
    fn add_note_file(&self, project_path: &str, field_map: &Table) {
        let note_name = field_map.get("note_name").unwrap().as_str().unwrap();
        // 路径不能这样拼接
        let note_path = Path::new(project_path).join(note_name);
        let note_path_str = note_path.to_str().unwrap();
        // 判断笔记是否存在
        if note_path.exists() {
            println!(
                "{} 笔记文件已存在: {}",
                "Warning".yellow(),
                note_path_str.yellow()
            );
            return;
        }
        println!("创建笔记: {}", note_path_str.green());
        match std::fs::write(&note_path, "") {
            Ok(_) => {}
            Err(err) => {
                panic!("Error 创建笔记失败: {:?}", err);
            }
        }
    }
    fn update_config_courses_number(&self, field_map: &Table, config_obj: &Table) {
        let mut config_obj = config_obj.clone();
        let mut courses_number = field_map
            .get("courses_number")
            .unwrap()
            .as_integer()
            .unwrap();
        courses_number += 1;
        let mut new_filed_map = config_obj
            .get(TABLE_NAME)
            .unwrap()
            .as_table()
            .unwrap()
            .clone();
        new_filed_map.insert("courses_number".to_string(), Value::from(courses_number));
        config_obj.insert(TABLE_NAME.to_string(), Value::from(new_filed_map));
        println!("{:}", config_obj);
        // 更新配置文件
        match update_config_file(&config_obj) {
            Ok(_) => {}
            Err(err) => {
                // 按理来说不应该出现这种情况
                println!("{} {}", "Error 更新配置文件失败".red(), err);
                println!("{} {}", "更新 courses_number 字段失败，这会导致下次创建错误的项目文件夹，请手动更新courses_number字段到新的值: ".yellow(), courses_number);
            }
        }
    }
    /// 获取项目文件夹名称
    fn get_project_name(&self, field_map: &Table) -> String {
        let course_name = field_map.get("course_name").unwrap().as_str().unwrap();
        let courses_number = field_map
            .get("courses_number")
            .unwrap()
            .as_integer()
            .unwrap();
        format!("{}-{}", courses_number, course_name)
    }
}
