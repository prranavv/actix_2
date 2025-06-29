use actix_web::{get, HttpResponse, Result};

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("OK"))
}