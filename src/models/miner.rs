use {
  serde::{ Deserialize, Serialize },
  actix_web::web::Data,
  std::collections::BTreeMap,
  surrealdb::sql::{Object, Value, thing, Array},
};

use crate::prelude::*;
use crate::utils::{macros::map};
use crate::repository::surrealdb_repo::{Creatable, Patchable, DataBase};

// ----------- Miner JSON Payload (Rest) ----------

#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
  pub id: Option<String>,
  pub address: String,
  pub club_name: String, 
  pub nickname: String,
  pub hash_rate: i32, // MH/s
  pub shares_mined: i32,
}

// --- Implementation to convert miner into SurrealDB Value
impl From<Miner> for Value {
  fn from(val: Miner) -> Self {
    match val.id {
      Some(v) => {
        map![
          "id".into() => v.into(),
          "address".into() => val.address.into(),
          "nickname".into() => val.nickname.into(),
          "club_name".into() => val.club_name.into(),
          "hash_rate".into() => val.hash_rate.into(),
          "shares_mined".into() => val.shares_mined.into(),
        ].into()
      },
      None => {
        map![
          "address".into() => val.address.into(),
          "nickname".into() => val.nickname.into(),
          "club_name".into() => val.club_name.into(),
          "hash_rate".into() => val.hash_rate.into(),
          "shares_mined".into() => val.shares_mined.into(),
        ].into()
      }
    }
  }
}

impl Creatable for Miner{}

// ------ Patch Miner Model --------------

pub struct MinerPatch {
  pub address: Option<String>,
  pub club_name: Option<String>, 
  pub nickname: Option<String>,
  pub hash_rate: Option<i32>, // MH/s
  pub shares_mined: Option<i32>,
}

// --- Implementation to convesrt miner patch  into SurrealDB Value
impl From<MinerPatch> for Value {
  fn from(val: MinerPatch) -> Self {
    let mut value: BTreeMap<String, Value> = BTreeMap::new();

    if let Some(addr) = val.address {
      value.insert("address".into(), addr.into());
    }

    if let Some(c_name) = val.club_name {
      value.insert("club_name".into(), c_name.into());
    }

    if let Some(n_name) = val.nickname {
      value.insert("nickname".into(), n_name.into());
    }

    if let Some(h_rate) = val.hash_rate {
      value.insert("hash_rate".into(), h_rate.into());
    }

    if let Some(s_mined) = val.shares_mined {
      value.insert("shares_mined".into(), s_mined.into());
    }

    Value::from(value)
  }
}

impl Patchable for MinerPatch {}

// ----- Post Request for new Miner ----------------

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
  nickname: String,
}

// ---------- DAO Object (DB Table Records) ---------

pub struct MinerDAO;

impl MinerDAO {
  pub async fn get_all(db: Data<DataBase>) -> Result<Vec<Object>, Error> {
    let sql = "SELECT * FROM miner;";

    let ress = db.datastore.execute(sql, &db.session, None, true).await?;

    let first_res = ress.into_iter().next().expect("Did not get a response from DB");

    let array: Array = W(first_res.result?).try_into()?;

    array.into_iter().map(|value| W(value).try_into()).collect()
  }

  pub async fn create<T: Creatable>(db: Data<DataBase>, tb: &str, data: T) -> Result<Object, Error> {
    let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

    let data: Object = W(data.into()).try_into()?;

    let vars: BTreeMap<String, Value> = map![
      "tb".into() => tb.into(),
      "data".into() => Value::from(data)];

    let ress = db.datastore.execute(sql, &db.session, Some(vars), false).await?;

    let first_val = ress.into_iter().next().map(|r| r.result).expect("id not returned")?;
    
    W(first_val.first()).try_into()
  }

  pub async fn get(db: Data<DataBase>, mid: &str) -> Result<Object, Error> {
    let sql = "SELECT * FROM $th";
        
    let mid = format!("miner:{}", mid);

    let vars: BTreeMap<String, Value> = map!["th".into() => thing(&mid)?.into()];

    let ress = db.datastore.execute(sql, &db.session, Some(vars), true).await?;

    let first_res = ress.into_iter().next().expect("Did not get a response");

    W(first_res.result?.first()).try_into()
  }

  pub async fn update<T: Patchable>(db: Data<DataBase>, mid: &str, data: T) -> Result<Object, Error> {
    let sql = "UPDATE $th MERGE $data RETURN *";

      let mid = format!("miner:{}", mid);

    let vars = map![
      "th".into() => thing(&mid)?.into(),
      "data".into() => data.into()
    ];

      let ress = db.datastore.execute(sql, &db.session, Some(vars), true).await?;

      let first_res = ress.into_iter().next().expect("id not returned");

      let result = first_res.result?;
      
      W(result.first()).try_into()
  }

  pub async fn delete(db: Data<DataBase>, mid: &str) -> Result<String, Error> {
    let sql = "DELETE $th RETURN *";

    let mid = format!("miner:{}", mid);
  
    let vars = map!["th".into() => thing(&mid)?.into()];
  
    let ress = db.datastore.execute(sql, &db.session, Some(vars), false).await?;
  
    let first_res = ress.into_iter().next().expect("id not returned");
  
    first_res.result?;
  
    Ok(mid)
  }
}