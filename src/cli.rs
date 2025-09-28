use anyhow::anyhow;
use clap::Parser;
use chrono::{Local, Datelike, NaiveDate};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(name = "git-stats")]
#[command(about, version)]
pub struct CliArgs {
    /// Git user email to filter commits
    #[arg(short, long)]
    pub email: String,

    /// Directory to scan for Git repositories
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Year to analyze (e.g., 2024), defaults to current year
    #[arg(short, long)]
    pub year: Option<i32>,
}

impl CliArgs {
    pub fn get_date_range(&self) -> Result<(NaiveDate, NaiveDate)> {
        let current_year = Local::now().year();
        let year = self.year.unwrap_or(current_year);

        let since = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| anyhow!("无效的年份: {}", year))?;

        let until = NaiveDate::from_ymd_opt(year, 12, 31)
            .ok_or_else(|| anyhow!("无效的年份: {}", year))?;

        Ok((since, until))
    }
}