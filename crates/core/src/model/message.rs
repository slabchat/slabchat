use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// A chat message
pub struct Message {
	pub id: String,
	/// The user id of the author of the message
	pub user: String,
	/// Markdown content
	pub body: String,
	pub created_at: DateTime<Utc>,
}

impl Message {
	pub fn new(user: String, body: String) -> Message {
		Message {
			id: nanoid!(36),
			user,
			body,
			created_at: Utc::now(),
		}
	}
}
