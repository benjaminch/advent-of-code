use thiserror::Error;

#[derive(Error, Debug)]
pub enum Aoc2024Day2Error {
    #[error("cannot read from stdin")]
    CannotReadFromStdIn,
    #[error("cannot read from stdout")]
    CannotWriteToStdOut,
    #[error("wrong input: {message}")]
    WrongInput { message: String },
}

pub enum Report<'a> {
    Safe { report: &'a [i32] },
    Unsafe { report: &'a [i32] },
}

impl<'a> Report<'a> {
    pub fn new(report: &'a [i32]) -> Self {
        // if report.len() < 2 {
        //     return Report::Unsafe { report };
        // }

        let mut last_was_increasing = false;
        let mut current = 1;
        let mut previous = 0;
        while current < report.len() {
            let diff = report[current].abs_diff(report[previous]);
            let is_increasing = report[previous] < report[current];
            if current == 1 {
                last_was_increasing = is_increasing;
            }
            if diff > 3 || diff == 0 || is_increasing != last_was_increasing {
                return Report::Unsafe { report };
            }
            previous = current;
            current += 1;
        }

        Report::Safe { report }
    }
}

pub fn get_reports_from_str(input: &str) -> Result<Vec<Vec<i32>>, Aoc2024Day2Error> {
    let mut reports: Vec<Vec<i32>> = Vec::new(); // TODO: reports can be stored in unique Vec but
                                                 // needs to have special sauce to handle the variable report length
    for line in input.lines() {
        let parts = line.trim().split_ascii_whitespace();

        let mut report = Vec::new(); // TODO: pre-allocate capacity
        for part in parts {
            report.push(part.parse().map_err(|_| Aoc2024Day2Error::WrongInput {
                message: format!("Could not parse number from `{}`", part),
            })?);
        }

        reports.push(report);
    }

    Ok(reports)
}

pub fn count_total_safe_reports(reports: Vec<Vec<i32>>) -> Result<usize, Aoc2024Day2Error> {
    Ok(reports
        .iter()
        .map(|report| Report::new(report))
        .filter(|r| match r {
            Report::Safe { .. } => true,
            Report::Unsafe { .. } => false,
        })
        .count())
}

fn create_sub_reports_with_max_one_skipped_level(report: &[i32]) -> Vec<Vec<i32>> {
    (0..report.len())
        .map(|skipped| create_sub_report_with_one_skipped_level(skipped, report))
        .collect()
}

fn create_sub_report_with_one_skipped_level(skipped: usize, report: &[i32]) -> Vec<i32> {
    let mut sub_report = Vec::with_capacity(report.len() - 1);
    report.iter().enumerate().for_each(|(index, &value)| {
        if index != skipped {
            sub_report.push(value);
        }
    });
    sub_report
}

pub fn count_total_safe_reports_with_max_one_skipped_level(
    reports: Vec<Vec<i32>>,
) -> Result<isize, Aoc2024Day2Error> {
    Ok(reports
        .iter()
        .map(|report| {
            let sub_reports = create_sub_reports_with_max_one_skipped_level(report);
            sub_reports.iter().any(|sub_report| {
                let report = Report::new(sub_report);
                match report {
                    Report::Safe { .. } => true,
                    Report::Unsafe { .. } => false,
                }
            })
        })
        .map(|x| x as isize)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lists_from_str() {
        assert_eq!(
            get_reports_from_str(
                "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n"
            )
            .expect("should not fail"),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ],
        );
    }

    #[test]
    fn test_get_total_safe_reports() {
        assert_eq!(
            count_total_safe_reports(vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ],)
            .expect("should not fail"),
            2,
        );
    }

    #[test]
    fn test_total_safe_reports_with_max_error() {
        assert_eq!(
            count_total_safe_reports_with_max_one_skipped_level(vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ],)
            .expect("should not fail"),
            4,
        );
    }
}
