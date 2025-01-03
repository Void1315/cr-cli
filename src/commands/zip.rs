use std::{io::{Read, Write}, path::Path};

use clap::Parser;
use colored::Colorize;
use toml::Table;
use walkdir::WalkDir;
use zip::ZipWriter;

use crate::config::get_default_zip_file_name;

use super::MyCommand;

const TABLE_NAME: &str = "zip";

#[derive(Parser, Debug)]
/// 关于压缩的命令
/// 命令可以帮你压缩文件夹 并生成默认班级格式的压缩文件
pub struct Zip {
    #[arg(long, short)]
    /// 需要忽略的文件夹名称，例如输入: .git .vs Debug
    /// 将会在进行压缩时忽略这些文件夹
    pub ignore: Option<Vec<String>>,
    #[arg(long, short)]
    /// 需要压缩的文件夹路径,例如: /home/username/workspace
    /// 将会递归的压缩这个文件夹，生成一个压缩文件
    pub dir_path: String,
}

impl MyCommand for &Zip {
    fn run(&self, config_obj: &toml::Table) {
        let filed_map = self.parse_field(config_obj);
        self.zip(&filed_map, config_obj);
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

impl IntoIterator for &Zip {
    type Item = (String, toml::Value);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        if let Some(ignore) = &self.ignore {
            vec.push((
                "ignore".to_string(),
                toml::Value::Array(
                    ignore
                        .iter()
                        .map(|s| toml::Value::String(s.clone()))
                        .collect(),
                ),
            ));
        }
        vec.push(("dir_path".to_string(), toml::Value::String(self.dir_path.clone())));

        vec.into_iter()
    }
}

// 业务逻辑
impl Zip {
    pub fn _zip(
        dir_path_str: &str,
        ignore_dir: &mut Vec<String>,
        file_name_str: &str,
        config_obj: &Table,
    ) {
        let mut dir_path = Path::new(dir_path_str).to_owned();
        let current_dir = std::env::current_dir().unwrap();
        if dir_path.is_relative() {
            // 获取当前命令行所在路径
            dir_path = current_dir.join(dir_path);
        }
        if !dir_path.is_dir() {
            // 带颜色打印eprintfln
            eprintln!(
                "{} {} {}",
                "Error: ".red(),
                dir_path_str,
                "不是一个文件夹".red()
            );
            std::process::exit(1);
        }
        // 3. 复制文件
        let zip_file = std::fs::File::create(current_dir.join(&file_name_str)).unwrap();
        let mut zip_writer = ZipWriter::new(zip_file);
        let mut options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        // 如果config中含有密码，则使用
        let zip_table = config_obj.get("zip").unwrap().as_table().unwrap();
        if let Some(password) = zip_table.get("password") {
            // 如果密码存在
            let password = password.as_str().unwrap();
            if !password.is_empty() {
                // 如果不是空字符串
                options = options.with_aes_encryption(zip::AesMode::Aes128, password);
            }
        }
        // 遍历当前文件夹下所有文件
        let it = WalkDir::new(&dir_path).into_iter();
        let mut buf = Vec::new();

        for entry in it {
            let entry = entry.unwrap();
            let path = entry.path();
            // 去除前缀
            let strip_prefix_str = path.strip_prefix(&dir_path).unwrap().to_str().unwrap();
            if strip_prefix_str == file_name_str {
                continue;
            }
            // 通过路径分隔符分割 判断是否在忽略列表中
            let path_vec: Vec<&str> = strip_prefix_str.split(std::path::MAIN_SEPARATOR).collect();
            let mut is_ignore = false;
            for the_path_str in path_vec {
                if ignore_dir.contains(&the_path_str.to_string()) {
                    is_ignore = true;
                    break;
                }
            }
            if is_ignore {
                continue;
            }
            let unix_style_path = strip_prefix_str.replace("\\", "/");
            if path.is_dir() {
                zip_writer.add_directory(unix_style_path, options).unwrap();
            }else{
                zip_writer.start_file_from_path(unix_style_path, options).unwrap();
                let mut file = std::fs::File::open(path).unwrap();
                file.read_to_end(&mut buf).unwrap();
                zip_writer.write_all(&buf).unwrap();
                buf.clear();
            }
        }
        zip_writer.finish().unwrap();

        // 6. 打印压缩文件信息
        let zip_file = current_dir.join(&file_name_str);
        let zip_info = zip_file.metadata().unwrap();
        println!("压缩文件路径: {}", zip_file.display());
        println!("压缩文件大小: {}KB", zip_info.len() / 1024);
    }

    fn zip(&self, filed_map: &Table, config_obj: &Table) {
        let dir_path_str = filed_map.get("dir_path").unwrap().as_str().unwrap();
        let mut ignore_dir = filed_map
            .get("ignore")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        let file_name_str = get_default_zip_file_name(config_obj);
        Zip::_zip(dir_path_str, &mut ignore_dir, &file_name_str, &config_obj)
    }
}
