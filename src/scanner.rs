use std::path::{PathBuf};
use std::time::Duration;
use walkdir::WalkDir;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub fn scan_git_repos(root: &str) -> Result<Vec<PathBuf>> {
    let mut repos = Vec::new();
    println!("☕ 正在扫描目录: {root}");

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("正在努力寻找 Git 仓库...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")?
            .tick_chars("🌑🌒🌓🌔🌕🌖🌗🌘"),
    );

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() && entry.file_name() == ".git" {
            if let Some(repo_path) = entry.path().parent() {
                repos.push(repo_path.to_path_buf());
            }
        }
    }
    spinner.finish_with_message(format!("✅ 扫描完成，共发现 {} 个 Git 仓库", repos.len()));
    Ok(repos)
}
