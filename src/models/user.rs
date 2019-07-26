use std::collections::HashMap;

pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub username: String,
    // pub login_attempts: u32,
    // pub last_login: String,
    // pub created_at: String,
    // pub updated_at: String
}

impl User {
    pub fn new(user: HashMap<&str, &str>) -> User {
        let id: u64 = user.get("id").unwrap().to_string().parse().unwrap();
        User {
            id: id as u64,
            first_name: user.get("first_name").unwrap().to_string(),
            last_name: user.get("last_name").unwrap().to_string(),
            username: user.get("username").unwrap().to_string(),
            password: user.get("password").unwrap().to_string()
        }
    }

    pub fn fullname(&self) -> String {
        let full_name = format!("{} {}", self.first_name, &self.last_name);
        full_name
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}