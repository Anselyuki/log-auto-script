use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;

use lazy_static::lazy_static;
use log::{error, info, warn};
use platform_dirs::AppDirs;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod consts {
    extern crate lazy_static;
    use crate::config::Profile;
    use lazy_static::lazy_static;
    use platform_dirs::AppDirs;
    use std::path::PathBuf;

    lazy_static! {
        // 在 MacOS下遵守 XDG 规范,即创建的配置文件夹为 `~/.config/log-auto-script`
        pub static ref CONFIG_PATH: PathBuf = AppDirs::new(Some("log-auto-script"), true).unwrap().config_dir;
        pub static ref PROFILE: Profile = Profile::new();
    }
}

/// 配置文件解析结果
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub repo_path: Vec<String>,
    pub branches: Vec<String>,
    pub authors: Vec<String>,
    pub authorization: Option<String>,
}

impl Profile {
    /// 创建默认配置文件
    ///
    /// 详细的默认配置文件可以参考私有方法:`Profile::default_profile()`
    pub fn create_default() {
        let path = consts::CONFIG_PATH.as_path();
        Self::create_dir(path);
        let path = &consts::CONFIG_PATH.join("config.yml");
        dbg!(path);
        // 将 profile 序列化为 YAML 字符串
        let yaml = serde_yaml::to_string(&Self::default_profile()).unwrap();
        dbg!(yaml.clone());
        // 打开文件并写入 yaml 字符串
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(e) => {
                error!("无法创建文件: {:?}", e);
                exit(exitcode::IOERR);
            }
        };
        match file.write_all(yaml.as_bytes()) {
            Ok(_) => {
                info!("已成功创建配置文件:{}", path.display());
            }
            Err(e) => {
                error!("无法写入文件{:?}", e);
                exit(exitcode::IOERR);
            }
        }
    }

    /// 如果路径不存在则创建
    pub fn create_dir(path: &Path) {
        if !path.exists() {
            if let Err(error) = fs::create_dir_all(path) {
                error!("创建文件/文件夹失败!\n[Cause]:{:?}", error)
            }
        }
    }

    pub fn open_config() {
        let path = &consts::CONFIG_PATH.join("config.yml");
        if !path.exists() {
            info!("不存在已有的配置文件,请使用 config --detail(-d) 标志来创建默认配置文件");
            exit(exitcode::OK)
        }
        match open::that(path) {
            Ok(_) => {
                info!("已成功打开配置文件:{}", path.display());
            }
            Err(e) => {
                error!("无法打开文件{:?}", e);
                exit(exitcode::IOERR);
            }
        }
    }

    /// 加载配置文件,默认配置文件为`config.yml`
    ///
    /// > 如果想要创建默认配置文件,请使用`Profile::create_default()`方法
    ///
    /// - 不会抛出异常,最坏的情况下也会返回默认配置文件
    /// - 如果指定的配置文件不存在或解析失败,会产生警告信息提示配置文件配置不正确
    pub fn new() -> Profile
    where
        Profile: DeserializeOwned,
    {
        let path = &consts::CONFIG_PATH.join("config.yml");
        if !path.exists() {
            return Self::default_profile();
        }

        // 通过 std::fs 读取配置文件内容,解析失败也返回默认配置文件
        let yaml_value = match std::fs::read_to_string(path) {
            Ok(file_str) => file_str,
            Err(error) => return Self::error_handler(error.to_string()),
        };
        serde_yaml::from_str(&yaml_value)
            .unwrap_or_else(|error| Self::error_handler(error.to_string()))
    }

    /// 处理失败处理,返回默认配置文件
    fn error_handler(error: String) -> Profile {
        warn!("配置文件解析失败,使用默认值\n[Cause]: {}", error);
        info!("因为懒的问题没有配置跳过空字段,所以请在默认配置文件基础上修改喵: (config --default 生成默认配置文件)");
        Self::default_profile()
    }

    /// 默认配置文件的字段
    fn default_profile() -> Profile {
        Profile {
            repo_path: vec!["/path/to/your/repo".to_string()],
            branches: vec!["master".to_string()],
            authors: vec!["your_name".to_string()],
            authorization: Some("Bearer your_token".to_string()),
        }
    }
}
