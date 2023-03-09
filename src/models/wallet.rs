use {
  serde::{ Deserialize, Serialize },
  actix_web::web::Data,
  std::collections::BTreeMap,
  surrealdb::sql::{Object, Value, thing, Array},
  //crate::models::miner::*,
  crate::prelude::*,
  crate::utils::{macros::map},
  crate::repository::surrealdb_repo::{Creatable, Patchable, DataBase},
};



// ------------- Wallet JSON Payload (REST) --------

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
  pub id: Option<String>,
  pub address: String,
  pub club_name: String,
  pub total_hash_rate: i32,
  pub total_shares_mined: i32,
  pub total_workers_online: i32,
  //pub workers_online: Vec<Miner>,
}

impl Wallet {
  pub fn new(
    id: Option<String>,
    address: String,
    club_name: String,
    total_hash_rate: i32,
    total_shares_mined: i32,
    total_workers_online: i32,
  ) -> Self {
    Self {
      id,
      address,
      club_name,
      total_hash_rate,
      total_shares_mined,
      total_workers_online,
    }
  }
}

// --- Implementation to convert wallet into SurrealDB Value
impl From<Wallet> for Value {
  fn from(val: Wallet) -> Self {
    match val.id {
      Some(v) => {
        map![
          "id".into() => v.into(),
          "address".into() => val.address.into(),
          "club_name".into() => val.club_name.into(),
          "total_hash_rate".into() => val.total_hash_rate.into(),
          "total_shares_mined".into() => val.total_shares_mined.into(),
          "total_workers_online".into() => val.total_workers_online.into(),
        ].into()
      },
      None => {
        map![
          "address".into() => val.address.into(),
          "club_name".into() => val.club_name.into(),
          "total_hash_rate".into() => val.total_hash_rate.into(),
          "total_shares_mined".into() => val.total_shares_mined.into(),
          "total_workers_online".into() => val.total_workers_online.into(),
        ].into()
      }
    }
  }
}

impl Creatable for Wallet{}

// ------- Patch Wallet model
#[derive(Debug, Deserialize, Serialize)]
pub struct PatchWallet {
  pub address: Option<String>,
  pub club_name: Option<String>,
  pub total_hash_rate: Option<i32>,
  pub total_shares_mined: Option<i32>,
  pub total_workers_online: Option<i32>,
}

// ------- Implement from PatchWallet to surreal db value
impl From<PatchWallet> for Value {
  fn from(val: PatchWallet) -> Self {
    let mut value: BTreeMap<String, Value> = BTreeMap::new();

    if let Some(addr) = val.address {
      value.insert("address".into(), addr.into());
    }

    if let Some(c_name) = val.club_name {
      value.insert("club_name".into(), c_name.into());
    }

    if let Some(th_rate) = val.total_hash_rate {
      value.insert("total_hash_rate".into(), th_rate.into());
    }

    if let Some(ts_mined) = val.total_shares_mined {
      value.insert("total_shares_mined".into(), ts_mined.into());
    }

    if let Some(tw_online) = val.total_workers_online {
      value.insert("total_workers_online".into(), tw_online.into());
    }

    Value::from(value)
  }
}

impl Patchable for PatchWallet {}

// ----------- Post Request Body for new Wallet

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
  club_name: String,
}

// ---------- Wallet DAO Object -----------

pub struct WalletDAO;

impl WalletDAO {
  pub async fn get_all(db: Data<DataBase>) -> Result<Vec<Object>, Error> {
    let sql = "SELECT * FROM wallet;";

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
        
    let mid = format!("wallet:{}", mid);

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