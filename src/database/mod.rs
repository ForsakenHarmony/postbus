use diesel::{pg::PgConnection, prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use self::models::*;

pub mod models;
mod schema;

pub struct Database {
	pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
	pub fn connect(database_url: String) -> Self {
		let cm = ConnectionManager::new(&database_url);
		let pool = Pool::new(cm).expect(&format!("Error connecting to {}", database_url));

		Database { pool }
	}

	pub fn get_all_scheduled_events(&mut self) -> Vec<Event> {
		use schema::event::dsl::*;

		let conn = self.pool.get().unwrap();

		let events = event.load::<Event>(&conn).expect("Error loading events");

		events
	}

	pub fn get_scheduled_events(&mut self, chat_id: i64) -> Vec<Event> {
		use schema::event::dsl::*;

		let conn = self.pool.get().unwrap();

		let events = event
			.filter(chat.eq(chat_id))
			.load::<Event>(&conn)
			.expect("Error loading events");

		events
	}

	pub fn add_scheduled_event(
		&mut self,
		message: String,
		chat_id: i64,
		hour: i32,
		minute: i32,
	) -> Event {
		use schema::{chat, event};

		let conn = self.pool.get().unwrap();

		diesel::insert_into(chat::table)
			.values(&chat::dsl::id.eq(chat_id))
			.on_conflict_do_nothing()
			.execute(&conn)
			.expect("Error creating chat");

		let new_event = NewEvent {
			hour,
			minute,
			chat: chat_id,
			message,
		};

		diesel::insert_into(event::table)
			.values(&new_event)
			//            .returning((id, hour, minute, chat, message_list, message))
			.get_result(&conn)
			.expect("Error saving new event")
	}
}
