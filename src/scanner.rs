use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;

pub fn scan_git_repos(root: &str) -> Result<Vec<PathBuf>> {
    let mut repos = Vec::new();
    println!("正在扫描目录: {}", root);
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() && entry.file_name() == ".git" {
            if let Some(repo_path) = entry.path().parent() {
                repos.push(repo_path.to_path_buf());
            }
        }
    }
    println!("扫描目录结束：{}", repos.len());
    Ok(repos)
}
