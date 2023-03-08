use {
  serde::{ Deserialize, Serialize },
};

// ----------- Miner JSON Payload (Rest) ----------

#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
  pub id: String,
  pub address: String,
  pub club_name: String, 
  pub nickname: String,
  pub hash_rate: i32, // MH/s
  pub shares_mined: i32,
}

// ----- Post Request for new Miner ----------------

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
  nickname: String,
}

// ---------- DAO Object (DB Table Records) ---------

pub struct MinerDAO {
  pub id: String,
  pub address: String,
  pub nickname: String,
  pub hash_rate: i32, // MH/s
  pub shares_mined: i32,
}