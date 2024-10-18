use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: UserID,
    pub email: Email,

    pub username: Option<UserName>,
}

impl User {
    pub fn new(id: &str, email: &str) -> Result<Self> {
        Ok(User {
            id: id.parse()?,
            email: email.parse()?,
            username: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Role {
    pub id: RoleID,
    pub name: RoleName,

    pub url: Option<String>,
    pub model_class: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

impl Role {
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self> {
        Ok(Role {
            id: id.parse()?,
            name: name.parse()?,
            url: None,
            model_class: None,
            r#type: None,
            description: Some(description.to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Group {
    pub id: GroupID,
    pub name: GroupName,

    pub model_class: Option<String>,
    pub url: Option<String>,
    pub roles_url: Option<String>,
    pub users_url: Option<String>,
}

impl Group {
    pub fn new(id: &str, name: &str) -> Result<Self> {
        Ok(Group {
            id: id.parse()?,
            name: name.parse()?,
            model_class: None,
            url: None,
            roles_url: None,
            users_url: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleDefinitionModel {
    pub name: RoleName,
    pub description: String,
    pub user_ids: Option<Vec<UserID>>,
    pub group_ids: Option<Vec<GroupID>>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupCreatePayload {
    pub name: GroupName,
    pub user_ids: Option<Vec<UserID>>,
    pub role_ids: Option<Vec<RoleID>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupUpdatePayload {
    pub name: Option<GroupName>,
    pub user_ids: Option<Vec<UserID>>,
    pub role_ids: Option<Vec<RoleID>>,
}

/*
 * NewTypes
 */


#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct UserID(String);

impl Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserID {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserID(s.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct RoleID(String);

impl Display for RoleID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RoleID {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RoleID(s.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct GroupID(String);

impl Display for GroupID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for GroupID {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GroupID(s.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct UserName(String);

impl Display for UserName { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserName {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserName(s.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct RoleName(String);

impl Display for RoleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RoleName {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RoleName(s.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct GroupName(String);

impl Display for GroupName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for GroupName {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GroupName(s.to_string()))
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Email(String);

fn is_valid_email(email: &str) -> bool {
    let email = email.split('@').collect::<Vec<&str>>();
    if email.len() != 2 {
        return false;
    }
    true
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Email {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_email(s) {
            Ok(Email(s.to_string()))
        } else {
            Err(anyhow::anyhow!("invalid email"))
        }
    }
}
