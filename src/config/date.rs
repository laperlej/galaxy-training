use std::str::FromStr;
use chrono::{NaiveDate, Utc};

/// Represents a date using a `NaiveDate` from the `chrono` crate.
///
/// # Examples
///
/// ```
/// use your_crate_name::Date;
/// use std::str::FromStr;
///
/// let date = Date::from_str("2023-05-15").unwrap();
/// assert_eq!(date.0.to_string(), "2023-05-15");
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub NaiveDate);

impl Date {
    /// Returns a `Date` instance representing the current date.
    ///
    /// # Examples
    ///
    /// ```
    /// use config::Date;
    /// use chrono::Utc;
    ///
    /// let now = Date::now();
    /// assert_eq!(now.0.to_string(), Utc::now().date_naive().to_string());
    /// ```
    pub fn now() -> Date {
        Date(Utc::now().date_naive())
    }
    /// Returns a `Date` instance representing the given year, month, and day.
    ///
    /// # Examples
    ///
    /// ```
    /// use config::Date;
    ///
    /// let date = Date::from_ymd(2023, 5, 15);
    /// assert_eq!(date.0.to_string(), "2023-05-15");
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Date {
        Date(NaiveDate::from_ymd_opt(year, month, day).unwrap())
    }
}

impl FromStr for Date {
    type Err = anyhow::Error;

    /// Parses a date string in the format "YYYY-MM-DD" into a `Date` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use config::Date;
    /// use std::str::FromStr;
    ///
    /// let date = Date::from_str("2023-05-15").unwrap();
    /// assert_eq!(date.0.to_string(), "2023-05-15");
    ///
    /// assert!(Date::from_str("invalid-date").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
        Ok(Date(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_now() {
        let now = Date::now();
        assert_eq!(now.0, Utc::now().date_naive());
    }

    #[test]
    fn test_date_from_ymd() {
        let date = Date::from_ymd(2023, 5, 15);
        assert_eq!(date.0, NaiveDate::from_ymd_opt(2023, 5, 15).unwrap());
    }

    #[test]
    fn test_date_from_str_valid() {
        let date = Date::from_str("2023-05-15").unwrap();
        assert_eq!(date.0, NaiveDate::from_ymd_opt(2023, 5, 15).unwrap());
    }

    #[test]
    fn test_date_from_str_invalid() {
        assert!(Date::from_str("invalid-date").is_err());
    }

    #[test]
    fn test_date_ordering() {
        let date1 = Date::from_str("2023-05-15").unwrap();
        let date2 = Date::from_str("2023-05-16").unwrap();
        assert!(date1 < date2);
    }
}
