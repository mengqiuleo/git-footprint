use clap::Parser;
use chrono::NaiveDate;

#[derive(Parser, Debug)]
#[command(name = "git-stats")]
#[command(about = "Generate Git contribution stats.", long_about = None)]
pub struct CliArgs {
    /// Git user email to filter commits
    #[arg(short, long)]
    pub email: String,

    /// Directory to scan for Git repositories
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Start date (YYYY-MM-DD)
    #[arg(long)]
    pub since: Option<NaiveDate>,

    /// End date (YYYY-MM-DD)
    #[arg(long)]
    pub until: Option<NaiveDate>,
}