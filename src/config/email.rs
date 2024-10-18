use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Email(pub String);

fn is_valid_email(email: &str) -> bool {
    let email = email.split('@').collect::<Vec<&str>>();
    if email.len() != 2 {
        return false;
    }
    true
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
