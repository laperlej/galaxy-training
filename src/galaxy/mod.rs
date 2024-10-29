mod config;
pub mod types;
mod client;
mod mock;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use crate::galaxy::types::*;

pub fn init_galaxy() -> Result<crate::galaxy::Galaxy> {
    let env_var_provider = crate::galaxy::config::RealEnvVarProvider::new();
    let galaxy_config = crate::galaxy::config::Config::new(&env_var_provider)?;
    Galaxy::new(galaxy_config)
}

#[async_trait]
pub trait GalaxyAPI: GroupRepository + RoleRepository + UserRepository {
}

#[async_trait]
pub trait GroupRepository {
    async fn get_groups(&self) -> Result<Vec<Group>>;
    async fn create_group(&mut self, name: &str) -> Result<Group>;
    async fn update_group(&mut self, group_id: &GroupID, payload: &GroupUpdatePayload) -> Result<Group>;
}


#[async_trait]
pub trait GroupRoleRepository {
    async fn get_group_roles(&self, group_id: &GroupID) -> Result<Vec<Role>>;
    async fn add_role_to_group(&mut self, role_id: &RoleID, group_id: &GroupID) -> Result<()>;
}

#[async_trait]
pub trait GroupUserRepository {
    async fn get_group_users(&self, group_id: &GroupID) -> Result<Vec<User>>;
    async fn add_user_to_group(&mut self, user_id: &UserID, group_id: &GroupID) -> Result<()>;
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


pub struct Galaxy {
    client: client::Client,
}

impl Galaxy {
    pub fn new(config: config::Config) -> Result<Galaxy> {
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
        if response.status() != 200 {
            return Err(anyhow!("Failed to get users: {:?}", response));
        }
        let users: Vec<User> = response.json().await?;
        Ok(users)
    }
}

#[async_trait]
impl RoleRepository for Galaxy {
    async fn get_roles(&self) -> Result<Vec<Role>> {
        let response = self.client.get("/api/roles").await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to get roles: {:?}", response));
        }
        let roles: Vec<Role> = response.json().await?;
        Ok(roles)
    }

    async fn create_role(&mut self, name: &str, description: &str) -> Result<Role> {
        let role = RoleDefinitionModel {
            name: name.parse()?,
            description: description.to_string(),
            user_ids: None,
            group_ids: None,
        };
        let response = self.client.post("/api/roles", role).await?;
        if response.status() != 201 {
            return Err(anyhow!("Failed to create role: {:?}", response));
        }
        let new_role: Role = response.json().await?;
        Ok(new_role)
    }
}

#[async_trait]
impl GroupRepository for Galaxy {
    async fn get_groups(&self) -> Result<Vec<Group>> {
        let response = self.client.get("/api/groups").await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to get groups: {:?}", response));
        }
        let groups: Vec<Group> = response.json().await?;
        Ok(groups)
    }

    async fn create_group(&mut self, name: &str) -> Result<Group> {
        let group = GroupCreatePayload {
            name: name.parse()?,
            user_ids: None,
            role_ids: None,
        };
        let response = self.client.post("/api/groups", group).await?;
        if response.status() != 201 {
            return Err(anyhow!("Failed to create group: {:?}", response));
        }
        let new_group: Group = response.json().await?;
        Ok(new_group)
    }

    async fn update_group(&mut self, group_id: &GroupID, payload: &GroupUpdatePayload) -> Result<Group> {
        let endpoint = format!("/api/groups/{}", group_id);
        let response = self.client.put(endpoint.as_str(), payload).await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to update group: {:?}", response));
        }
        let new_group: Group = response.json().await?;
        Ok(new_group)
    }
}

#[async_trait]
impl GroupUserRepository for Galaxy {
    async fn get_group_users(&self, group_id: &GroupID) -> Result<Vec<User>> {
        let endpoint = format!("/api/groups/{}/users", group_id);
        let response = self.client.get(endpoint.as_str()).await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to get group users: {:?}", response));
        }
        let users: Vec<User> = response.json().await?;
        Ok(users)
    }

    async fn add_user_to_group(&mut self, user_id: &UserID, group_id: &GroupID) -> Result<()> {
        let endpoint = format!("/api/groups/{}/user/{}", group_id, user_id);
        let response = self.client.put(endpoint.as_str(), ()).await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to add user to group: {:?}", response));
        }
        Ok(())
    }
}

