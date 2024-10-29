//! # Configuration Module
//! 
//! This module provides functionality for parsing and managing configuration files
//! for a scheduling system. It includes structures for representing time ranges,
//! groups, and schedules, as well as functions for parsing TOML configuration files.
//! 
//! ## Example
//! 
//! ```
//! use your_crate_name::config::{ConfigFile, TimeRange, Date};
//! use std::str::FromStr;
//! 
//! let config_str = r#"
//! [groups]
//! team_a = ["alice@example.com", "bob@example.com"]
//! team_b = ["charlie@example.com", "david@example.com"]
//! 
//! [schedule]
//! team_a = [
//!     { from = "2023-01-01", to = "2023-06-30" },
//!     { from = "2023-07-01", to = "2023-12-31" }
//! ]
//! team_b = [
//!     { from = "2023-01-01", to = "2023-12-31" }
//! ]
//! "#;
//! 
//! let config: ConfigFile = ConfigFile::from_str(config_str).unwrap();
//! assert_eq!(config.groups.len(), 2);
//! assert_eq!(config.schedule.len(), 2);
//! ```

pub mod date;

use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;
use crate::galaxy::types::{Email, GroupName};

use date::Date;

/// Represents a time range with a start and end date.
#[derive(Debug)]
pub struct TimeRange {
    pub from: Date,
    pub to: Date,
}

impl TimeRange {
    /// Checks if a given date is within the time range.
    ///
    /// # Arguments
    ///
    /// * `date` - The date to check.
    ///
    /// # Returns
    ///
    /// `true` if the date is within the range, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::config::{TimeRange, Date};
    ///
    /// let range = TimeRange {
    ///     from: Date(2023, 1, 1),
    ///     to: Date(2023, 12, 31),
    /// };
    /// assert!(range.contains(&Date(2023, 6, 15)));
    /// assert!(!range.contains(&Date(2024, 1, 1)));
    /// ```
    pub fn contains(&self, date: &Date) -> bool {
        date.0 >= self.from.0 && date.0 <= self.to.0
    }
}

/// Represents the parsed configuration file.
#[derive(Debug)]
pub struct ConfigFile {
    /// Mapping of group names to lists of email addresses.
    pub groups: HashMap<GroupName, Vec<Email>>,
    /// Mapping of group names to lists of time ranges for scheduling.
    pub schedule: HashMap<GroupName, Vec<TimeRange>>,
}

impl FromStr for ConfigFile {
    type Err = anyhow::Error;

    /// Parses a string into a `ConfigFile`.
    ///
    /// # Arguments
    ///
    /// * `s` - The string containing the TOML configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `ConfigFile` or an error.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::config::ConfigFile;
    /// use std::str::FromStr;
    ///
    /// let config_str = r#"
    /// [groups]
    /// team_a = ["alice@example.com"]
    ///
    /// [schedule]
    /// team_a = [
    ///     { from = "2023-01-01", to = "2023-12-31" }
    /// ]
    /// "#;
    ///
    /// let config = ConfigFile::from_str(config_str).unwrap();
    /// assert_eq!(config.groups.len(), 1);
    /// assert_eq!(config.schedule.len(), 1);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config: toml::Table = toml::from_str(s)?;
        let groups_table = config.get("groups").ok_or(anyhow::anyhow!("groups not found"))?.as_table().ok_or(anyhow::anyhow!("groups not found"))?;
        let schedule_table = config.get("schedule").ok_or(anyhow::anyhow!("schedule not found"))?.as_table().ok_or(anyhow::anyhow!("schedule not found"))?;
        let groups = parse_groups(groups_table)?;
        let schedule = parse_schedule(schedule_table)?;
        Ok(ConfigFile {
            groups,
            schedule,
        })
    }
}

/// Parses the groups section of the configuration.
///
/// # Arguments
///
/// * `groups` - The TOML table containing group information.
///
/// # Returns
///
/// A `Result` containing a `HashMap` of group names to email lists.
fn parse_groups(groups: &toml::Table) -> Result<HashMap<GroupName, Vec<Email>>> {
    let mut groups_map = HashMap::new();
    for (group_name, emails) in groups.iter() {
        let email_array = emails.as_array().ok_or(anyhow::anyhow!("emails not found"))?;
        let emails =  email_array.iter()
            .map(|email| email.as_str())
            .map(|email| email.ok_or(anyhow::anyhow!("email not found")))
            .flat_map(|email| email.map(|email| email.parse::<Email>()))
            .collect::<Result<Vec<Email>>>()?;
        groups_map.insert(group_name.parse()?, emails);
    }
    Ok(groups_map)
}

