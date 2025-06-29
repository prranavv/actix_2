use actix_web::{App, HttpServer};
mod api;

use api::create_user::create;
use api::health::health;


#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Server running on port 8000");
    HttpServer::new(move||{
        App::new()
        .service(create)  
        .service(health)
    })
    .bind(("0.0.0.0",8000))?
    .run()
    .await
}