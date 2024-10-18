use crate::galaxy::{GalaxyAPI, GroupRepository, GroupRoleRepository, GroupUserRepository, RoleRepository, UserRepository, Group, Role, User, GroupUpdatePayload};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use async_trait::async_trait;
use crate::galaxy::types::*;

/*
 * MockGalaxy
 */

struct  MockGalaxy {
    id_generator: IDGenerator,
    users: HashMap<UserID, User>,
    roles: HashMap<RoleID, Role>,
    groups: HashMap<GroupID, Group>,
    group_roles: HashMap<GroupID, HashSet<RoleID>>,
    group_users: HashMap<GroupID, HashSet<UserID>>,
}

impl MockGalaxy {
    fn new() -> Result<Self> {
        let id_generator = IDGenerator::new();
        let mut users = HashMap::new();
        let mut roles = HashMap::new();
        let mut groups = HashMap::new();
        let mut group_roles = HashMap::new();
        let mut group_users = HashMap::new();
        users.insert("user1".parse()?, User::new("user1", "user1@email.com")?);
        roles.insert("role1".parse()?, Role::new("role1", "role1", "role1 description")?);
        groups.insert("group1".parse()?, Group::new("group1", "group1")?);
        group_roles.insert("group1".parse()?, HashSet::new());
        group_users.insert("group1".parse()?, HashSet::new());
        Ok(MockGalaxy {
            id_generator,
            users,
            roles,
            groups,
            group_roles,
            group_users,
        })
    }
}

impl GalaxyAPI for MockGalaxy {
}

#[async_trait]
impl GroupRepository for MockGalaxy {
    async fn get_groups(&self) -> Result<Vec<Group>> {
        Ok(self.groups.values().cloned().collect())
    }

    async fn create_group(&mut self, name: &str) -> Result<Group> {
        let id =  self.id_generator.next();
        let group = Group::new(&id, name)?;
        self.groups.insert(group.id.clone(), group.clone());
        self.group_roles.insert(group.id.clone(), HashSet::new());
        self.group_users.insert(group.id.clone(), HashSet::new());
        Ok(group)
    }

    async fn update_group(&mut self, group_id: &GroupID, payload: &GroupUpdatePayload) -> Result<Group> {
        let group = match self.groups.get_mut(group_id) {
            Some(group) => group,
            None => return Err(anyhow::anyhow!("group {} does not exist", group_id)),
        };
        if let Some(name) = &payload.name {
            group.name = name.clone();
        }
        if let Some(user_ids) = &payload.user_ids {
            self.group_users.insert(group_id.clone(), HashSet::from_iter(user_ids.clone()));
        }
        if let Some(role_ids) = &payload.role_ids {
            self.group_roles.insert(group_id.clone(), HashSet::from_iter(role_ids.clone()));
        }
        Ok(group.clone())
    }
}

#[async_trait]
impl GroupRoleRepository for MockGalaxy {
    async fn get_group_roles(&self, group_id: &GroupID) -> Result<Vec<Role>> {
        let group_roles = match self.group_roles.get(group_id) {
            Some(group_roles) => group_roles.clone(),
            None => HashSet::new(),
        };
        Ok(group_roles.iter()
            .filter_map(|role_id| self.roles.get(role_id))
            .cloned()
            .collect())
        
    }

    async fn add_role_to_group(&mut self, role_id: &RoleID, group_id: &GroupID) -> Result<()> {
        if !self.roles.contains_key(role_id) {
            return Err(anyhow::anyhow!("role {} does not exist", role_id));
        }
        match self.group_roles.get_mut(group_id) {
            Some(group_roles) => {
                let _ = group_roles.insert(role_id.clone());
                Ok(())
            },
            None => Err(anyhow::anyhow!("group {} does not exist", group_id)),
        }
    }
}

#[async_trait]
impl GroupUserRepository for MockGalaxy {
    async fn get_group_users(&self, group_id: &GroupID) -> Result<Vec<User>> {
        let group_users = match self.group_users.get(group_id) {
            Some(group_users) => group_users.clone(),
            None => HashSet::new(),
        };
        Ok(group_users.iter()
            .filter_map(|user_id| self.users.get(user_id))
            .cloned()
            .collect())
    }
    async fn add_user_to_group(&mut self, user_id: &UserID, group_id: &GroupID) -> Result<()> {
        if !self.users.contains_key(user_id) {
            return Err(anyhow::anyhow!("user {} does not exist", user_id));
        }
        match self.group_users.get_mut(group_id) {
            Some(group_users) => {
                let _ = group_users.insert(user_id.clone());
                Ok(())
            },
            None => Err(anyhow::anyhow!("group {} does not exist", group_id)),
        }
    }
}

