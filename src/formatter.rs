use chrono::NaiveDate;
use crate::analyzer::AnalysisResult;
use colored::*;
use crate::contribution_calendar::{create_contribution_calendar, draw_contribution_calendar};

pub fn print_report(r: &AnalysisResult, since: NaiveDate, until: NaiveDate) {
    println!("\n{}\n", "Git 活跃度统计报告".bold().cyan());

    println!("总提交次数: {}", r.total_commits);
    println!("\n各仓库提交量:");
    for per in &r.per_repo {
        println!("- {}: {} 次", per.name, per.commit_count);
    }

    println!("\n按小时分布的提交数量:");

    let max_count = r.commits_per_hour.iter().max().copied().unwrap_or(1);
    let max_bar_width = 50;

    for (hour, count) in r.commits_per_hour.iter().enumerate() {
        let width = if *count > 0 {
            (count * max_bar_width) / max_count
        } else {
            0
        };

        let bar = "◼".repeat(width).green();
        println!(
            "{:02}:00 - {:>4} {}",
            hour,
            count.to_string().yellow(),
            bar
        );
    }


    println!("\n代码语言分布:");

    let max_count = r.languages.iter().map(|l| l.lines).max().unwrap_or(1);
    let max_bar_width = 50;

    for lang_stat in r.languages.iter().take(10) {
        let width = if lang_stat.lines > 0 {
            (lang_stat.lines * max_bar_width) / max_count
        } else {
            0
        };
        let bar = "◼".repeat(width).green();

        println!(
            "{:>15} {}",
            &lang_stat.name.cyan(),
            bar
        );
    }



    println!("\n每日提交热力图:");
    let contribution_calendar = create_contribution_calendar(&r.commits_per_day, since, until);

    draw_contribution_calendar(&contribution_calendar);
    println!();
}