use crate::actors::TelegramSender;
use std::collections::HashMap;
use telegram_bot_async::CallbackQuery;
use uuid::Uuid;
use xtra::prelude::*;

// pub struct RegisterCallback {
// 	pub id:              Uuid,
// 	pub source:          Actor,
// 	pub normal_messages: bool,
// }
//
// impl RegisterCallback {
// 	pub fn create(id: Uuid, source: Actor, normal_messages: bool) -> Self {
// 		RegisterCallback {
// 			id,
// 			source,
// 			normal_messages,
// 		}
// 	}
// }

// pub struct RouteCallback {
// 	pub query: CallbackQuery,
// }

pub struct Callback {
	pub id: Uuid,
}

impl Message for Callback {
	type Result = ();
}

#[spaad::entangled]
pub struct CallbackRouter {
	telegram_sender: TelegramSender,

	map: HashMap<Uuid, Box<dyn MessageChannel<Callback>>>,
}

#[spaad::entangled]
impl Actor for CallbackRouter {}

#[spaad::entangled]
impl CallbackRouter {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(telegram_sender: TelegramSender) -> Self {
		CallbackRouter {
			telegram_sender,
			map: HashMap::new(),
		}
	}

	#[spaad::handler]
	pub async fn register_callback(
		&mut self,
		id: Uuid,
		source: Box<dyn MessageChannel<Callback>>,
		_normal_messages: bool,
	) {
		self.map.insert(id, source);
	}

	#[spaad::handler]
	pub async fn route_callback(&mut self, query: CallbackQuery) {
		let id = match Uuid::parse_str(&query.data) {
			Ok(id) => id,
			Err(err) => {
				eprintln!(
					"Failed parsing callback Uuid from Telegram {:?}: {:?}",
					query.data, err
				);
				return;
			}
		};

		if let Some(chan) = self.map.get(&id) {
			if chan.send(Callback { id }).await.is_ok() {
				self.telegram_sender.answer_callback(query.clone()).await;
			} else {
				self.map.remove(&id);
			}
		}
	}
}
