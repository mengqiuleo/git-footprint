use std::collections::HashMap;
use crate::git_parser::CommitInfo;
use chrono::Timelike;
use chrono::{Local};
use std::path::Path;
use tokei::{Config, Language, Languages};

#[derive(Debug)]
pub struct AnalysisResult {
    pub total_commits: usize,
    pub per_repo: Vec<(String, usize)>,
    pub commits_per_hour: [usize; 24],
    pub languages: HashMap<String, usize>,
}

pub fn analyze(data: &Vec<(std::path::PathBuf, Vec<CommitInfo>)>) -> AnalysisResult {
    let mut total = 0;
    let mut per_repo = Vec::new();
    let mut per_hour = [0usize; 24];
    let mut languages = HashMap::new();

    for (repo, commits) in data {
        total += commits.len();
        per_repo.push((repo.to_string_lossy().to_string(), commits.len()));
        for commit in commits {
            let local_time = commit.time.with_timezone(&Local);
            per_hour[local_time.hour() as usize] += 1;
        }

        // 分析仓库语言
        let repo_langs = analyze_languages(&repo);
        for (lang, lines) in repo_langs {
            *languages.entry(lang).or_insert(0) += lines;
        }
    }

    per_repo.sort_by(|a, b| b.1.cmp(&a.1));

    AnalysisResult {
        total_commits: total,
        per_repo,
        commits_per_hour: per_hour,
        languages
    }
}


fn analyze_languages(repo_path: &Path) -> HashMap<String, usize> {
    let mut languages = Languages::new();
    let config = Config::default();

    languages.get_statistics(&[repo_path], &[], &config);

    languages
        .into_iter()
        .filter(|(lang, stats)| stats.lines() > 0)
        .map(|(lang, stats)| (lang.name().to_string(), stats.lines()))
        .collect()
}