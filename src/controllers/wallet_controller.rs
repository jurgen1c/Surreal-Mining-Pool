use {
  actix_web::{ Responder, get, post, put, delete, web::{Json, Path, Data} },
  crate::models::wallet::*,
  crate::utils::http::*,
  crate::repository::{surrealdb_repo::DataBase},
};

// List all Wallets
#[get("/wallets")]
pub async fn get_wallets(db: Data<DataBase>) -> impl Responder {
  let result = WalletDAO::get_all(db).await;

  match result {
    Ok(wallets) => ResponseType::Ok(wallets).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response(),
  }
}

// Get a Wallet
#[get("/wallets/{id}")]
pub async fn get_wallet(db: Data<DataBase>, path: Path<String> ) -> impl Responder {
  let id = path.into_inner();
  if id.is_empty() {
    return ResponseType::NotFound(NotFoundMessage::new("invalid ID".into())).get_response();
  }

  let wallet = WalletDAO::get(db, &id).await;
  match wallet {
    Ok(wallet) => ResponseType::Ok(wallet).get_response(),
    Err(err) => ResponseType::NotFound(
      NotFoundMessage::new(err.to_string())
    ).get_response()
  }
}

// Create a Wallet
#[post("/wallets")]
pub async fn create_wallet(
  db: Data<DataBase>,
  wallet_request: Json<Wallet>
) -> impl Responder {
  let wallet_ress = WalletDAO::create(db, "wallet", wallet_request.into_inner()).await;

  match wallet_ress {
    Ok(wallet) => ResponseType::Created(wallet).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response(),
  }
}

#[put("/wallets/{id}")]
pub async fn update_wallet(
  db: Data<DataBase>,
  path: Path<String>,
  wallet_patch: Json<PatchWallet>,
) -> impl Responder {
  let id = path.into_inner();
  if id.is_empty() {
    return ResponseType::NotFound("invalid ID").get_response();
  }

  let update_result = WalletDAO::update(db, &id, wallet_patch.into_inner()).await;

  match update_result {
    Ok(wallet) => ResponseType::Ok(wallet).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response(),
  }
}

#[delete("/wallets/{id}")]
pub async fn delete_wallet(db: Data<DataBase>, path: Path<String>) -> impl Responder {
  let id = path.into_inner();
  if id.is_empty() {
    return ResponseType::NotFound("invalid ID").get_response();
  }

  let del_ress = WalletDAO::delete(db, &id).await;

  match del_ress {
    Ok(wallet) => ResponseType::Ok(wallet).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response(),
  } 
}
