use chrono::{Datelike, Duration, Months, NaiveDate};

use anyhow::Result;

pub fn get_monthly_date_ranges(
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<(NaiveDate, NaiveDate)>, anyhow::Error> {
    if from.day() != 1 {
        return Err(anyhow::anyhow!("from must be 1st day of month"));
    }
    if to.checked_add_signed(Duration::days(1)).unwrap().day() != 1 {
        return Err(anyhow::anyhow!("to must be last day of month"));
    }

    let mut result = vec![];

    let mut start = from;
    loop {
        let end = start
            .checked_add_months(Months::new(1))
            .unwrap()
            .pred_opt()
            .unwrap();
        result.push((start, end));
        if to == end {
            break;
        }
        start = start.checked_add_months(Months::new(1)).unwrap()
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_monthly_date_ranges_ok() {
        struct Case<'a> {
            name: &'a str,
            from: NaiveDate,
            to: NaiveDate,
            want: Vec<(NaiveDate, NaiveDate)>,
        }
        let cases = &[
            Case {
                name: "a month",
                from: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                to: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
                want: vec![(
                    NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
                )],
            },
            Case {
                name: "a few month",
                from: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                to: NaiveDate::from_ymd_opt(2024, 11, 30).unwrap(),
                want: vec![
                    (
                        NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                        NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
                    ),
                    (
                        NaiveDate::from_ymd_opt(2024, 11, 1).unwrap(),
                        NaiveDate::from_ymd_opt(2024, 11, 30).unwrap(),
                    ),
                ],
            },
            Case {
                name: "a few months across the year",
                from: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                to: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                want: vec![
                    (
                        NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                        NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
                    ),
                    (
                        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                        NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    ),
                ],
            },
        ];
        for case in cases {
            let got = get_monthly_date_ranges(case.from, case.to).unwrap();
            assert_eq!(case.want, got, "{}", case.name);
        }
    }

    #[test]
    fn test_get_monthly_date_ranges_err() {
        struct Case<'a> {
            name: &'a str,
            from: NaiveDate,
            to: NaiveDate,
        }
        let cases = &[
            Case {
                name: "'from' must be 1st day of month",
                from: NaiveDate::from_ymd_opt(2024, 10, 2).unwrap(),
                to: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
            },
            Case {
                name: "'to' must be last day of month",
                from: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                to: NaiveDate::from_ymd_opt(2024, 10, 30).unwrap(),
            },
        ];
        for case in cases {
            let got = get_monthly_date_ranges(case.from, case.to);
            assert!(got.is_err(), "{}", case.name);
        }
    }
}
