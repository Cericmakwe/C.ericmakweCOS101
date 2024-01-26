use std::fs::File;
use std::io::{self, Write};

struct Company {
    name: String,
    year: i32,
    shares: i32,
    liabilities: i32,
}

impl Company {
    fn lev_percent(&self) -> (f64, f64, f64) {
        let leverage = (self.shares - self.liabilities) as f64 / self.shares as f64;
        let percentage_leverage = leverage * 100.0;
        let leverage_multiplier = percentage_leverage * 2.0;

        (leverage, percentage_leverage, leverage_multiplier)
    }
}

fn login() -> bool {
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter your password:");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let name = username.trim().len() >= 3 && username.trim().len() <= 8;
    let pass = password.trim().chars().all(|pw| pw.is_ascii_lowercase() || pw.is_numeric())
        && !password.contains(|pw: char| pw == '$' || pw == '#' || pw == '@');

    name && pass
}

fn calculate_bonus(percentage_leverage: f64, liabilities: i32) -> f64 {
    if liabilities < 10_000_000 {
        percentage_leverage * 0.05
    } else {
        0.0
    }
}

fn main() {
    if login() {
        let companies = vec![
            Company {
                name: String::from("Cadbury"),
                year: 1965,
                shares: 1_000_000,
                liabilities: 500_000,
            },
            // Add other companies here
        ];

        let mut data_file = File::create("data.txt").expect("Unable to create file");

        writeln!(
            data_file,
            "| {:<15} | {:<12} | {:<10} | {:<10} | {:<8} | {:<18} | {:<22} | {:<10} |",
            "Name",
            "Year Founded",
            "Shares",
            "Liabilities",
            "Leverage",
            "Percentage Leverage",
            "Percentage Leverage Multiplier",
            "Bonus"
        )
        .expect("Unable to write to file");

        for company in &companies {
            let (leverage, percentage_leverage, leverage_multiplier) = company.lev_percent();
            let bonus = calculate_bonus(percentage_leverage, company.liabilities);

            writeln!(
                data_file,
                "| {:<15} | {:<12} | {:<10} | {:<10} | {:<8.2} | {:<18.2} | {:<22.2} | {:<10.2} |",
                company.name,
                company.year,
                company.shares,
                company.liabilities,
                leverage,
                percentage_leverage,
                leverage_multiplier,
                bonus
            )
            .expect("Unable to write to file");
        }

        println!("Data saved successfully.");
    } else {
        println!("Invalid login.");
    }
}