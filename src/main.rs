mod config;
mod galaxy;

use anyhow::Result;
use std::collections::{HashSet, HashMap};
use crate::galaxy::types::*;
use crate::galaxy::GalaxyAPI;
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

    async fn create_missing_roles(&mut self, missing_roles: impl Iterator<Item=&RoleName>) -> Result<()> {
        for role in missing_roles {
            let role = self.galaxy.create_role(&role.to_string(), "").await?;
            println!("Created role {}", role.name);
        }
        Ok(())
    }

    async fn create_missing_groups(&mut self, missing_groups: impl Iterator<Item=&GroupName>) -> Result<()> {
        for group in missing_groups {
            let group = self.galaxy.create_group(&group.to_string()).await?;
            println!("Created group {}", group.name);
        }
        Ok(())
    }


    async fn apply_config(&mut self, config: &config::ConfigFile) -> Result<()> {
        let (users, roles, groups) = tokio::join!(
            self.galaxy.get_users(),
            self.galaxy.get_roles(),
            self.galaxy.get_groups()
        );
        let (users, roles, groups) = (users?, roles?, groups?);

        let galaxy_users: HashMap<Email, User> = HashMap::from_iter(users.iter().map(|user| (user.email.clone(), user.clone())));
        let galaxy_groups: HashSet<GroupName> = HashSet::from_iter(groups.iter().map(|group| group.name.clone()));
        let galaxy_roles: HashSet<RoleName> = HashSet::from_iter(roles.iter().map(|role| role.name.clone()));

        let training_role_name: RoleName = "training".parse()?;
        let config_roles: HashSet<RoleName> = HashSet::from([training_role_name.clone()]);
        let config_groups: HashSet<GroupName> = HashSet::from_iter(config.groups.iter().map(|group| group.0.clone()));

        let missing_roles = config_roles.difference(&galaxy_roles);
        let missing_groups = config_groups.difference(&galaxy_groups);

        self.create_missing_roles(missing_roles).await?;
        self.create_missing_groups(missing_groups).await?;

        let training_role_id = roles.iter().find(|role| role.name == training_role_name).unwrap().id.clone();

        let today = Date::now();

        for group_name in config.groups.keys() {
            let group_users = config.groups.get(group_name).unwrap();
            let schedule = config.schedule.get(group_name).unwrap();
            let group = groups.iter().find(|group| group.name == *group_name).unwrap();
            let mut user_ids = Vec::new();
            let mut role_ids = Vec::new();

            for email in group_users.iter() {
                let user = galaxy_users.get(email).unwrap();
                user_ids.push(user.id.clone());
            }

            if schedule.iter().any(|schedule_item| schedule_item.contains(&today)) {
                role_ids.push(training_role_id.clone());
            }

            let payload = GroupUpdatePayload {
                name: Some(group_name.clone()),
                user_ids: Some(user_ids),
                role_ids: Some(role_ids),
            };

            self.galaxy.update_group(&group.id, &payload).await?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 1 {
        println!("Usage: training-manager <config-file>");
        return Ok(());
    }
    let config_path = &args[0];
    let config: config::ConfigFile = config::read_config(config_path)?;

    let galaxy = galaxy::init_galaxy()?;

    let mut training_manager = TrainingManager::new(Box::new(galaxy));

    training_manager.apply_config(&config).await?;

    Ok(())
}

