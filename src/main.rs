mod models;
mod controllers;
mod repository;
mod utils;
mod prelude;

use {
	actix_web::{middleware, App, HttpServer },
	actix_web::web::Data,
	std::{ env, io },
	controllers::miner_controller::{get_miner, list_miners, create_miner},
	controllers::wallet_controller::{ get_wallet, get_wallets, create_wallet },
	repository::surrealdb_repo::DataBase,
};

#[tokio::main]
async fn main() -> io::Result<()> {
	env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
	env_logger::init();
	let db =  DataBase::build().await.expect("Error connecting to DB!");

	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(db.clone()))
			.service(get_miner)
			.service(list_miners)
			.service(create_miner)
			.service(get_wallet)
			.service(get_wallets)
			.service(create_wallet)
			.wrap(middleware::Logger::default())
	}).bind("0.0.0.0:9000")?
	.run()
	.await
}