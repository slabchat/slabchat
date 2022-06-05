use crate::error::{Error, Result};
use argon2::{self, Config};
use mongodb::{bson::doc, Collection};
use nanoid::nanoid;
use slab_core::model::user::{User, UserAuth, UserInput};
use tracing::instrument;

// TODO: authentication (with tokens?)

#[instrument]
pub async fn create_user(
	db: Collection<UserAuth>,
	input: UserInput,
) -> Result<User> {
	let mut user = UserAuth::new(input.clone());

	if let Some(_existing_user) = db
		.find_one(
			doc! {
				"email": input.email,
			},
			None,
		)
		.await
		.unwrap()
	{
		return Err(Error::UserAlreadyExists {
			email: user.user.email,
		});
	}

	// hash the password
	let config = Config::default();

	let hash = argon2::hash_encoded(
		input.password.as_bytes(),
		nanoid!(50).as_bytes(),
		&config,
	)
	.unwrap();

	// set the password to the hash
	user.password = hash;

	// save user to database
	db.insert_one(user.clone(), None).await.unwrap();

	Ok(user.user)
}
