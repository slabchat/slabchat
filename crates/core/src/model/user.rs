use serde::{Deserialize, Serialize};

#[derive(
	Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default,
)]
pub struct User {
	pub email: String,
	pub username: String,
	pub discriminator: String,
	pub roles: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UserInput {
	pub email: String,
	pub username: String,
	pub discriminator: String,
	pub password: String,
	pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuth {
	#[serde(flatten)]
	pub user: User,
	pub password: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UserList {
//     pub users: Vec<User>,
// }

impl UserAuth {
	pub fn new(input: UserInput) -> UserAuth {
		UserAuth {
			user: User {
				email: input.email,
				username: input.username,
				discriminator: input.discriminator,
				roles: input.roles,
			},
			password: input.password,
		}
	}
}

impl User {
	pub fn get_id(&self) -> String {
		format!("{}#{}", self.username, self.discriminator)
	}
}
