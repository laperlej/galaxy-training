use std::str::FromStr;
use chrono::{NaiveDate, Utc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub NaiveDate);

impl Date {
    pub fn now() -> Date {
        Date(Utc::now().naive_local().into())
    }
}

impl FromStr for Date {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
        Ok(Date(date))
    }
}
