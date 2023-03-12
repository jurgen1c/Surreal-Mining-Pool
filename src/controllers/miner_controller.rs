use {
  actix_web::{ Responder, get, post, put, delete, web::{Json, Path, Data} },
  crate::models::miner::*,
  crate::utils::http::{ResponseType, NotFoundMessage},
  crate::repository::{surrealdb_repo::DataBase},
};

// List all miners
#[get("/miners")]
pub async fn list_miners(db: Data<DataBase>) -> impl Responder {
  let result = MinerDAO::get_all(db).await;

  match result {
    Ok(miners) => ResponseType::Ok(miners).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response(),
  }
}

// Get Miner by id
#[get("/miners/{id}")]
pub async fn get_miner(db: Data<DataBase>, path: Path<String>) -> impl Responder {
  let id = path.into_inner();
  if id.is_empty() {
    return ResponseType::NotFound("invalid ID").get_response();
  }

  let miner_detail = MinerDAO::get(db, &id).await;
  match miner_detail {
    Ok(miner) => ResponseType::Ok(miner).get_response(),
    Err(err) => ResponseType::NotFound(
      NotFoundMessage::new(err.to_string())
    ).get_response(),
  }
}

// Create a new Miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(
  db: Data<DataBase>,
  miner_request: Json<NewMinerRequest>,
  path: Path<String>,
) -> impl Responder {
  let wallet_id = path.into_inner();
  if wallet_id.is_empty() {
    return ResponseType::NotFound("invalid Wallet ID").get_response();
  }
  println!("{}", wallet_id);

  let miner = Miner {
    id: None,
    wallet_id: wallet_id,
    address: miner_request.address.to_owned(),
    club_name: miner_request.club_name.to_owned(),
    nickname: miner_request.nickname.to_owned(),
    hash_rate: miner_request.hash_rate.to_owned(),
    shares_mined: miner_request.shares_mined.to_owned(),
  };

  let miner_detail = MinerDAO::create(db, "miner", miner).await;
  match miner_detail {
    Ok(miner) =>  ResponseType::Created(miner).get_response(),
    Err(err) => ResponseType::InternalError::<String>(err.to_string()).get_response(),
  }
}

// Update a Miner
#[put("/miners/{id}")]
pub async fn update_miner(
  db: Data<DataBase>,
  path: Path<String>,
  miner_patch: Json<MinerPatch>
) -> impl Responder {
  let id = path.into_inner();
  if id.is_empty() {
    return ResponseType::NotFound("invalid ID").get_response();
  }

  let update_result = MinerDAO::update(db, &id, miner_patch.into_inner()).await;

  match update_result {
    Ok(miner) => ResponseType::Ok(miner).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response()
  }
}

// Delete a Miner
#[delete("/miner/{id}")]
pub async fn delete_miner(db: Data<DataBase>, path: Path<String>) -> impl Responder {
  let id = path.into_inner();
  if id.is_empty() {
    return ResponseType::NotFound("invalid ID").get_response();
  }

  let delete_response = MinerDAO::delete(db, &id).await;

  match delete_response {
    Ok(miner) => ResponseType::Ok(miner).get_response(),
    Err(err) => ResponseType::InternalError(err.to_string()).get_response()
  }
}