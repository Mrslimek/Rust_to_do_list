use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Todo {
    pub id: i64,
    pub text: String,
}
