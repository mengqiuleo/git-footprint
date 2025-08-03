mod cli;
mod scanner;
mod git_parser;
mod analyzer;
mod formatter;

use anyhow::Result;
use cli::CliArgs;
use clap::Parser;
use scanner::scan_git_repos;
use git_parser::parse_git_logs;
use analyzer::analyze;
use formatter::print_report;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let repos = scan_git_repos(&args.path)?;
    let mut all_commits = Vec::new();

    for repo in &repos {
        let commits = parse_git_logs(repo, &args.email, args.since, args.until)?;
        all_commits.push((repo.clone(), commits));
    }

    let analysis = analyze(&all_commits);
    print_report(&analysis);

    Ok(())
}