#[async_trait]
impl GroupRoleRepository for Galaxy {
    async fn get_group_roles(&self, group_id: &GroupID) -> Result<Vec<Role>> {
        let endpoint = format!("/api/groups/{}/roles", group_id);
        let response = self.client.get(endpoint.as_str()).await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to get group roles: {:?}", response));
        }
        let roles: Vec<Role> = response.json().await?;
        Ok(roles)
    }

    async fn add_role_to_group(&mut self, role_id: &RoleID, group_id: &GroupID) -> Result<()> {
        let endpoint = format!("/api/groups/{}/roles/{}", group_id, role_id);
        let response = self.client.put(endpoint.as_str(), ()).await?;
        if response.status() != 200 {
            return Err(anyhow!("Failed to add role to group: {:?}", response));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::runtime::Runtime;

    fn setup_test() -> (mockito::ServerGuard, Galaxy, Runtime) {
        let server = mockito::Server::new();
        let config = config::Config {
            galaxy_url: server.url(),
            api_key: "test-api-key".to_string(),
        };
        let galaxy = Galaxy::new(config).unwrap();
        let runtime = Runtime::new().unwrap();
        (server, galaxy, runtime)
    }

    #[test]
    fn test_get_users() {
        let (mut server, galaxy, runtime) = setup_test();
        let mock = server.mock("GET", "/api/users")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .with_body(r#"[{"id": "1", "username": "john", "email": "john@example.com"}]"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.get_users().await.unwrap();
            assert_eq!(response.len(), 1);
        });

        mock.assert();
    }

    #[test]
    fn test_get_roles() {
        let (mut server, galaxy, runtime) = setup_test();
        let mock = server.mock("GET", "/api/roles")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .with_body(r#"[{"id": "1", "name": "John Doe", "description": "john@example.com"}]"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.get_roles().await.unwrap();
            assert_eq!(response.len(), 1);
        });

        mock.assert();
    }

    #[test]
    fn test_create_role() {
        let (mut server, mut galaxy, runtime) = setup_test();
        let mock = server.mock("POST", "/api/roles")
            .match_header("x-api-key", "test-api-key")
            .match_body(mockito::Matcher::PartialJson(json!({"name":"John Doe","description":"john@example.com"})))
            .with_status(201)
            .with_body(r#"{"id": "1", "name": "John Doe", "description": "john@example.com"}"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.create_role("John Doe", "john@example.com").await.unwrap();
            assert_eq!(response.id, "1".parse::<RoleID>().unwrap());
        });

        mock.assert();
    }

    #[test]
    fn test_get_groups() {
        let (mut server, galaxy, runtime) = setup_test();
        let mock = server.mock("GET", "/api/groups")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .with_body(r#"[{"id": "1", "name": "Group 1"}]"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.get_groups().await.unwrap();
            assert_eq!(response.len(), 1);
        });

        mock.assert();
    }

    #[test]
    fn test_create_group() {
        let (mut server, mut galaxy, runtime) = setup_test();
        let mock = server.mock("POST", "/api/groups")
            .match_header("x-api-key", "test-api-key")
            .match_body(mockito::Matcher::PartialJson(json!({"name":"Group 1"})))
            .with_status(201)
            .with_body(r#"{"id": "1", "name": "Group 1"}"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.create_group("Group 1").await.unwrap();
            assert_eq!(response.id, "1".parse::<GroupID>().unwrap());
        });

        mock.assert();
    }

    #[test]
    fn test_update_group() {
        let (mut server, mut galaxy, runtime) = setup_test();
        let mock = server.mock("PUT", "/api/groups/1")
            .match_header("x-api-key", "test-api-key")
            .match_body(mockito::Matcher::PartialJson(json!({"name":"Group 1"})))
            .with_status(200)
            .with_body(r#"{"id": "1", "name": "Group 1"}"#)
            .create();
        let payload = GroupUpdatePayload {
            name: Some("Group 1".parse().unwrap()),
            role_ids: None,
            user_ids: None,
        };
        runtime.block_on(async {
            let response = galaxy.update_group(&"1".parse::<GroupID>().unwrap(), &payload).await.unwrap();
            assert_eq!(response.id, "1".parse::<GroupID>().unwrap());
        });

        mock.assert();
    }

    #[test]
    fn test_get_group_users() {
        let (mut server, galaxy, runtime) = setup_test();
        let mock = server.mock("GET", "/api/groups/1/users")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .with_body(r#"[{"id": "1", "username": "john", "email": "john@example.com"}]"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.get_group_users(&"1".parse::<GroupID>().unwrap()).await.unwrap();
            assert_eq!(response.len(), 1);
        });

        mock.assert();
    }

    #[test]
    fn test_add_user_to_group() {
        let (mut server, mut galaxy, runtime) = setup_test();
        let mock = server.mock("PUT", "/api/groups/1/user/2")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .create();

        runtime.block_on(async {
            galaxy.add_user_to_group(&"2".parse::<UserID>().unwrap(), &"1".parse::<GroupID>().unwrap()).await.unwrap();
        });

        mock.assert();
    }

    #[test]
    fn test_get_group_roles() {
        let (mut server, galaxy, runtime) = setup_test();
        let mock = server.mock("GET", "/api/groups/1/roles")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .with_body(r#"[{"id": "1", "name": "John Doe", "description": "john@example.com"}]"#)
            .create();

        runtime.block_on(async {
            let response = galaxy.get_group_roles(&"1".parse::<GroupID>().unwrap()).await.unwrap();
            assert_eq!(response.len(), 1);
        });

        mock.assert();
    }

    #[test]
    fn test_add_role_to_group() {
        let (mut server, mut galaxy, runtime) = setup_test();
        let mock = server.mock("PUT", "/api/groups/1/roles/2")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .create();

        runtime.block_on(async {
            galaxy.add_role_to_group(&"2".parse::<RoleID>().unwrap(), &"1".parse::<GroupID>().unwrap()).await.unwrap();
        });

        mock.assert();
    }
}
