mod config;
mod galaxy;

use anyhow::Result;
use std::collections::{HashSet, HashMap};
use crate::galaxy::{User, GroupUpdatePayload, GalaxyAPI};
use crate::config::date::Date;

struct TrainingManager {
    galaxy: Box<dyn GalaxyAPI>,
}

impl TrainingManager {
    fn new(galaxy: Box<dyn GalaxyAPI>) -> Self {
        TrainingManager {
            galaxy,
        }
    }

    async fn apply_config(&mut self, config: &config::ConfigFile) -> Result<()> {
        let users = self.galaxy.get_users().await?;
        let roles = self.galaxy.get_roles().await?;
        let groups = self.galaxy.get_groups().await?;

        let galaxy_users: HashMap<String, User> = HashMap::from_iter(users.iter().map(|user| (user.email.clone(), user.clone())));
        let galaxy_groups: HashSet<String> = HashSet::from_iter(groups.iter().map(|group| group.name.clone()));
        let galaxy_roles: HashSet<String> = HashSet::from_iter(roles.iter().map(|role| role.name.clone()));

        let config_roles: HashSet<String> = HashSet::from(["training".to_string()]);
        let config_groups: HashSet<String> = HashSet::from_iter(config.groups.iter().map(|group| group.0.clone()));

        let missing_roles = config_roles.difference(&galaxy_roles);
        for role in missing_roles {
            let role = self.galaxy.create_role(role.as_str(), "").await?;
            println!("Created role {}", role.name);
        }

        let missing_groups = config_groups.difference(&galaxy_groups);
        for group in missing_groups {
            let group = self.galaxy.create_group(group.as_str()).await?;
            println!("Created group {}", group.name);
        }

        let training_role_id = roles.iter().find(|role| role.name == "training").unwrap().id.clone();

        let today = Date::now();

        for group_name in config.groups.keys() {
            let group_users = config.groups.get(group_name).unwrap();
            let schedule = config.schedule.get(group_name).unwrap();
            let group = groups.iter().find(|group| group.name == *group_name).unwrap();
            let mut user_ids = Vec::new();
            let mut role_ids = Vec::new();

            for email in group_users.iter() {
                let user = galaxy_users.get(&email.0).unwrap();
                user_ids.push(user.id.clone());
            }

            if schedule.iter().any(|schedule_item| schedule_item.contains(&today)) {
                role_ids.push(training_role_id.clone());
            }

            let payload = GroupUpdatePayload {
                name: Some(group_name.to_string()),
                user_ids: Some(user_ids),
                role_ids: Some(role_ids),
            };

            self.galaxy.update_group(group.id.as_str(), payload).await?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: config::ConfigFile = std::fs::read_to_string("./src/sample.toml")?.parse()?;
    let galaxy = galaxy::Galaxy::new()?;

    let mut training_manager = TrainingManager::new(Box::new(galaxy));

    training_manager.apply_config(&config).await?;

    Ok(())
}

