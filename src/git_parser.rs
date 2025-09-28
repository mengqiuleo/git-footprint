use git2::{Repository, BranchType, Sort, Commit};
use chrono::{NaiveDate, DateTime, Utc, TimeZone, Local};
use anyhow::Result;
use std::collections::HashSet;
use std::path::{PathBuf};

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub time: DateTime<Local>,
    pub message: String,
}

fn get_commit_local_time(commit: &Commit) -> DateTime<Local>  {
    let git_commit_time = Utc.timestamp_opt(commit.time().seconds(), 0).single().unwrap();

    git_commit_time.with_timezone(&Local)
}

pub fn parse_git_logs(path: &PathBuf, email: &str, since: NaiveDate, until: NaiveDate) -> Result<Vec<CommitInfo>> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;


    for branch in repo.branches(Some(BranchType::Local))? {
        let (branch, _) = branch?;
        if let Some(oid) = branch.get().target() {
            revwalk.push(oid)?;
        }
    }

    revwalk.set_sorting(Sort::TIME)?;

    let mut seen = HashSet::new();
    let mut commits = Vec::new();

    for oid in revwalk.flatten() {
        if seen.contains(&oid) {
            continue;
        }
        seen.insert(oid);

        if let Ok(commit) = repo.find_commit(oid) {
            if !commit.author().email().map_or(false, |author_email| author_email == email) {
                continue;
            }

            let commit_time = get_commit_local_time(&commit);
            let commit_date = commit_time.date_naive();
            if commit_date < since || commit_date > until {
                continue;
            }

            commits.push(CommitInfo {
                time: commit_time,
                message: commit.message().unwrap_or_default().to_string(),
            });
        }
    }

    Ok(commits)
}