/// Parses the schedule section of the configuration.
///
/// # Arguments
///
/// * `schedule` - The TOML table containing schedule information.
///
/// # Returns
///
/// A `Result` containing a `HashMap` of group names to lists of time ranges.
fn parse_schedule(schedule: &toml::Table) -> Result<HashMap<GroupName, Vec<TimeRange>>> {
    let mut schedule_map = HashMap::new();
    for (group_name, schedules) in schedule.iter() {
        let schedules = schedules.as_array().ok_or(anyhow::anyhow!("schedules not found"))?;        
        let schedules: Vec<TimeRange> = schedules.iter().map(parse_schedule_item).collect::<Result<Vec<TimeRange>>>()?;
        schedule_map.insert(group_name.parse()?, schedules);
    }
    Ok(schedule_map)
}

/// Parses a single schedule item.
///
/// # Arguments
///
/// * `schedule` - The TOML value representing a single schedule item.
///
/// # Returns
///
/// A `Result` containing a `TimeRange`.
fn parse_schedule_item(schedule: &toml::Value) -> Result<TimeRange> {
    let schedule = schedule.as_table().ok_or(anyhow::anyhow!("schedule not found"))?;
    let from = schedule.get("from").ok_or(anyhow::anyhow!("from not found"))?.as_str().ok_or(anyhow::anyhow!("from not found"))?;
    let to = schedule.get("to").ok_or(anyhow::anyhow!("to not found"))?.as_str().ok_or(anyhow::anyhow!("to not found"))?;
    Ok(TimeRange {
        from: from.parse::<Date>()?,
        to: to.parse::<Date>()?,
    })
}

/// Reads the configuration from a file.
///
/// # Arguments
///
/// * `path` - The path to the configuration file.
///
/// # Returns
///
/// A `Result` containing the parsed `ConfigFile`.
pub fn read_config(path: &str) -> Result<ConfigFile> {
    let config = std::fs::read_to_string(path)?;
    config.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const TEST_CONFIG_FILE: &str = "./src/sample.toml";

    #[test]
    fn test_time_range_contains() {
        let range = TimeRange {
            from: Date::from_ymd(2023, 1, 1),
            to: Date::from_ymd(2023, 12, 31),
        };
        assert!(range.contains(&Date::from_ymd(2023, 6, 15)));
        assert!(!range.contains(&Date::from_ymd(2022, 12, 31)));
        assert!(!range.contains(&Date::from_ymd(2024, 1, 1)));
    }

    #[test]
    fn test_parse_config() -> Result<()> {
        let config_str = r#"
        [groups]
        team_a = ["alice@example.com", "bob@example.com"]
        team_b = ["charlie@example.com"]

        [schedule]
        team_a = [
            { from = "2023-01-01", to = "2023-06-30" },
            { from = "2023-07-01", to = "2023-12-31" }
        ]
        team_b = [
            { from = "2023-01-01", to = "2023-12-31" }
        ]
        "#;

        let config: ConfigFile = ConfigFile::from_str(config_str)?;
        assert_eq!(config.groups.len(), 2);
        assert_eq!(config.schedule.len(), 2);
        assert_eq!(config.groups.get(&"team_a".parse()?).unwrap().len(), 2);
        assert_eq!(config.groups.get(&"team_b".parse()?).unwrap().len(), 1);
        assert_eq!(config.schedule.get(&"team_a".parse()?).unwrap().len(), 2);
        assert_eq!(config.schedule.get(&"team_b".parse()?).unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn test_read_config() -> Result<()> {
        let config = read_config(TEST_CONFIG_FILE)?;
        assert!(!config.groups.is_empty());
        assert!(!config.schedule.is_empty());
        Ok(())
    }

    #[test]
    fn test_read_config_not_found() {
        let config = read_config("not_found.toml");
        assert!(config.is_err());
    }

    #[test]
    fn test_parse_invalid_config() {
        let invalid_config = r#"
        [groups]
        team_a = ["invalid_email"]

        [schedule]
        team_a = [
            { from = "invalid_date", to = "2023-12-31" }
        ]
        "#;

        assert!(ConfigFile::from_str(invalid_config).is_err());
    }
}
