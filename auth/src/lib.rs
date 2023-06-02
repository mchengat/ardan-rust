use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}
pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "mansoor"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    fn new(username: &str, password: &str, role: LoginRole) -> Self {
        User {
            username: username.to_lowercase(),
            password: password.to_lowercase(),
            role,
        }
    }
}

pub fn get_default_users() -> HashMap<String, User> {
    // vec![
    //     User::new("admin", "password", LoginRole::Admin),
    //     User::new("bob", "password", LoginRole::User),
    // ]
    let mut users = HashMap::new();
    users.insert(
        "admin".to_owned(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_owned(),
        User::new("bob", "password", LoginRole::User),
    );
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).expect("Failed to read file");
        serde_json::from_str(&users_json).expect("Unable to deserialize")
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).expect("Serialization failed");
        std::fs::write(users_path, users_json).expect("file error");
        users
    }
}

#[allow(clippy::needless_return)]
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    match users.get(username) {
        Some(user) => {
            if user.password == password {
                return Some(LoginAction::Granted(user.role));
            } else {
                return Some(LoginAction::Denied);
            }
        }
        None => {
            return None;
        }
    }
    // match users.iter().find(|user| user.username == username) {
    //     Some(user) => {
    //         if user.password == password {
    //             return Some(LoginAction::Granted(user.role));
    //         } else {
    //             return Some(LoginAction::Denied);
    //         }
    //     }
    //     None => {
    //         return None;
    //     }
    // }
    // if username == "admin" && password == "password" {
    //     Some(LoginAction::Granted(LoginRole::Admin))
    // } else if username == "bob" && password == "password" {
    //     Some(LoginAction::Granted(LoginRole::User))
    // } else {
    //     // LoginAction::Denied
    //     None
    // }
}

#[cfg(test)]
mod test {
    use crate::{login, LoginAction};

    #[test]
    fn test_enums() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(crate::LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(crate::LoginRole::User))
        );
        assert_eq!(login("admin", "wrong"), Some(LoginAction::Denied));
        assert_eq!(login("wrong", "password"), Some(LoginAction::Denied));
    }
}
