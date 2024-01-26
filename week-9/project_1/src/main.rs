use std::fs::File;
use std::io::prelude::*;

struct Company {
    username: String,
    password: String,
    name: String,
    year: i32,
    shares: i32,
    liabilities: i32,
}

impl Company {
    fn calculate_metrics(&self) -> (f64, f64, f64, f64) {
        let total_assets = self.shares as f64;
        let total_liabilities = self.liabilities as f64;
        let leverage = total_assets / total_liabilities;
        let percentage_leverage = leverage / 100.0;
        let leverage_multiplier = percentage_leverage / (1.0 - percentage_leverage);
        let bonus = (1.0 - percentage_leverage) * 100.0;
        (leverage, percentage_leverage, leverage_multiplier, bonus)
    }
}

fn main() {
    let companies = [
        Company {
            username: String::from("cadbury"),
            password: String::from("pass123"),
            name: String::from("Cadbury"),
            year: 2000,
            shares: 2_000_000,
            liabilities: 1_000_000,
        },
        // ... (the rest of the companies)
    ];

    let mut data_file = File::create("data.txt").expect("Unable to create file");

    for company in companies.iter() {
        let (leverage, percentage_leverage, leverage_multiplier, bonus) = company.calculate_metrics();

        writeln!(
            data_file,
            "Company: {}\nYear Founded: {}\nShares: {}\nLiabilities: {}\nLeverage: {:.2}\nPercentage Leverage: {:.2}\nPercentage Leverage Multiplier: {:.2}\nBonus: {:.2}\n",
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
}