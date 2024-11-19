use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub name: String,
  pub email: String,
  pub password: String,
}
