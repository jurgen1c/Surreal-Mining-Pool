use std::sync::Arc;
use surrealdb::sql::Value;
use surrealdb::{Datastore, Session, Error};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct DataBase {
	pub datastore: Arc<Datastore>,
	pub session: Session,
}

impl DataBase {
  pub async fn build() -> Result<Self, Error> {
    Ok(
      Self {
        datastore: Arc::new(
          Datastore::new("file://mining_pool.db").await.expect("Unable to create Datastore")
        ),
        session: Session::for_kv().with_ns("my_ns").with_db("mining_pool")
      }
    )
  }
}