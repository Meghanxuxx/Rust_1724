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
    pub step: i32,           // 当前是第几步 (1-3)
    pub content: String,    // 用户输入的内容
    pub user_id: Option<String>,     // 用户ID，未登录时为空
}
