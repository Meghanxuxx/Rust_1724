use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// user info
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub age: Option<i32>,  // optional stuff
    pub gender: Option<String>,  // optional stuff
}

// store users in json file
pub struct UserStore {
    file_path: String,  // where we save the json
    users: Vec<User>,   // all our users
}

impl UserStore {
    // create new store
    pub fn new() -> Result<UserStore, String> {
        let file_path = "./users.json".to_string(); // where we store the users
        
        // read the file if it exists
        let users = if Path::new(&file_path).exists() {
            let content = match fs::read_to_string(&file_path) {
                Ok(data) => data,
                Err(e) => return Err(format!("cant read file error: {}", e))
            };
            
            // try to parse json
            match serde_json::from_str(&content) {
                Ok(user_list) => user_list,
                Err(e) => return Err(format!("json broke error: {}", e))
            }
        } else {
            Vec::new()  // no file = empty list
        };

        Ok(UserStore { 
            file_path,
            users   // store the users
        })
    }

    // save to json file
    fn save_to_file(&self) -> Result<(), String> {
        let json = match serde_json::to_string_pretty(&self.users) {
            Ok(data) => data,
            Err(e) => return Err(format!("Can't make json: {}", e))
        };
        
        // try to save it
        if let Err(e) = fs::write(&self.file_path, json) {
            return Err(format!("couldnt save file: {}", e));
        }
        
        Ok(())
    }

    // add a new user
    pub fn add_user(&mut self, new_user: User) -> Result<bool, String> {
        // check if username already exists
        let exists = self.users
            .iter()
            .any(|u| u.name == new_user.name);
        
        // if exists, can‘t add
        if exists {
            return Ok(false);
        }

        // add user and save
        self.users.push(new_user);
        match self.save_to_file() {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    // check login info
    pub fn verify_login(&self, username: &str, password: &str) -> Result<bool, String> {
        // just loop through and check
        for user in &self.users {
            if user.name == username {
                return Ok(user.password == password);
            }
        }
        Ok(false)  // user not found
    }
} 