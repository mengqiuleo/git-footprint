use crate::analyzer::AnalysisResult;

pub fn print_report(r: &AnalysisResult) {
    println!("\n📊 Git 活跃度统计报告 📊\n");
    println!("总提交次数: {}", r.total_commits);
    println!("\n各仓库提交量:");
    for (repo, count) in &r.per_repo {
        println!("- {}: {} 次", repo, count);
    }

    println!("\n按小时分布的提交数量:");
    for (hour, count) in r.commits_per_hour.iter().enumerate() {
        let bar = "█".repeat(*count / 2);
        println!("{:02}:00 - {:>3} {}", hour, count, bar);
    }
}