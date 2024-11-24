use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
pub type URI = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct DateTime(String);

impl From<&chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(value: &chrono::DateTime<chrono::Utc>) -> Self {
        DateTime(value.to_rfc3339())
    }
}

impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
    type Error = chrono::ParseError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        chrono::DateTime::parse_from_rfc3339(value.0.as_str()).map(Into::into)
    }
}

impl DateTime {
    pub fn diff_seconds(&self, other: &DateTime) -> i64 {
        let target = self.0.parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let compare = other.0.parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let duration = target - compare;

        duration.num_seconds()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn get_dummy_date_time() -> DateTime {
        DateTime("2024-11-22T12:34:56Z".to_string())
    }
}
