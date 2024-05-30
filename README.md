# `CR-CLI`

## 使用说明

1. 将程序下载，并将程序目录添加到变量中。
2. 将程序目录中的`config.example.toml`文件，重命名为`config.toml`。
3. 按照参数说明修改`config.toml`文件中的参数。
4. enjoy

## 配置文件

**`[global]`**

| 参数字段名称 | 类型     | 说明     |
| ------------ | -------- | -------- |
| `user_name`  | `String` | 人员名称 |
| `class_name` | `String` | 班级名称 |
|              |          |          |



**`[new]`**

| 参数字段名称     | 类型     | 说明                                                         |
| ---------------- | -------- | ------------------------------------------------------------ |
| `course_name`    | `String` | 本次课程内容名称的默认名称                                   |
| `courses_number` | `Int`    | 课程序号 将会创建[courses_number-course_name]的文件夹        |
| `note_name`      | `String` | 笔记文件的默认名称                                           |
| `workspace`      | `String` | 工作目录 将会在此目录下创建新的文件夹(windows路径最好使用单引号) |

**`[zip]`**

| 参数字段名称 | 类型       | 说明                                               |
| ------------ | ---------- | -------------------------------------------------- |
| `dir_path`   | `String`   | 需要压缩的文件夹的路径 `.`将会打包此路径下所有文件 |
| `ignore`     | `[String]` | 压缩时忽略的文件夹名称列表                         |



**`[mail]`**

> 不使用`mail -s`发送邮件可以不填写`smtp`相关字段

| 参数字段名称  | 类型       | 说明                       |
| ------------- | ---------- | -------------------------- |
| `email`       | `String`   | 发送邮件的邮箱地址         |
| `password`    | `[String]` | 发送邮件的邮箱密码         |
| `smtp_server` | `String`   | 发送邮件的`smtp`服务器地址 |
| `smtp_port`   | `Int`      | 发送邮件的`smtp`服务器端口 |
| `receiver`    | `String`   | 接收邮件的邮箱地址         |
|               |            |                            |



##  `New`命令

### 参数

```shell
将会创建一个新的项目目录。使用以下参数进行定制
Usage: cr-cli.exe new [OPTIONS]

Options:
      --course-name <COURSE_NAME>
          课程名称，文件夹名称一部分。例如： 虚基类
      --courses-number <COURSES_NUMBER>
          课程序号，文件夹名称的一部分。例如: 12
      --note-name <NOTE_NAME>
          笔记文件名称，包含文件后缀。例如: 笔记.md
  -w, --workspace <WORKSPACE>
          工作目录文件夹路径，将会在此目录中创建新的项目，例如: /home/username/workspace。
```



### 建立新的课程

```shell
# 配置好你的配置文件
cd your-workspace
cr-cli.exe new
```

## `Zip`命令

### 参数

```shell
关于压缩的命令 命令可以帮你压缩文件夹 并生成默认班级格式的压缩文件

Usage: cr-cli.exe zip [OPTIONS] --dir-path <DIR_PATH>

Options:
  -i, --ignore <IGNORE>      需要忽略的文件夹名称，例如输入: .git .vs Debug 将会在进行压缩时忽略这些文件夹
  -d, --dir-path <DIR_PATH>  必填参数！需要压缩的文件夹路径,例如: /home/username/workspace 将会递归的压缩这个文件夹，生成一个压缩文件
  -h, --help                 Print help
  -V, --version              Print version
```



### 压缩一个当前路径的所有文件

```shell
cd your-dir
cr-cli.exe zip -d ./
# 生成一个符合格式的7z文件
```



## `Mail`命令

### 参数

```shell
发送邮件的命令 可以生成本地邮件文件 和发送邮件，并支持自动压缩,自动发送

Usage: cr-cli.exe mail [OPTIONS]

Options:
  -s, --send                     是否发送邮件
  -a, --auto                     自动打包，自动发送，一键完成
  -f, --attachment <ATTACHMENT>  附件路径 可选
  -o, --output <OUTPUT>          将邮件生成的原始信息输出到文件
  -h, --help                     Print help
  -V, --version                  Print version
```

### 一键打包 一键发送

```shell
# 配置好你的smtp参数
cr-cli mail -a

```

### 发送并保存原始邮件到文件

```shell
cr-cli mail -a -o mail.eml
```

### 仅生成原始邮件

```shell
cr-cli mail -o mail.eml
```

