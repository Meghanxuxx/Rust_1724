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
        let path = "./users.json".to_string(); // where we store the users
        
        // read the file if it exists
        let user_list = if Path::new(&path).exists() {
            let file_data = match fs::read_to_string(&path) {
                Ok(data) => data,
                Err(error) => return Err(format!("cant read file error: {}", error))
            };
            
            // try to parse json
            match serde_json::from_str(&file_data) {
                Ok(users) => users,
                Err(error) => return Err(format!("json broke error: {}", error))
            }
        } else {
            Vec::new()  // no file = empty list
        };

        Ok(UserStore { 
            file_path: path,
            users: user_list   // store the users
        })
    }

    // save to json file
    fn save_users(&self) -> Result<(), String> {
        let data = match serde_json::to_string_pretty(&self.users) {
            Ok(json) => json,
            Err(error) => return Err(format!("Can't make json: {}", error))
        };
        
        // try to save it
        if let Err(error) = fs::write(&self.file_path, data) {
            return Err(format!("couldnt save file: {}", error));
        }
        
        Ok(())
    }

    // add a new user
    pub fn add_user(&mut self, user: User) -> Result<bool, String> {
        // check if username already exists
        let name_exists = self.users
            .iter()
            .any(|existing_user| existing_user.name == user.name);
        
        // if exists, can't add
        if name_exists {
            return Ok(false);
        }

        // add user and save
        self.users.push(user);
        match self.save_users() {
            Ok(_) => Ok(true),
            Err(error) => Err(error)
        }
    }

    // check login info
    pub fn verify_login(&self, name: &str, pass: &str) -> Result<bool, String> {
        // just loop through and check
        for user in &self.users {
            if user.name == name {
                return Ok(user.password == pass);
            }
        }
        Ok(false)  // user not found
    }
} 