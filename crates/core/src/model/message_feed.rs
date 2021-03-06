use super::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Feed {
	messages: Vec<Message>,
}

impl Feed {
	pub fn add_message(&mut self, message: Message) {
		self.messages.push(message);
		self.messages.sort_by_key(|message| message.created_at)
	}

	pub fn messages_iter(&self) -> impl Iterator<Item = &Message> {
		self.messages.iter()
	}
}
