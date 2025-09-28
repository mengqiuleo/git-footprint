use std::collections::HashMap;
use chrono::{Datelike, NaiveDate};
use colored::*;

#[derive(Debug)]
pub struct ContributionCalendar {
    pub weeks: Vec<Week>,
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

pub fn create_contribution_calendar(
    commits_per_day: &HashMap<NaiveDate, usize>,
    start_date: NaiveDate,
    end_date: NaiveDate
) -> ContributionCalendar {
    let total_weeks = 53;

    let mut weeks = Vec::new();

    let mut current_date = start_date;

    for _ in 0..total_weeks {
        let mut week = Week { contribution_days: Vec::new() };

        for _ in 0..7 {
            if current_date <= end_date {
                let count = commits_per_day.get(&current_date).copied().unwrap_or(0);
                week.contribution_days.push(ContributionDay {
                    date: current_date,
                    contribution_count: count,
                });
            } else {
                week.contribution_days.push(ContributionDay {
                    date: current_date,
                    contribution_count: 0,
                });

            }

            current_date += chrono::Duration::days(1);
        }
        weeks.push(week);
    }

    ContributionCalendar {
        weeks
    }
}


pub fn draw_contribution_calendar(calendar: &ContributionCalendar) {
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    print!("    ");
    let mut current_month = None;
    for week_idx in 0..53 {
        if let Some(day) = calendar.weeks[week_idx].contribution_days.first() {
            let month = day.date.month();
            if current_month != Some(month) {
                current_month = Some(month);
                if month <= 12 {
                    print!("{}      ", months[month as usize - 1]);
                    continue;
                }
            }
        }
    }
    println!();


    let weekdays = ["Mon", "Wed", "Fri"];

    for row in 0..7 {
        if row % 2 == 1 && row / 2 < weekdays.len() {
            print!("{:>3} ", weekdays[row / 2]);
        } else {
            print!("    ");
        }

        for week_idx in 0..53 {
            if week_idx < calendar.weeks.len() {
                let week = &calendar.weeks[week_idx];
                if let Some(day) = week.contribution_days.get(row) {
                    let symbol = match day.contribution_count {
                        0 => "■".normal(),
                        1..=10 => "■".green().dimmed(),
                        11..=30 => "■".green(), 
                        31..=50 => "■".bright_green(), 
                        _ => "■".bright_green().bold(),
                    };
                    print!("{} ", symbol);
                }
            }
        }
        println!();
    }
}