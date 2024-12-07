use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoverLetterInput {
    pub step: i32,
    pub content: String,
    pub user_id: Option<String>, 
} 