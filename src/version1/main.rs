use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Datelike, Duration, Local, Timelike, Weekday};
use clap::{Parser, ValueEnum};
use git2::{Commit, Repository, Signature};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};
use tabled::{Table, Tabled};
use walkdir::WalkDir;
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Git 用户名
    #[arg(short, long)]
    username: String,

    /// 统计时间范围
    #[arg(short = 't', long, value_enum, default_value_t = TimeRange::Year)]
    range: TimeRange,

    /// 搜索的根目录
    #[arg(short = 'd', long, default_value = ".")]
    root: PathBuf,

    /// 输出格式
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Debug)]
enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Table,
    Json,
}

#[derive(Debug, Default, Clone, Serialize)]
struct GitStats {
    total_commits: usize,
    repos: HashMap<String, RepoStats>,
    late_night_commits: usize,
    latest_commit_time: Option<DateTime<Local>>,
    commits_by_hour: [usize; 24],
    commits_by_weekday: [usize; 7],
    languages: HashMap<String, usize>,
}

#[derive(Debug, Default, Clone, Serialize)]
struct RepoStats {
    commits: usize,
    additions: usize,
    deletions: usize,
    files_changed: usize,
}

#[derive(Tabled)]
struct RepoTableRow {
    name: String,
    commits: usize,
    additions: usize,
    deletions: usize,
    files_changed: usize,
}

impl RepoStats {
    fn merge(&mut self, other: RepoStats) {
        self.commits += other.commits;
        self.additions += other.additions;
        self.deletions += other.deletions;
        self.files_changed += other.files_changed;
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let stats = collect_stats(&args)?;

    match args.format {
        OutputFormat::Table => print_table(&stats),
        OutputFormat::Json => print_json(&stats)?,
    }

    Ok(())
}

fn collect_stats(args: &Args) -> Result<GitStats> {
    let repos = find_git_repos(&args.root)?;
    let pb = ProgressBar::new(repos.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {msg}"
        )?
            .progress_chars("##-")
    );

    let stats = repos
        .par_iter()
        .filter_map(|repo_path| {
            let result  = process_repo(repo_path, &args.username, &args.range);
            pb.inc(1);
            pb.set_message(format!("Processing: {}", repo_path.display()));
            result.transpose()  // 将 Result<Option<T>> 转换为 Option<Result<T>>
        })
        .collect::<Result<Vec<GitStats>>>()?
        .into_iter()
        .fold(GitStats::default(), |mut acc, stats| {
            acc.total_commits += stats.total_commits;
            acc.late_night_commits += stats.late_night_commits;

            if stats.latest_commit_time > acc.latest_commit_time {
                acc.latest_commit_time = stats.latest_commit_time;
            }

            for (repo, repo_stats) in stats.repos {
                acc.repos.entry(repo).or_default().merge(repo_stats);
            }

            for (lang, count) in stats.languages {
                *acc.languages.entry(lang).or_default() += count;
            }

            for (i, &count) in stats.commits_by_hour.iter().enumerate() {
                acc.commits_by_hour[i] += count;
            }

            for (i, &count) in stats.commits_by_weekday.iter().enumerate() {
                acc.commits_by_weekday[i] += count;
            }

            acc
        });

    pb.finish_with_message(format!("🎉 完成 {} 个仓库的统计", repos.len()));
    Ok(stats)
}

fn find_git_repos(root: &Path) -> Result<Vec<PathBuf>> {
    let mut repos = Vec::new();
    println!("正在扫描目录: {}", root.display());
    let walker = WalkDir::new(root)
        .into_iter()
        // .filter_entry(|e| !is_hidden(e));
        .filter_entry(|e| {
            let is_hidden = is_hidden(e);
            if is_hidden {
                // println!("跳过隐藏项: {}", e.path().display());
            }
            !is_hidden
        });

    for entry in walker {
        let entry = entry?;
        // println!("检查: {}", entry.path().display()); // 添加调试输出
        if entry.file_name() == ".git" {
            let repo_path = entry.path().parent().unwrap().to_path_buf();
            println!("发现Git仓库: {}", repo_path.display()); // 添加调试输出
            repos.push(repo_path);
        }
    }

    Ok(repos)
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    // entry.file_name()
    //     .to_str()
    //     .map(|s| s.starts_with('.'))
    //     .unwrap_or(false)

    let name = entry.file_name().to_str().unwrap_or("");

    // 允许.git目录通过
    if name == ".git" {
        return false;
    }

    // 跳过特定系统目录
    let is_system_dir = name == "$RECYCLE.BIN"
        || name == "System Volume Information"
        || name == "Windows"
        || name == "tmp";

    // 跳过常见的构建目录和隐藏文件
    let is_common_skip = name.starts_with('.')
        || name == "target"
        || name == "node_modules"
        || name == "build"
        || name == "dist";

    is_system_dir || is_common_skip
}