#[async_trait]
impl RoleRepository for MockGalaxy {
    async fn get_roles(&self) -> Result<Vec<Role>> {
        Ok(self.roles.values().cloned().collect())
    }

    async fn create_role(&mut self, name: &str, description: &str) -> Result<Role> {
        let id =  self.id_generator.next();
        let role = Role::new(&id, name, description)?;
        self.roles.insert(id.parse()?, role.clone());
        Ok(role)
    }
}

#[async_trait]
impl UserRepository for MockGalaxy {
    async fn get_users(&self) -> Result<Vec<User>> {
        Ok(self.users.values().cloned().collect())
    }
}

/*
 * IDGenerator
 */

struct IDGenerator {
    id: u32,
}

impl IDGenerator {
    fn new() -> Self {
        IDGenerator {
            id: 0,
        }
    }
    fn next(&mut self) -> String {
        let id = self.id;
        self.id += 1;
        id.to_string()
    }
}

/*
 * Tests
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_groups() {
        let mut galaxy = MockGalaxy::new().unwrap();
        let group = galaxy.create_group("group2").await.unwrap();
        let groups = galaxy.get_groups().await.unwrap();
        assert!(groups.contains(&group));
    }

    #[tokio::test]
    async fn test_update_group() {
        let mut galaxy = MockGalaxy::new().unwrap();
        let group = galaxy.create_group("group2").await.unwrap();
        let user_ids = vec!["user1".parse().unwrap()];
        let role_ids = vec!["role1".parse().unwrap()];
        let group = galaxy.update_group(&group.id, &GroupUpdatePayload {
            name: Some("group2 updated".parse().unwrap()),
            user_ids: Some(user_ids),
            role_ids: Some(role_ids),
        }).await.unwrap();
        let after = galaxy.get_groups().await.unwrap();
        let after_roles = galaxy.get_group_roles(&group.id).await.unwrap();
        let after_users = galaxy.get_group_users(&group.id).await.unwrap();
        assert!(after.contains(&group));
        assert_eq!(after_roles.len(), 1);
        assert_eq!(after_users.len(), 1);
    }

    #[tokio::test]
    async fn test_roles() {
        let mut galaxy = MockGalaxy::new().unwrap();
        let roles = galaxy.get_roles().await.unwrap();
        assert_eq!(roles.len(), 1);
        let role = galaxy.create_role("role2", "role2 description").await.unwrap();
        let roles = galaxy.get_roles().await.unwrap();
        assert!(roles.contains(&role));
    }

    #[tokio::test]
    async fn test_users() {
        let galaxy = MockGalaxy::new().unwrap();
        let users = galaxy.get_users().await.unwrap();
        let email = "user1@email.com".parse().unwrap();
        assert!(users.iter().map(|user| user.email.clone()).collect::<HashSet<Email>>().contains(&email));
    }

    #[tokio::test]
    async fn test_group_roles() {
        let mut galaxy = MockGalaxy::new().unwrap();
        let before = galaxy.get_group_roles(&"group1".parse().unwrap()).await.unwrap();
        assert_eq!(before.len(), 0);
        galaxy.add_role_to_group(&"role1".parse().unwrap(), &"group1".parse().unwrap()).await.unwrap();
        let after = galaxy.get_group_roles(&"group1".parse().unwrap()).await.unwrap();
        assert_eq!(after.len(), 1);
    }

    #[tokio::test]
    async fn test_group_users() {
        let mut galaxy = MockGalaxy::new().unwrap();
        let before = galaxy.get_group_users(&"group1".parse().unwrap()).await.unwrap();
        assert_eq!(before.len(), 0);
        galaxy.add_user_to_group(&"user1".parse().unwrap(), &"group1".parse().unwrap()).await.unwrap();
        let after = galaxy.get_group_users(&"group1".parse().unwrap()).await.unwrap();
        assert_eq!(after.len(), 1);
    }
}
