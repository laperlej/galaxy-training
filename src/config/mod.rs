pub mod date;

use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;
use crate::galaxy::types::{Email, GroupName};

use date::Date;

const TEST_CONFIG_FILE: &str = "./src/sample.toml";

#[derive(Debug)]
pub struct TimeRange {
    pub from: Date,
    pub to: Date,
}

impl TimeRange {
    pub fn contains(&self, date: &Date) -> bool {
        date.0 >= self.from.0 && date.0 <= self.to.0
    }
}

#[derive(Debug)]
pub struct ConfigFile {
    pub groups: HashMap<GroupName, Vec<Email>>,
    pub schedule: HashMap<GroupName, Vec<TimeRange>>,
}

impl FromStr for ConfigFile {
    type Err = anyhow::Error;
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

fn parse_schedule(schedule: &toml::Table) -> Result<HashMap<GroupName, Vec<TimeRange>>> {
    let mut schedule_map = HashMap::new();
    for (group_name, schedules) in schedule.iter() {
        let schedules = schedules.as_array().ok_or(anyhow::anyhow!("schedules not found"))?;        
        let schedules: Vec<TimeRange> = schedules.iter().map(parse_schedule_item).collect::<Result<Vec<TimeRange>>>()?;
        schedule_map.insert(group_name.parse()?, schedules);
    }
    Ok(schedule_map)
}

fn parse_schedule_item(schedule: &toml::Value) -> Result<TimeRange> {
    let schedule = schedule.as_table().ok_or(anyhow::anyhow!("schedule not found"))?;
    let from = schedule.get("from").ok_or(anyhow::anyhow!("from not found"))?.as_str().ok_or(anyhow::anyhow!("from not found"))?;
    let to = schedule.get("to").ok_or(anyhow::anyhow!("to not found"))?.as_str().ok_or(anyhow::anyhow!("to not found"))?;
    Ok(TimeRange {
        from: from.parse::<Date>()?,
        to: to.parse::<Date>()?,
    })
}

fn read_config() -> Result<ConfigFile> {
    let config = std::fs::read_to_string(TEST_CONFIG_FILE)?;
    let config: toml::Table = toml::from_str(&config)?;

    let groups_table = config.get("groups").ok_or(anyhow::anyhow!("groups not found"))?.as_table().ok_or(anyhow::anyhow!("groups not found"))?;
    let schedule_table = config.get("schedule").ok_or(anyhow::anyhow!("schedule not found"))?.as_table().ok_or(anyhow::anyhow!("schedule not found"))?;

    let groups = parse_groups(groups_table)?;
    let schedule = parse_schedule(schedule_table)?;

    Ok(ConfigFile {
        groups,
        schedule,
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_config() -> Result<()> {
        let _ = std::fs::read_to_string(TEST_CONFIG_FILE)?.parse::<ConfigFile>()?;
        Ok(())
    }
}
