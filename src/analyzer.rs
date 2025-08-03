use crate::git_parser::CommitInfo;
use chrono::Timelike;

pub struct AnalysisResult {
    pub total_commits: usize,
    pub per_repo: Vec<(String, usize)>,
    pub commits_per_hour: [usize; 24],
}

pub fn analyze(data: &Vec<(std::path::PathBuf, Vec<CommitInfo>)>) -> AnalysisResult {
    let mut total = 0;
    let mut per_repo = Vec::new();
    let mut per_hour = [0usize; 24];

    for (repo, commits) in data {
        total += commits.len();
        per_repo.push((repo.to_string_lossy().to_string(), commits.len()));
        for commit in commits {
            per_hour[commit.time.hour() as usize] += 1;
        }
    }

    per_repo.sort_by(|a, b| b.1.cmp(&a.1));

    AnalysisResult {
        total_commits: total,
        per_repo,
        commits_per_hour: per_hour,
    }
}
