use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};


#[post("/create")]
async fn create()->impl Responder{
    let keypair = Keypair::new();
    let pub_key = keypair.pubkey().to_string();
    let private_key = keypair.to_base58_string();
    return HttpResponse::Ok().json(CreateResponse{
        public_key: pub_key,
        private_key: private_key
    });
}

#[derive(Serialize,Deserialize)]
struct CreateResponse {
    public_key: String,
    private_key: String
}