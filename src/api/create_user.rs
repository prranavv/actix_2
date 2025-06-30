use actix_web::{post, web::{self}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::UserDB;


#[post("/create")]
async fn create(db: web::Data<UserDB>)->impl Responder{
    let mut db=db.lock().unwrap();
    let keypair = Keypair::new();
    let pub_key = keypair.pubkey().to_string();
    let private_key = keypair.to_base58_string();
    db.insert(pub_key.clone(), private_key.clone());
    return HttpResponse::Ok().json(CreateResponse{
        public_key: pub_key,
        private_key: private_key
    });
}
#[post("/get/{key}")]
async fn get_private_key(key:web::Path<String>,db: web::Data<UserDB>)->impl Responder{
    let db = db.lock().unwrap();
    let key = key.into_inner();
    match db.get(&key){
        Some(key)=>return HttpResponse::Ok().json(key),
        None=>return HttpResponse::Ok().json("value")
    }
}

#[derive(Serialize,Deserialize)]
struct CreateResponse {
    public_key: String,
    private_key: String
}