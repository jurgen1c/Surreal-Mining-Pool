#[path = "../utils.rs"]
mod utils;

use {
  actix_web::{ Responder, get, post },
  actix_web::web::Json,
  crate::models::miner::*,
  utils::{ResponseType, NotFoundMessage},
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
pub async fn create_miner(miner_request: Json<NewMinerRequest>) -> impl Responder {
  /*
    TODO: Create a new MinerDAO object from requested inputs and write to DB
   */
  let miner: Vec<Miner> = vec![];
  ResponseType::Created(miner).get_response()
}