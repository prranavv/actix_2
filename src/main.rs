use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::web::Data;
use actix_web::{App, HttpServer};
mod api;

use api::create_user::create;
use api::health::health;
use api::create_user::get_private_key;

type UserDB = Arc<Mutex<HashMap<String,String>>>;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Server running on port 8000");
    let user_db:UserDB=Arc::new(Mutex::new(HashMap::<String,String>::new()));
    HttpServer::new(move||{
        let data = Data::new(user_db.clone());
        App::new()
        .app_data(data)
        .service(create)  
        .service(health)
        .service(get_private_key)
    })
    .bind(("0.0.0.0",8000))?
    .run()
    .await
}