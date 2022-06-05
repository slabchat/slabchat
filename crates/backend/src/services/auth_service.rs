use crate::{
	config::CONFIG,
	error::{Error, Result, WebResult},
};
use chrono::Utc;
use hyper::{
	header::{HeaderValue, AUTHORIZATION},
	HeaderMap,
};
use jsonwebtoken::{
	decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use warp::{header::headers_cloned, reject, Filter, Rejection};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
	email: String,
	roles: Vec<String>,
	exp: usize,
}

pub fn with_auth(
	roles: Vec<String>,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
	headers_cloned()
		.map(move |headers: HeaderMap<HeaderValue>| (roles.clone(), headers))
		.and_then(authorize)
}

#[instrument]
pub fn create_jwt(uid: &str, roles: Vec<String>) -> Result<String> {
	let expiration = Utc::now()
		.checked_add_signed(chrono::Duration::seconds(60))
		.expect("valid timestamp")
		.timestamp();

	let claims = Claims {
		email: uid.to_owned(),
		roles: roles.to_vec(),
		exp: expiration as usize,
	};
	let header = Header::new(Algorithm::HS512);
	encode(
		&header,
		&claims,
		&EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
	)
	.map_err(|_| Error::JWTTokenCreationError)
}

#[instrument]
async fn authorize(
	(roles, headers): (Vec<String>, HeaderMap<HeaderValue>),
) -> WebResult<String> {
	match jwt_from_header(&headers) {
		Ok(jwt) => {
			let decoded = decode::<Claims>(
				&jwt,
				&DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
				&Validation::new(Algorithm::HS512),
			)
			.map_err(|_| reject::custom(Error::JWTTokenError))?;

			if roles.contains(&"admin".to_string())
				&& !decoded.claims.roles.contains(&"admin".to_string())
			{
				return Err(reject::custom(Error::NoPermissionError));
			}

			Ok(decoded.claims.email)
		}
		Err(e) => Err(reject::custom(e)),
	}
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
	let header = match headers.get(AUTHORIZATION) {
		Some(v) => v,
		None => return Err(Error::NoAuthHeaderError),
	};
	let auth_header = match std::str::from_utf8(header.as_bytes()) {
		Ok(v) => v,
		Err(_) => return Err(Error::NoAuthHeaderError),
	};
	if !auth_header.starts_with("Bearer ") {
		return Err(Error::InvalidAuthHeaderError);
	}
	Ok(auth_header.trim_start_matches("Bearer ").to_owned())
}
