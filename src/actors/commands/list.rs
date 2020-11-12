use crate::{
	actors::{
		callback_router::Callback,
		CallbackRouter,
		DestroyCommand,
		Scheduler,
		TelegramSender,
	},
	database::models::Event,
};
use telegram_bot_async::{InlineKeyboardButton, MessageChat};
use uuid::Uuid;
use xtra::prelude::*;

#[spaad::entangled]
pub struct List {
	message: Option<telegram_bot_async::Message>,

	telegram_sender: TelegramSender,
	callback_router: CallbackRouter,
	scheduler:       Scheduler,
	events:          Vec<Event>,
	button_id:       Uuid,
}

#[spaad::entangled]
impl Actor for List {}

#[spaad::entangled]
impl List {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(
		telegram_sender: TelegramSender,
		callback_router: CallbackRouter,
		scheduler: Scheduler,
	) -> Self {
		List {
			message: None,

			telegram_sender,
			callback_router,
			scheduler,

			events: Vec::new(),
			button_id: Uuid::new_v4(),
		}
	}

	#[spaad::handler]
	pub async fn create_command(&mut self, chat: MessageChat, ctx: &mut xtra::Context<Self>) {
		let addr = ctx.address().unwrap();

		let events = self.scheduler.get_events(chat.id().0).await;
		self.events = events.clone();

		self.callback_router
			.register_callback(self.button_id, Box::new(addr), false)
			.await;

		let msg = self
			.telegram_sender
			.send_message(
				chat.clone(),
				"Menu message".to_string(),
				Some(
					vec![vec![InlineKeyboardButton::callback(
						"test",
						&self.button_id.to_string(),
					)]]
					.into(),
				),
			)
			.await;

		self.message = Some(msg);
	}

	#[spaad::handler(msg = "Callback")]
	pub async fn callback(&mut self, callback: Callback) {
		if callback.id == self.button_id {
			if let Some(msg) = &self.message {
				let msg = self
					.telegram_sender
					.edit_message(
						msg.chat.clone(),
						msg.id,
						"Menu message".to_string(),
						Some(
							vec![vec![InlineKeyboardButton::callback(
								"ok",
								&self.button_id.to_string(),
							)]]
							.into(),
						),
					)
					.await;

				self.message = Some(msg);

				// let msg = self
				// 	.telegram_sender
				// 	.ask(
				// 		EditMessage {
				// 			chat:     msg.chat.clone(),
				// 			message:  msg.id,
				// 			text:     "Menu message".to_string(),
				// 			keyboard: Some(
				// 				vec![vec![InlineKeyboardButton::callback(
				// 					"ok",
				// 					&self.button_id.to_string(),
				// 				)]]
				// 				.into(),
				// 			),
				// 		},
				// 		Response::Wait,
				// 	)
				// 	.await
				// 	.expect("could not edit")
				// 	.unwrap();
			}
			println!("Pressed button");
		}
	}

	#[spaad::handler(msg = "DestroyCommand")]
	pub async fn destroy_command(&mut self, _cmd: DestroyCommand, ctx: &mut xtra::Context<Self>) {
		println!("Destroy {}", self.button_id);
		if let Some(msg) = &self.message {
			self.telegram_sender
				.edit_message(msg.chat.clone(), msg.id, "Menu message".to_string(), None)
				.await;
		}
		ctx.stop();
	}
}
