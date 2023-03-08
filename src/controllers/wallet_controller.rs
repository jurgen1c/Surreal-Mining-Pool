#[path = "../utils.rs"]
mod utils;

use {
  actix_web::{ get, post, Responder },
  actix_web::web::Json,
  crate::models::wallet::*,
  utils::*,
};

// List all Wallets
#[get("/wallets")]
pub async fn get_wallets() -> impl Responder {
  let wallets: Vec<Wallet> = vec![];
  ResponseType::Ok(wallets).get_response()
}

// Get a Wallet
#[get("/wallets/{is}")]
pub async fn get_wallet() -> impl Responder {
  let wallet: Option<Wallet> = None;
  match wallet {
    Some(wallet) => ResponseType::Ok(wallet).get_response(),
    None => ResponseType::NotFound(
      NotFoundMessage::new("No wallet found for id provided".into())
    ).get_response()
  }
}

// Create a Wallet
#[post("/wallets")]
pub async fn create_wallet(wallet_request: Json<NewWalletRequest>) -> impl Responder {
  let wallet: Vec<Wallet> = vec![];
  ResponseType::Created(wallet).get_response()
}