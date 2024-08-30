use std::collections::HashSet;

use chrono::{Datelike, Days, Local, TimeZone};
use git2::{BranchType, Commit, Repository};

///
/// 构建 Git 提交日志
///
pub fn build_log_vec(
    git_dir: &str,
    branch_name: &str,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    author: &str,
) -> HashSet<String> {
    // 限制只显示本周以来的提交记录, 如果没有指定日期, 则默认为今天
    let day = day.unwrap_or(Local::now().day());
    let month = month.unwrap_or(Local::now().month());
    let year = year.unwrap_or(Local::now().year());
    // 今天的时间范围
    let until = Local
        .with_ymd_and_hms(year, month, day, 23, 59, 59)
        .unwrap();
    // 默认使用 5 天前的时间
    let since = until.checked_sub_days(Days::new(5)).unwrap();

    let repo = Repository::open(git_dir).expect("无法打开仓库");
    // 创建 RevWalk 对 Git 仓库执行操作
    let mut revwalk = repo.revwalk().expect("无法创建 RevWalk");

    // 获取分支
    let branch = repo
        .find_branch(branch_name, BranchType::Local)
        .or_else(|_| repo.find_branch(branch_name, BranchType::Remote))
        .expect("无法获取分支");

    revwalk
        .push(branch.get().target().unwrap())
        .expect("无法将分支加入RevWalk");
    revwalk
        .set_sorting(git2::Sort::TIME)
        .expect("无法设置排序方式");

    revwalk
        .filter_map(Result::ok)
        .filter_map(|oid| repo.find_commit(oid).ok())
        .filter(|commit| {
            let time = commit.time().seconds();
            // 过滤掉不符合条件的提交记录
            time > since.timestamp()
                && time < until.timestamp()
                && commit.author().name().eq(&Some(author))
                && !commit.parents().count().eq(&2)
        })
        // 通过 commit 构建提交信息
        .filter_map(|commit| build_commit_message(&commit, commit.time().seconds()))
        .collect()
}

/// # 构建提交信息
/// 日期格式: [2021-08-01 12:00] 提交信息
fn build_commit_message(commit: &Commit, timestamp: i64) -> Option<String> {
    let summary = commit.summary().expect("无法获取提交信息");
    let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
    Some(format!(
        "[{}] {}",
        datetime.format("%Y-%m-%d %H:%M"),
        summary
    ))
}

/// # 日志去重,重新排序
///
/// 将`HashSet`中的日志字符串转换为排序后的`Vec`日志字符串
///
/// 日志按时间戳降序排序
///
/// * `logs` - 一个包含日志字符串的`HashSet`的可变引用
pub fn log_distant(logs: &mut HashSet<String>) -> Vec<String> {
    let mut log_vec: Vec<String> = logs.iter().map(|s| s.to_owned()).collect::<Vec<String>>();
    log_vec.sort_by(|a, b| {
        // 由于日志字符串的格式固定为"[2021-08-01 12:00] 提交信息"，所以时间戳的起始位置为1，长度为17
        let timestamp_a = &a[1..17];
        let timestamp_b = &b[1..17];
        timestamp_b.cmp(timestamp_a)
    });
    log_vec.to_vec()
}
