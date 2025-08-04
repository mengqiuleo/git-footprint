use anyhow::anyhow;
use clap::Parser;
use chrono::{Local, Datelike, NaiveDate};
use anyhow::Result;

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

impl CliArgs {
    pub fn get_date_range(&self) -> Result<(NaiveDate, NaiveDate)>  {
        let current_year = Local::now().year();

        let since = self.since.unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap()
        });

        let until = self.until.unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap()
        });

        if since > until {
            return Err(anyhow!("结束日期不能早于开始日期"));
        }

        Ok((since, until))
    }
}