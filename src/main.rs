use actix_web::{web, App, HttpServer};
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

mod api;
mod models;

use crate::api::user::create_user;
use crate::api::user::greeter;
use crate::models::user::User;

type UserDB = Arc<Mutex<HashMap<u32, User>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let port: u16 = 8080;

  println!("Running server in port {}", port);

  let user_db: UserDB = Arc::new(Mutex::new(HashMap::new()));

  HttpServer::new(move || {
    let app_data = web::Data::new(user_db.clone());
    App::new().app_data(app_data)
      .service(greeter)
      .service(create_user)
  })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
