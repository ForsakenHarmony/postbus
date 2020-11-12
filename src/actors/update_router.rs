use crate::actors::{telegram_sender::TelegramSender, CallbackRouter, CommandHandler};
use telegram_bot_async::{MessageEntityKind, MessageKind, UpdateKind};
use tracing::info;
use xtra::prelude::*;

pub struct Update {
	pub update: telegram_bot_async::Update,
}

#[spaad::entangled]
pub struct UpdateRouter {
	telegram_sender: TelegramSender,
	callback_router: CallbackRouter,
	command_handler: CommandHandler,
}

#[spaad::entangled]
impl Actor for UpdateRouter {}

#[spaad::entangled]
impl UpdateRouter {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(
		telegram_sender: TelegramSender,
		callback_router: CallbackRouter,
		command_handler: CommandHandler,
	) -> Self {
		UpdateRouter {
			telegram_sender,
			callback_router,
			command_handler,
		}
	}

	#[spaad::handler]
	pub async fn update(&mut self, update: telegram_bot_async::Update) {
		info!(id = update.id, "router");
		match &update.kind {
			UpdateKind::Message(msg) => {
				info!("message {:?}", &msg);
				if let MessageKind::Text { data, entities } = &msg.kind {
					let command = entities.iter().find(|e| {
						if let MessageEntityKind::BotCommand = e.kind {
							e.offset == 0
						} else {
							false
						}
					});
					if command.is_some() {
						self.command_handler
							.telegram_command(msg.chat.clone(), data.clone())
							.await;
					}
				}
			}
			UpdateKind::CallbackQuery(cb) => {
				info!("callback {:?}", &cb);
				self.callback_router.route_callback(cb.clone()).await;
			}
			_ => {}
		}
	}
}
