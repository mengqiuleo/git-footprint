use crate::analyzer::AnalysisResult;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};

use std::io::{stdout};
// use crossterm::{terminal, ExecutableCommand};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
// pub fn print_report(r: &AnalysisResult) {
//     println!("打印结果");
//
//     let mut terminal = ratatui::init();
//
//     let total_commits = format!("总提交次数: {}", r.total_commits);
//
//     terminal.draw(|f| {
//         let chunks = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints([
//                 Constraint::Length(3),
//                 Constraint::Min(10),
//                 Constraint::Min(10),
//             ])
//             .split(f.area());
//
//         let header = Paragraph::new(total_commits.clone())
//             .style(Style::default().fg(Color::Cyan))
//             .block(Block::default().title("📊 Git 活跃度统计报告 📊").borders(Borders::ALL))
//             .wrap(Wrap { trim: true });
//         f.render_widget(header, chunks[0]);
//
//         let repo_rows: Vec<Row> = r.per_repo.iter().map(|(name, count)| {
//             Row::new(vec![name.clone(), count.to_string()])
//         }).collect();
//
//         let repo_table = Table::new(repo_rows, vec![Constraint::Percentage(70), Constraint::Percentage(30)])
//             .header(Row::new(vec!["仓库", "提交次数"]).style(Style::default().fg(Color::Yellow)))
//             .block(Block::default().title("各仓库提交量").borders(Borders::ALL));
//         f.render_widget(repo_table, chunks[1]);
//
//         let hour_rows: Vec<Row> = r.commits_per_hour.iter().enumerate().map(|(hour, count)| {
//             Row::new(vec![format!("{:02}:00", hour), count.to_string()])
//         }).collect();
//
//         let hour_table = Table::new(hour_rows, vec![Constraint::Percentage(50), Constraint::Percentage(50)])
//             .header(Row::new(vec!["时间", "提交次数"]).style(Style::default().fg(Color::Green)))
//             .block(Block::default().title("按小时分布的提交数量").borders(Borders::ALL));
//         f.render_widget(hour_table, chunks[2]);
//     }).unwrap();
//
//     std::thread::sleep(std::time::Duration::from_secs(10));
// }


pub fn print_report(r: &AnalysisResult) {
    println!("\n📊 Git 活跃度统计报告 📊\n");
    println!("总提交次数: {}", r.total_commits);
    println!("\n各仓库提交量:");
    for (repo, count) in &r.per_repo {
        println!("- {}: {} 次", repo, count);
    }

    println!("\n按小时分布的提交数量:");

    let max_count = *r.commits_per_hour.iter().max().unwrap_or(&1);
    let max_bar_width = 50;

    for (hour, count) in r.commits_per_hour.iter().enumerate() {
        let width = if count > &0 {
            (count * max_bar_width) / max_count
        } else {
            0
        };
        let bar = style("█".repeat(width)).green();
        println!(
            "{:02}:00 - {:>3} {}",
            style(hour).bold(),
            style(count).yellow(),
            bar
        );
    }


    // 语言统计
    println!("\n{}", style("代码语言分布:").bold());

    let total_lines: usize = r.languages.values().sum();
    let mut lang_vec: Vec<_> = r.languages.iter().collect();
    lang_vec.sort_by(|a, b| b.1.cmp(a.1)); // 按代码量排序

    for (lang, lines) in lang_vec.iter().take(10) {
        let percentage = (**lines as f64 / total_lines as f64) * 100.0;
        let bar_width = (percentage * 2.0) as usize;
        println!(
            "{:>6.1}% {:<15} {}",
            percentage,
            style(lang).cyan(),
            style("█".repeat(bar_width)).green()
        );
    }
}
