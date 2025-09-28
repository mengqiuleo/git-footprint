use std::collections::HashMap;
use std::path::{PathBuf};
use chrono::{NaiveDate, Timelike};
use std::path::Path;
use tokei::{Config, Languages};
use crate::git_parser::CommitInfo;

#[derive(Debug)]
pub struct RepoStat {
    pub name: String,
    pub commit_count: usize,
}

#[derive(Debug)]
pub struct LanguageStat {
    pub name: String,
    pub lines: usize,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub total_commits: usize,
    pub per_repo: Vec<RepoStat>,
    pub commits_per_hour: [usize; 24],
    pub commits_per_day: HashMap<NaiveDate, usize>,
    pub languages: Vec<LanguageStat>,
}

pub fn analyze(data: &Vec<(PathBuf, Vec<CommitInfo>)>) -> AnalysisResult {
    let mut total = 0;
    let mut per_repo = Vec::new();
    let mut per_hour = [0usize; 24];
    let mut per_day: HashMap<NaiveDate, usize> = HashMap::new();
    let mut languages: Vec<LanguageStat> = Vec::new();

    for (repo, commits) in data {
        total += commits.len();
        per_repo.push(RepoStat {
            name: repo.to_string_lossy().to_string(),
            commit_count: commits.len(),
        });
        for commit in commits {
            let hour = commit.time.hour() as usize;
            per_hour[hour] += 1;

            let date = commit.time.date_naive();
            per_day.entry(date)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for lang_stat in analyze_languages(repo) {
            if let Some(existing) = languages.iter_mut().find(|l| l.name == lang_stat.name) {
                existing.lines += lang_stat.lines;
            } else {
                languages.push(lang_stat);
            }
        }
    }

    per_repo.sort_by(|a, b| b.commit_count.cmp(&a.commit_count));
    languages.sort_by(|a, b| b.lines.cmp(&a.lines));

    AnalysisResult {
        total_commits: total,
        per_repo,
        commits_per_hour: per_hour,
        commits_per_day: per_day,
        languages
    }
}


fn analyze_languages(repo_path: &Path) -> Vec<LanguageStat> {
    let mut languages = Languages::new();
    let config = Config::default();

    languages.get_statistics(&[repo_path], &[], &config);

    languages
        .into_iter()
        .filter(|(_lang, stats)| stats.lines() > 0)
        .map(|(lang, stats)| LanguageStat {
            name: lang.name().to_string(),
            lines: stats.lines(),
        })
        .collect()
}