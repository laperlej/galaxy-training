mod config;
mod client;
mod mock;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[async_trait]
pub trait GalaxyAPI: GroupRepository + RoleRepository + UserRepository {
}

#[async_trait]
pub trait GroupRepository {
    async fn get_groups(&self) -> Result<Vec<Group>>;
    async fn create_group(&mut self, name: &str) -> Result<Group>;
    async fn update_group(&mut self, group_id: &str, payload: GroupUpdatePayload) -> Result<Group>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupUpdatePayload {
    pub name: Option<String>,
    pub user_ids: Option<Vec<String>>,
    pub role_ids: Option<Vec<String>>,
}

#[async_trait]
pub trait GroupRoleRepository {
    async fn get_group_roles(&self, group_id: &str) -> Result<Vec<Role>>;
    async fn add_role_to_group(&mut self, role_id: &str, group_id: &str) -> Result<()>;
}

#[async_trait]
pub trait GroupUserRepository {
    async fn get_group_users(&self, group_id: &str) -> Result<Vec<User>>;
    async fn add_user_to_group(&mut self, user_id: &str, group_id: &str) -> Result<()>;
}

#[async_trait]
pub trait RoleRepository {
    async fn get_roles(&self) -> Result<Vec<Role>>;
    async fn create_role(&mut self, name: &str, description: &str) -> Result<Role>;
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> Result<Vec<User>>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,

    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Role {
    pub id: String,
    pub name: String,

    pub url: Option<String>,
    pub model_class: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleDefinitionModel {
    pub name: String,
    pub description: String,
    pub user_ids: Option<Vec<String>>,
    pub group_ids: Option<Vec<String>>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Group {
    pub id: String,
    pub name: String,

    pub model_class: Option<String>,
    pub url: Option<String>,
    pub roles_url: Option<String>,
    pub users_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupCreatePayload {
    pub name: String,
    pub user_ids: Option<Vec<String>>,
    pub role_ids: Option<Vec<String>>,
}

impl User {
    pub fn new(id: &str, email: &str) -> Self {
        User {
            id: id.to_string(),
            email: email.to_string(),
            username: None,
        }
    }
}

impl Role {
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Role {
            id: id.to_string(),
            name: name.to_string(),
            url: None,
            model_class: None,
            r#type: None,
            description: Some(description.to_string()),
        }
    }
}

impl Group {
    pub fn new(id: &str, name: &str) -> Self {
        Group {
            id: id.to_string(),
            name: name.to_string(),
            model_class: None,
            url: None,
            roles_url: None,
            users_url: None,
        }
    }
}

pub struct Galaxy {
    client: client::Client,
}

impl Galaxy {
    pub fn new() -> Result<Galaxy> {
        let config = config::Config::new()?;
        Ok(Galaxy {
            client: client::Client::new(config),
        })
    }
}

impl GalaxyAPI for Galaxy {
}

#[async_trait]
impl UserRepository for Galaxy {
    async fn get_users(&self) -> Result<Vec<User>> {
        let response = self.client.get("/api/users").await?;
        let users: Vec<User> = response.json().await?;
        Ok(users)
    }
}

#[async_trait]
impl RoleRepository for Galaxy {
    async fn get_roles(&self) -> Result<Vec<Role>> {
        let response = self.client.get("/api/roles").await?;
        let roles: Vec<Role> = response.json().await?;
        Ok(roles)
    }

    async fn create_role(&mut self, name: &str, description: &str) -> Result<Role> {
        let role = RoleDefinitionModel {
            name: name.to_string(),
            description: description.to_string(),
            user_ids: None,
            group_ids: None,
        };
        let response = self.client.post("/api/roles", role).await?;
        let new_role: Role = response.json().await?;
        Ok(new_role)
    }
}

#[async_trait]
impl GroupRepository for Galaxy {
    async fn get_groups(&self) -> Result<Vec<Group>> {
        let response = self.client.get("/api/groups").await?;
        let groups: Vec<Group> = response.json().await?;
        Ok(groups)
    }

    async fn create_group(&mut self, name: &str) -> Result<Group> {
        let group = GroupCreatePayload {
            name: name.to_string(),
            user_ids: None,
            role_ids: None,
        };
        let response = self.client.post("/api/groups", group).await?;
        let new_group: Group = response.json().await?;
        Ok(new_group)
    }

    async fn update_group(&mut self, group_id: &str, payload: GroupUpdatePayload) -> Result<Group> {
        let endpoint = format!("/api/groups/{}", group_id);
        let response = self.client.put(endpoint.as_str(), payload).await?;
        let new_group: Group = response.json().await?;
        Ok(new_group)
    }
}

#[async_trait]
impl GroupUserRepository for Galaxy {
    async fn get_group_users(&self, group_id: &str) -> Result<Vec<User>> {
        let endpoint = format!("/api/groups/{}/users", group_id);
        let response = self.client.get(endpoint.as_str()).await?;
        let users: Vec<User> = response.json().await?;
        Ok(users)
    }

    async fn add_user_to_group(&mut self, user_id: &str, group_id: &str) -> Result<()> {
        let endpoint = format!("/api/groups/{}/user/{}", group_id, user_id);
        let _ = self.client.put(endpoint.as_str(), ()).await?;
        Ok(())
    }
}

#[async_trait]
impl GroupRoleRepository for Galaxy {
    async fn get_group_roles(&self, group_id: &str) -> Result<Vec<Role>> {
        let endpoint = format!("/api/groups/{}/roles", group_id);
        let response = self.client.get(endpoint.as_str()).await?;
        let roles: Vec<Role> = response.json().await?;
        Ok(roles)
    }

    async fn add_role_to_group(&mut self, role_id: &str, group_id: &str) -> Result<()> {
        let endpoint = format!("/api/groups/{}/roles/{}", group_id, role_id);
        let _ = self.client.put(endpoint.as_str(), ()).await?;
        Ok(())
    }
}
