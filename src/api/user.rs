
use actix_web::{web, HttpResponse, Responder, Error, Result};
use serde::Serialize;

use crate::UserDB;
use crate::models::user::User;

#[actix_web::get("/{id}")]
pub async fn greeter(id: web::Path<u32>, db: web::Data<UserDB>) -> Result<impl Responder, Error> {
  let user_id = id.into_inner();
  let db = db.lock().unwrap();

  match db.get(&user_id) {
    Some(user) => Ok(HttpResponse::Ok().json(user)),
    None => Err(actix_web::error::ErrorNotFound("User not found")),
  }
}

#[derive(Serialize)]
pub struct UserResponse {
  id: u32,
  name: String,
  email: String,
}

#[actix_web::post("/create-user")]
pub async fn create_user(userdata: web::Json<User>, db: web::Data<UserDB>) -> impl Responder {
  let mut db = db.lock().unwrap();
  let new_id = db.keys().max().unwrap_or(&0) + 1;
  let created_name = userdata.name.clone();
  let created_email = userdata.email.clone();
  db.insert(new_id, userdata.into_inner());

  HttpResponse::Created().json(
    UserResponse {
      id: new_id,
      name: created_name,
      email: created_email,
    }
  )
}
