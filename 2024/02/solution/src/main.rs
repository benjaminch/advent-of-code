use std::error::Error as StdError;
use std::io::{self, Read, Write};

use aoc_2024_solution_02::{
    count_total_safe_reports, count_total_safe_reports_with_max_one_skipped_level,
    get_reports_from_str,
};

fn main() -> Result<(), Box<dyn StdError>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let reports = get_reports_from_str(&input)?;
    writeln!(
        io::stdout(),
        "Part 1 / Total safe reports: {}",
        count_total_safe_reports(reports.clone())?
    )?;
    writeln!(
        io::stdout(),
        "Part 2 / Total safe reports (having max 1 error): {}",
        count_total_safe_reports_with_max_one_skipped_level(reports)?
    )?;
    Ok(())
}
