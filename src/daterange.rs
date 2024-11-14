use chrono::{Datelike, Duration, Months, NaiveDate};

use anyhow::Result;

pub fn get_monthly_date_ranges(
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<(NaiveDate, NaiveDate)>, anyhow::Error> {
    if from.day() != 1 {
        return Err(anyhow::anyhow!("start_date must be 1st day"));
    }
    if to.checked_add_signed(Duration::days(1)).unwrap().day() != 1 {
        return Err(anyhow::anyhow!("start_date must be 1st day"));
    }

    let mut result = vec![];

    let mut start = from.clone();
    loop {
        let end = start
            .pred_opt()
            .unwrap()
            .checked_add_months(Months::new(1))
            .unwrap();
        result.push((start, end));
        if to == end {
            break;
        }
        start = start.checked_add_months(Months::new(1)).unwrap()
    }

    Ok(result)
}
