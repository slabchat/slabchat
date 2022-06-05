use crate::{
	config::CONFIG,
	error::{self, Error, WebResult},
	services::auth_service::{self, with_auth},
};
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
use slab_core::model::user::UserAuth;
use tracing::instrument;
use warp::{reject, reply, Filter, Reply};

#[derive(Serialize, Debug)]
pub struct LoginResponse {
	pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

#[instrument]
pub async fn login_handler(
	users: Collection<UserAuth>,
	body: LoginRequest,
) -> WebResult<impl Reply> {
	match users
		.find_one(
			doc! {
				"email": body.email.clone()
			},
			None,
		)
		.await
		.unwrap()
	{
		Some(user) => {
			argon2::verify_encoded(&user.password, body.password.as_bytes())
				.map_err(|_| Error::WrongCredentialsError)?;

			let token = auth_service::create_jwt(&body.email, user.user.roles)
				.map_err(reject::custom)?;
			Ok(reply::json(&LoginResponse { token }))
		}
		None => Err(reject::custom(Error::WrongCredentialsError)),
	}
}

#[instrument]
pub async fn profile_handler(
	users: Collection<UserAuth>,
	email: String,
) -> WebResult<impl Reply> {
	match users
		.find_one(
			doc! {
				"email": email.clone()
			},
			None,
		)
		.await
		.unwrap()
	{
		Some(user) => Ok(warp::reply::json(&user.user)),
		None => Err(reject::custom(Error::UserNotFound { email })),
	}
}

pub async fn start_server(users: Collection<UserAuth>) {
	let status_route = warp::path("hello").map(|| "Hello world!".to_string());
	let with_users = warp::any().map(move || users.clone());

	let login_route = warp::path!("login")
		.and(warp::post())
		.and(with_users.clone())
		.and(warp::body::json())
		.and_then(login_handler);

	let profile_route = warp::path!("profile")
		.and(warp::get())
		.and(with_users)
		.and(with_auth(vec!["user".to_string(), "member".to_string()]))
		.and_then(profile_handler);

	let routes = login_route
		.or(profile_route)
		.or(status_route)
		.recover(error::handle_rejection);

	warp::serve(routes)
		.run(([0, 0, 0, 0], CONFIG.server.port))
		.await;
}
