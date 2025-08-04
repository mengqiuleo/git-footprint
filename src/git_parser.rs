use git2::{Repository, BranchType, Sort};
use chrono::{NaiveDate, DateTime, Utc, TimeZone};
use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub time: DateTime<Utc>,
    pub message: String,
}

pub fn parse_git_logs(path: &std::path::Path, email: &str, since: Option<NaiveDate>, until: Option<NaiveDate>) -> Result<Vec<CommitInfo>> {
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
            if let Some(author_email) = commit.author().email() {
                if author_email != email {
                    continue;
                }
            } else {
                continue;
            }

            let commit_time = Utc.timestamp_opt(commit.time().seconds(), 0).single().unwrap();
            if let Some(since) = since {
                if commit_time.date_naive() < since {
                    continue;
                }
            }
            if let Some(until) = until {
                if commit_time.date_naive() > until {
                    continue;
                }
            }

            commits.push(CommitInfo {
                time: commit_time,
                message: commit.message().unwrap_or_default().to_string(),
            });
        }
    }

    Ok(commits)
}
