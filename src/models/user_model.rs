use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub name: String,
    pub email: String,
}