// fn process_repo(
//     path: &Path,
//     username: &str,
//     range: &TimeRange,
// ) -> Result<GitStats> {
//     let repo = Repository::open(path)?;
//     let mut stats = GitStats::default();
//     let mut repo_stats = RepoStats::default();
//     let mut revwalk = repo.revwalk()?;
//
//     revwalk.push_head()?;
//
//     for oid in revwalk {
//         let oid = oid?;
//         let commit = repo.find_commit(oid)?;
//         let signature = commit.author();
//         if signature.name().map(|n| n == username).unwrap_or(false) {
//             process_commit(&repo, &commit, &mut stats, &mut repo_stats , range)?;
//         }
//     }
//
//     let repo_name = path.file_name()
//         .and_then(|n| n.to_str())
//         .unwrap_or("unknown")
//         .to_string();
//
//     stats.total_commits += repo_stats.commits;
//     stats.repos.insert(repo_name, repo_stats);
//     Ok(stats)
// }

// fn process_repo(
//     path: &Path,
//     username: &str,
//     range: &TimeRange,
// ) -> Result<GitStats> {
//     let repo = Repository::open(path)?;
//     let mut stats = GitStats::default();
//     let mut repo_stats = RepoStats::default();
//     let mut stats = GitStats::default();
//
//     // 获取所有分支
//     let branches = repo.branches(None)?;
//     let mut has_valid_branch = false;
//
//     for branch in branches {
//         let (branch, _) = branch?;
//         if let Some(commit) = branch.get().peel_to_commit().ok() {
//             has_valid_branch = true;
//             process_branch_commits(&repo, &commit, username, range, &mut stats)?;
//         }
//     }
//
//     // 如果没有分支但有提交（如分离HEAD状态）
//     if !has_valid_branch {
//         if let Ok(commit) = repo.head()?.peel_to_commit() {
//             process_branch_commits(&repo, &commit, username, range, &mut stats)?;
//         }
//     }
//
//     let repo_name = path.file_name()
//         .and_then(|n| n.to_str())
//         .unwrap_or("unknown")
//         .to_string();
//
//     stats.repos.insert(repo_name, repo_stats);
//     Ok(stats)
// }

fn process_repo(
    path: &Path,
    username: &str,
    range: &TimeRange,
) -> Result<Option<GitStats>> {
    let repo = Repository::open(path)?;
    let mut stats = GitStats::default();
    let mut repo_stats = RepoStats::default();


    // 1. 获取默认分支
    let default_branch = match get_default_branch(&repo) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("跳过 {}: {}", path.display(), e);
            return Ok(None);
        }
    };
    let branch_ref = format!("refs/heads/{}", default_branch);

    // 方法1：尝试 HEAD，如果失败则检查是否有任何提交
    let mut revwalk = repo.revwalk()?;

    // match repo.head() {
    //     Ok(head) => {
    //         // 正常情况：HEAD指向有效引用
    //         revwalk.push(head.target().unwrap())?;
    //     },
    //     Err(e) if e.code() == git2::ErrorCode::UnbornBranch => {
    //         // 特殊情况：新仓库（无任何提交）
    //         println!("提示: 仓库 {} 尚未创建任何分支", path.display());
    //         return Ok(stats); // 返回空统计
    //     },
    //     Err(e) => return Err(e.into()), // 其他错误
    // }

    if let Ok(reference) = repo.find_reference(&branch_ref) {
        revwalk.push(reference.target().unwrap())?;
    } else {
        eprintln!("警告: 仓库 {} 没有默认分支 {}", path.display(), default_branch);
        return Ok(Some(stats));
    }

    // 处理提交历史
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let signature = commit.author();

        if signature.name().map(|n| n == username).unwrap_or(false) {
            process_commit(&repo, &commit, &mut stats, &mut repo_stats , range)?;
        }
    }

    // 记录仓库信息
    let repo_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // let repo_stats = RepoStats {
    //     commits: stats.total_commits,
    //     additions: stats.additions,
    //     deletions: stats.deletions,
    //     files_changed: stats.files_changed,
    // };

    stats.repos.insert(repo_name, repo_stats);
    Ok(Some(stats))
}


// fn process_branch_commits(
//     repo: &Repository,
//     commit: &Commit,
//     username: &str,
//     range: &TimeRange,
//     stats: &mut GitStats,
// ) -> Result<()> {
//     let mut revwalk = repo.revwalk()?;
//     let mut stats = GitStats::default();
//     let mut repo_stats = RepoStats::default();
//     revwalk.push(commit.id())?;
//
//     for oid in revwalk {
//         let oid = oid?;
//         let commit = repo.find_commit(oid)?;
//         let signature = commit.author();
//
//         if signature.name().map(|n| n == username).unwrap_or(false) {
//             process_commit(&repo, &commit, &mut stats, &mut repo_stats , range)?;
//         }
//     }
//
//     Ok(())
// }

