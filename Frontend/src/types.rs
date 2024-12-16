use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub age: Option<i32>,
    pub gender: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoverLetterInput {
    pub step: i32,           // Current step (1-3)
    pub content: String,    // User input
    pub user_id: Option<String>,     // User ID, empty when not logged in
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub id: usize,
    pub content: String,
}