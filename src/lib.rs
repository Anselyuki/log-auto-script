//! 常用大模型问答封装及读取 Git 仓库提交信息的工具

/// 用于执行大模型生成的主要功能
pub mod clients;

/// 主要的工具类
pub mod utils;

pub mod config;
/// 解析本地 Git 仓库
pub mod repository;
