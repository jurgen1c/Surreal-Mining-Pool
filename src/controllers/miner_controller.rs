use {
  actix_web::{ Responder, get, post, put, delete, web::{Json, Path, Data} },
  crate::models::miner::*,
  crate::utils::http::{ResponseType, NotFoundMessage},
  crate::repository::{surrealdb_repo::DataBase},
};

// List all miners
#[get("/miners")]
pub async fn list_miners() -> impl Responder {
  /*
    TODO: Get all MinerDAO objects from DB and convert to Miner onjects 
   */
  let miners: Vec<Miner> = vec![];
  print!("In miners controller!!!");
  ResponseType::Ok(miners).get_response()
}

// Get Miner by id
#[get("/miners/{id}")]
pub async fn get_miner() -> impl Responder {
  /*
    TODO: Get the Miner DAO object from DB WHERE id = request id and convert to miner object
   */
  let miner: Option<Miner> = None;
  match miner {
    Some(miner) => ResponseType::Ok(miner).get_response(),
    None => ResponseType::NotFound(
      NotFoundMessage::new("No miner found for provided id".into())
    ).get_response(),
  }
}

// Create a new Miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(db: Data<DataBase>, miner_request: Json<Miner>) -> impl Responder {
  let miner = Miner {
    id: None,
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