fn process_commit(
    repo: &Repository,
    commit: &Commit,
    stats: &mut GitStats,
    repo_stats: &mut RepoStats,
    range: &TimeRange,
) -> Result<()> {
    let time = DateTime::from_timestamp(commit.time().seconds(), 0)
        .context("Invalid commit time")?
        .with_timezone(&Local);  // 转换为本地时间
    if !is_in_range(&time, range) {
        return Ok(());
    }

    stats.total_commits += 1;

    // 记录最晚提交时间
    if time.hour() >= 22 || time.hour() <= 4 {
        stats.late_night_commits += 1;
    }

    if stats.latest_commit_time.map_or(true, |t| time > t) {
        stats.latest_commit_time = Some(time);
    }

    // 按小时统计
    stats.commits_by_hour[time.hour() as usize] += 1;

    // 按星期统计
    let weekday = time.weekday().num_days_from_monday() as usize;
    stats.commits_by_weekday[weekday] += 1;

    // 统计变更
    if let Ok(tree) = commit.tree() {
        if let Some(parent) = commit.parents().next() {
            if let Ok(parent_tree) = parent.tree() {
                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;

                repo_stats.additions += diff.stats()?.insertions();
                repo_stats.deletions += diff.stats()?.deletions();
                repo_stats.files_changed += diff.stats()?.files_changed();

                // 语言统计
                for delta in diff.deltas() {
                    if let Some(file) = delta.new_file().path() {
                        if let Some(ext) = file.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                *stats.languages.entry(ext_str.to_string()).or_default() += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_default_branch(repo: &Repository) -> Result<String> {
    // 1. 首先尝试获取本地HEAD指向的分支
    if let Ok(head) = repo.head() {
        if let Some(name) = head.shorthand() {
            return Ok(name.to_string());
        }
    }

    // 2. 获取远程默认分支（正确处理Result）
    if let Ok(remote) = repo.find_remote("origin") {
        match remote.default_branch() {
            Ok(refname) => {
                // 最新 git2 (0.14+) 版本的正确处理方式
                let branch_name = String::from_utf8_lossy(&refname).replace("refs/heads/", "");
                return Ok(branch_name);
            },
            Err(e) => {
                eprintln!("提示: 无法获取origin远程的默认分支 - {}", e);
            }
        }
    }

    // 3. 回退方案：检查本地存在的分支
    let mut branches = repo.branches(None)?;
    if let Some(Ok((branch, _))) = branches.next() {
        if let Ok(name) = branch.name() {
            if let Some(name) = name {
                return Ok(name.to_string());
            }
        }
    }

    Err(anyhow!("无法确定默认分支"))
}


fn is_in_range(time: &DateTime<Local>, range: &TimeRange) -> bool {
    let now = Local::now();
    match range {
        TimeRange::Day => time.date_naive() == now.date_naive(),
        TimeRange::Week => {
            let week_start = now - Duration::days(now.weekday().num_days_from_monday() as i64);
            time >= &week_start
        }
        TimeRange::Month => time.month() == now.month() && time.year() == now.year(),
        TimeRange::Year => time.year() == now.year(),
    }
}

fn print_table(stats: &GitStats) {
    println!("\nGit Activity Report\n");

    // 基本统计
    println!("Total commits: {}", stats.total_commits);
    println!("Late night commits (10pm-4am): {}", stats.late_night_commits);
    if let Some(time) = stats.latest_commit_time {
        println!("Latest commit time: {}", time.format("%Y-%m-%d %H:%M:%S"));
    }

    // 按仓库统计
    let repo_rows: Vec<RepoTableRow> = stats.repos.iter()
        .map(|(name, repo_stats)| RepoTableRow {
            name: name.clone(),
            commits: repo_stats.commits,
            additions: repo_stats.additions,
            deletions: repo_stats.deletions,
            files_changed: repo_stats.files_changed,
        })
        .collect();

    println!("\nBy Repository:");
    println!("{}", Table::new(repo_rows));

    // 按语言统计
    let mut lang_vec: Vec<_> = stats.languages.iter().collect();
    lang_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nBy Language:");
    for (i, (lang, count)) in lang_vec.iter().take(5).enumerate() {
        println!("{}. {}: {}", i + 1, lang, count);
    }

    // 按小时统计
    println!("\nCommits by Hour:");
    for (hour, &count) in stats.commits_by_hour.iter().enumerate() {
        println!("{:02}:00 - {:02}:59: {}", hour, hour, count);
    }

    // 按星期统计
    println!("\nCommits by Weekday:");
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    for (i, &count) in stats.commits_by_weekday.iter().enumerate() {
        println!("{}: {}", weekdays[i], count);
    }
}

fn print_json(stats: &GitStats) -> Result<()> {
    let json = serde_json::to_string_pretty(stats)?;
    println!("{}", json);
    Ok(())
}