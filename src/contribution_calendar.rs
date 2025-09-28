use std::collections::HashMap;
use chrono::{Datelike, NaiveDate};
use colored::*;

#[derive(Debug)]
pub struct ContributionCalendar {
    pub weeks: Vec<Week>,
    pub total_contributions: usize,
}

#[derive(Debug)]
pub struct Week {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug)]
pub struct ContributionDay {
    pub date: NaiveDate,
    pub contribution_count: usize,
}

pub fn create_contribution_calendar(commits_per_day: &HashMap<NaiveDate, usize>) -> ContributionCalendar {
    let today = chrono::Local::now().date_naive();
    let one_year_ago = today - chrono::Duration::days(364); // 确保是53周

    let mut current_date = one_year_ago;
    let mut weeks = Vec::new();
    let mut total_contributions = 0;

    // 生成53周的数据
    for _ in 0..53 {
        let mut week = Week { contribution_days: Vec::new() };

        // 每周7天
        for _ in 0..7 {
            if current_date <= today {
                let count = commits_per_day.get(&current_date).copied().unwrap_or(0);
                week.contribution_days.push(ContributionDay {
                    date: current_date,
                    contribution_count: count,
                });
                total_contributions += count;
                current_date += chrono::Duration::days(1);
            } else {
                // 填充空白天
                week.contribution_days.push(ContributionDay {
                    date: current_date,
                    contribution_count: 0,
                });
            }
        }
        weeks.push(week);
    }

    ContributionCalendar {
        weeks,
        total_contributions,
    }
}


pub fn draw_contribution_calendar(calendar: &ContributionCalendar, term_width: usize) {
    let title = " Git Contribution Calendar (Last Year) ";
    let title_padding = (term_width.saturating_sub(title.len())) / 2;

    println!("{}{}", " ".repeat(title_padding), title);

    let total_text = format!("Total Commits: {}", calendar.total_contributions);
    let total_padding = (term_width.saturating_sub(total_text.len())) / 2;
    println!("{}{}\n", " ".repeat(total_padding), total_text);

    // Month headers - 调整月份显示逻辑
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    let calendar_width = 53; // GitHub风格是53周
    let cal_padding = (term_width.saturating_sub(calendar_width + 8)) / 2;

    // 打印月份行
    print!("{}", " ".repeat(cal_padding));
    print!("        ");

    // let mut current_month = None;
    // for week_idx in 0..calendar_width {
    //     if let Some(day) = calendar.weeks[week_idx].contribution_days.first() {
    //         let month = day.date.month();
    //         if current_month != Some(month) {
    //             current_month = Some(month);
    //             if month <= 12 {
    //                 print!("{}", months[month as usize - 1]);
    //                 continue;
    //             }
    //         }
    //     }
    //     print!(" ");
    // }
    // println!();

    let mut months_displayed = Vec::new();
    for week_idx in 0..calendar_width {
        if week_idx < calendar.weeks.len() {
            let week = &calendar.weeks[week_idx];

            // 检查这周是否包含月份的第一天
            let has_month_start = week.contribution_days.iter()
                .any(|day| day.date.day() == 1);

            if has_month_start {
                if let Some(day) = week.contribution_days.first() {
                    let month = day.date.month();
                    if month >= 1 && month <= 12 && !months_displayed.contains(&month) {
                        // 确保有空间显示（至少3个字符位置）
                        if week_idx <= calendar_width - 3 {
                            print!("{}", months[month as usize - 1]);
                            months_displayed.push(month);
                            continue;
                        }
                    }
                }
            }
        }
        print!(" ");
    }
    println!();

    // Days of week labels
    let weekdays = ["Mon", "Wed", "Fri"];

    // Draw the calendar grid
    for row in 0..7 {
        print!("{}", " ".repeat(cal_padding));

        // 星期标签
        if row % 2 == 1 && row / 2 < weekdays.len() {
            print!("{:>3} ", weekdays[row / 2]);
        } else {
            print!("    ");
        }

        // 贡献度方块
        for week_idx in 0..calendar_width {
            if week_idx < calendar.weeks.len() {
                let week = &calendar.weeks[week_idx];
                if let Some(day) = week.contribution_days.get(row) {
                    let symbol = match day.contribution_count {
                        0 => "■".truecolor(45, 51, 59),        // 无贡献
                        1..=2 => "■".truecolor(14, 68, 121),   // 少量
                        3..=5 => "■".truecolor(33, 110, 177),  // 中等
                        6..=10 => "■".truecolor(52, 152, 219), // 较多
                        _ => "■".truecolor(116, 185, 255),     // 很多
                    };
                    print!("{}", symbol);
                } else {
                    print!(" ");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Legend with actual colors
    let legend_padding = (term_width.saturating_sub(35)) / 2;
    print!("\n{}   Less  ", " ".repeat(legend_padding));
    print!("{}", "■".truecolor(45, 51, 59));
    print!("{}", "■".truecolor(14, 68, 121));
    print!("{}", "■".truecolor(33, 110, 177));
    print!("{}", "■".truecolor(52, 152, 219));
    print!("{}", "■".truecolor(116, 185, 255));
    println!("  More");
}