use std::{fs, io::Write, path::Path};

use crate::config::get_default_zip_file_name;
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use lettre::message::{header, Body, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use super::zip::Zip;
use super::MyCommand;

const TABLE_NAME: &str = "mail";

#[derive(Parser, Debug)]
/// 发送邮件的命令
/// 可以生成本地邮件文件 和发送邮件，并支持自动压缩,自动发送
pub struct Mail {
    #[arg(long, short)]
    /// 是否发送邮件
    pub send: bool,
    #[arg(long, short)]
    /// 自动打包，自动发送，一键完成
    pub auto: bool,
    #[arg(long, short = 'f')]
    /// 附件路径 可选
    pub attachment: Option<String>,
    #[arg(long, short)]
    /// 将邮件生成的原始信息输出到文件
    pub output: Option<String>,
}

impl IntoIterator for &Mail {
    type Item = (String, toml::Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, toml::Value>;

    fn into_iter(self) -> Self::IntoIter {
        let mut map = std::collections::HashMap::new();
        map.insert("send".to_string(), toml::Value::Boolean(self.send));
        if let Some(attachment) = &self.attachment {
            map.insert(
                "attachment".to_string(),
                toml::Value::String(attachment.clone()),
            );
        }
        if let Some(output) = &self.output {
            map.insert("output".to_string(), toml::Value::String(output.clone()));
        }
        map.insert("auto".to_string(), toml::Value::Boolean(self.auto));
        map.into_iter()
    }
}

impl MyCommand for &Mail {
    fn run(&self, config_obj: &toml::Table) {
        let filed_map = self.parse_field(config_obj);
        // block_on(self.send(&filed_map, config_obj));
        self.send_with_lettre(&filed_map, config_obj);
    }

    fn get_global_filed_map(&self, config_obj: &toml::Table) -> toml::Table {
        super::get_global_filed_map(config_obj)
    }

    fn get_filed_map(&self, config_obj: &toml::Table) -> toml::Table {
        match config_obj.get(TABLE_NAME) {
            Some(table) => {
                let table = table.as_table().unwrap();
                table.clone()
            }
            None => toml::Table::new(),
        }
    }
}

// 纯纯的业务逻辑
impl Mail {
    // 使用lettre进行邮件构造和发送
    fn send_with_lettre(&self, field_map: &toml::Table, config_obj: &toml::Table) {
        let message = self.build_message_with_lettre(field_map, config_obj);
        let mailer = self.build_conntent_with_lettre(field_map);
        // 如果需要写入文件 在此时写入
        match field_map.get("output") {
            None => {}
            Some(output) => {
                println!("{} {output}", "输出到文件: ".blue());
                let message_data = message.formatted();
                self.output_to_file(output.as_str().unwrap(), &message_data);
            }
        }

        let is_send = match field_map.get("send") {
            None => false,
            Some(send) => send.as_bool().unwrap(),
        };
        if is_send {
            mailer.send(&message).unwrap();
            println!("{}", "发送邮件成功！".green());
        }
    }

    fn output_to_file(&self, output: &str, message_data: &[u8]) {
        let mut path = Path::new(output).to_owned();
        // 如果是相对路径 则拼接上当前路径
        if !path.is_absolute() {
            let current_path = std::env::current_dir().unwrap();
            path = current_path.join(output);
        }
        // 判断当前文件是否存在，如果存在 则替换 如果不存在则创建
        let mut file = fs::File::create(path).unwrap();
        file.write_all(message_data).unwrap();
    }

    fn build_conntent_with_lettre(&self, field_map: &toml::Table) -> SmtpTransport {
        let user_email_address = field_map.get("email").unwrap().as_str().unwrap();
        let user_password = field_map.get("password").unwrap().as_str().unwrap();
        let smtp_server = field_map.get("smtp_server").unwrap().as_str().unwrap();
        let smtp_port = field_map.get("smtp_port").unwrap().as_integer().unwrap() as u16;

        let creds = Credentials::new(user_email_address.into(), user_password.into());
        let mailer = SmtpTransport::relay(smtp_server)
            .unwrap()
            .port(smtp_port)
            .credentials(creds)
            .build();
        mailer
    }

    fn build_message_with_lettre(
        &self,
        field_map: &toml::Table,
        config_obj: &toml::Table,
    ) -> Message {
        let email_address = field_map.get("email").unwrap().as_str().unwrap();
        let receiver_address = field_map.get("receiver").unwrap().as_str().unwrap();
        // 判断是否含有自动打包的选项
        match field_map.get("auto") {
            Some(auto) => {
                if auto.as_bool().unwrap() {
                    let file_name_str = get_default_zip_file_name(config_obj);
                    let ignore_dir = config_obj.get("zip").unwrap().as_table().unwrap().clone();
                    let ignore_dir = ignore_dir
                        .get("ignore")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .clone();
                    let mut ignore_dir = ignore_dir
                        .iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect::<Vec<String>>();

                    Zip::_zip(".", &mut ignore_dir, &file_name_str, config_obj);
                }
            }
            None => {}
        }
        let user_name = field_map.get("user_name").unwrap().as_str().unwrap();
        let class_name = field_map.get("class_name").unwrap().as_str().unwrap();
        let time_str = chrono::Local::now().format("%Y%m%d").to_string();
        // match附件路径是否存在
        let attachment_path_str = match field_map.get("attachment") {
            Some(attachment) => attachment.as_str().unwrap().to_string(),
            None => {
                // 用户没有输入附件路径 使用默认的附件路径
                format!("{}_{}_{}.zip", class_name, user_name, time_str)
            }
        };
        // 当前命令行所在路径
        let current_path = std::env::current_dir().unwrap();
        let attachment_path = current_path.join(attachment_path_str);
        let attachment_name = attachment_path.file_name().unwrap().to_str().unwrap(); // 附件名称
        let subject = format!("{}_{}_{}", class_name, user_name, time_str);
        // 判断附件路径是否存在
        if !attachment_path.exists() {
            eprintln!("{} {:?}", "Error 附件路径不存在:".red(), attachment_path);
            std::process::exit(1);
        }
        // 读取附件内容
        let file_data = fs::read(&attachment_path).unwrap();
        let attachment_singpart: SinglePart = SinglePart::builder()
            .header(header::ContentType::TEXT_PLAIN)
            .header(lettre::message::header::ContentDisposition::attachment(
                attachment_name,
            ))
            .body(file_data);

        // 本地时间
        let systime = Local::now();
        let body = Body::new(format!("{}", systime.format("%Y-%m-%d %H:%M:%S")));
        let message = Message::builder()
            .from(email_address.parse().unwrap())
            .to(receiver_address.parse().unwrap())
            .date(systime.into())
            .subject(subject)
            .multipart(
                MultiPart::mixed().singlepart(
                    SinglePart::builder().content_type(header::ContentType::TEXT_PLAIN).body(body),
                ).singlepart(attachment_singpart)
            )
            .unwrap();

        message
    }
}
