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

    /// Start date (YYYY-MM-DD), defaults to first day of current year, supported formats are YYYY-MM-DD, YYYY/MM/DD, YYYY.MM.DD
    #[arg(short = 's', long = "start", value_parser = parse_naive_date)]
    pub start_date: Option<NaiveDate>,

    /// End date (YYYY-MM-DD), defaults to last day of current year
    #[arg(short = 'e', long = "end", value_parser = parse_naive_date)]
    pub end_date: Option<NaiveDate>,
}


fn parse_naive_date(s: &str) -> Result<NaiveDate, String> {
    let formats = [
        "%Y-%m-%d",    // 2024-01-01
        "%Y/%m/%d",    // 2024/01/01
        "%Y.%m.%d",    // 2024.01.01
    ];

    for fmt in formats.iter() {
        if let Ok(date) = NaiveDate::parse_from_str(s, fmt) {
            return Ok(date);
        }
    }

    Err(format!("无法解析日期: '{}'，支持的格式: YYYY-MM-DD, YYYY/MM/DD, YYYY.MM.DD", s))
}

impl CliArgs {
    pub fn get_date_range(&self) -> Result<(NaiveDate, NaiveDate)>  {
        let current_year = Local::now().year();

        let since = self.start_date.unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap()
        });

        let until = self.end_date.unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap()
        });

        if since > until {
            return Err(anyhow!("结束日期不能早于开始日期"));
        }

        Ok((since, until))
    }
}