use crate::database::models::Event;
use xtra::prelude::*;

#[spaad::entangled]
pub struct Database {
	db: crate::database::Database,
}

#[spaad::entangled]
impl Actor for Database {}

#[spaad::entangled]
impl Database {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(db: crate::database::Database) -> Self {
		Database { db }
	}

	#[spaad::handler]
	pub async fn get_scheduled_events(&mut self, chat_id: i64) -> Vec<Event> {
		self.db.get_scheduled_events(chat_id)
	}

	#[spaad::handler]
	pub async fn get_all_scheduled_events(&mut self) -> Vec<Event> {
		self.db.get_all_scheduled_events()
	}

	#[spaad::handler]
	pub async fn add_scheduled_event(
		&mut self,
		message: String,
		chat_id: i64,
		hour: i32,
		minute: i32,
	) -> Event {
		self.db.add_scheduled_event(message, chat_id, hour, minute)
	}
}
