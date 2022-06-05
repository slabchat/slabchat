use crate::{
	config::CONFIG, error::Result, services::user_service::create_user,
};
use mongodb::{options::ClientOptions, Client};
use slab_core::model::user::UserInput;
use tracing::{info, warn};

pub mod config;
pub mod error;
pub mod server;
pub mod services;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	// connect to database
	let client_options =
		ClientOptions::parse(&CONFIG.database.uri).await.unwrap();
	let db = Client::with_options(client_options)
		.unwrap()
		.database("slabchat");
	let users = db.collection("users");

	info!("Creating admin user...");
	match create_user(
		users.clone(),
		UserInput {
			email: "admin@example.com".to_string(),
			discriminator: "1234".to_string(),
			password: "toor".to_string(),
			username: "admin".to_string(),
			roles: vec!["admin".to_string(), "member".to_string()],
		},
	)
	.await
	{
		Ok(user) => info!("Created admin user: {}", user.email),
		Err(err) => warn!("{}", err),
	}

	crate::server::start_server(users).await;

	Ok(())
}
