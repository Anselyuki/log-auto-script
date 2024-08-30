use crate::command::{AutoLoggerArgs, SubCommands};
use clap::Parser;
use log::{info, LevelFilter};
use log_auto_script_lib::clients::qian_wen_model::QianWenAdapter;
use log_auto_script_lib::clients::{ModelAdapterEnum, Prompt};
use log_auto_script_lib::config::consts::PROFILE;
use log_auto_script_lib::config::Profile;
use log_auto_script_lib::repository;
use log_auto_script_lib::utils::simple_log;
use std::collections::HashSet;
use std::process::exit;

fn main() {
    simple_log::init(LevelFilter::Info).unwrap();
    let args = AutoLoggerArgs::parse();
    match args.command {
        Some(SubCommands::Config { default }) => {
            if default {
                Profile::create_default()
            }
            Profile::open_config();
            exit(0);
        },
        None => {}
    }
    let year = args.year;
    let month = args.month;
    let day = args.day;

    // TODO 前端入参：git仓库地址列表
    let git_dirs = &PROFILE.repo_path;
    // TODO 前端入参: 分支名称
    let branches = &PROFILE.branches;
    // TODO 前端入参: 作者列表
    let authors = &PROFILE.authors;

    let mut logs: HashSet<String> = HashSet::new();
    // 遍历仓库地址列表,这里都是本地操作,所以不需要并发
    for git_dir in git_dirs {
        for branch in branches {
            for author in authors {
                let log = repository::build_log_vec(git_dir, branch, year, month, day, &author);
                logs.extend(log);
            }
        }
    }
    let log_vec = repository::log_distant(&mut logs);

    // 打印日志
    for log in &log_vec {
        info!("{}", log);
    }
    if log_vec.is_empty() {
        println!("没有找到提交记录");
        return;
    }
    let platform = ModelAdapterEnum::QianWen;
    let adapter: Box<dyn Prompt> = match platform {
        ModelAdapterEnum::QianWen => Box::new(QianWenAdapter::new()),
    };

    let message = adapter.get_message(year, month, day, log_vec);
    println!("{}", message.unwrap());
}

/// 解析命令行参数
///
mod command {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(name = "log-auto-script", bin_name = "log-auto-script")]
    #[command(author, about, version, next_line_help = false)]
    pub struct AutoLoggerArgs {
        pub author: Option<String>,
        #[arg(short, long)]
        pub year: Option<i32>,
        #[arg(short, long)]
        pub month: Option<u32>,
        #[arg(short, long)]
        pub day: Option<u32>,
        // 子命令枚举
        #[command(subcommand)]
        pub command: Option<SubCommands>,
    }

    #[derive(Subcommand)]
    pub enum SubCommands {
        /// 配置文件管理,详情请运行 config --help
        Config {
            /// 在配置文件夹内创建默认配置文件
            #[arg(short, long)]
            default: bool,
        },
    }
}
