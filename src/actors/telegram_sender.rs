use telegram_bot_async::{
	CallbackQuery,
	CanAnswerCallbackQuery,
	DefaultApi,
	InlineKeyboardMarkup,
	MessageChat,
	MessageId,
};
use xtra::prelude::*;

#[spaad::entangled]
pub struct TelegramSender {
	api: DefaultApi,
}

#[spaad::entangled]
impl Actor for TelegramSender {}

#[spaad::entangled]
impl TelegramSender {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(api: DefaultApi) -> Self {
		TelegramSender { api }
	}

	#[spaad::handler]
	pub async fn send_message(
		&mut self,
		chat: MessageChat,
		text: String,
		keyboard: Option<InlineKeyboardMarkup>,
	) -> telegram_bot_async::Message {
		let mut to_send = telegram_bot_async::SendMessage::new(&chat, &text);
		if let Some(kb) = &keyboard {
			to_send = to_send.reply_markup(kb.to_owned());
		}

		let message: telegram_bot_async::Message = self
			.api
			.send(to_send)
			.await
			.expect("should be able to send to telegram");

		message
	}

	#[spaad::handler]
	pub async fn edit_message(
		&mut self,
		chat: MessageChat,
		message: MessageId,
		text: String,
		keyboard: Option<InlineKeyboardMarkup>,
	) -> telegram_bot_async::Message {
		let mut to_send = telegram_bot_async::EditMessageText::new(&chat, message, &text);
		if let Some(kb) = &keyboard {
			to_send = to_send.reply_markup(kb.to_owned());
		}

		let message: telegram_bot_async::Message = self
			.api
			.send(to_send)
			.await
			.expect("should be able to send to telegram");

		message
	}

	#[spaad::handler]
	pub async fn answer_callback(&mut self, callback: CallbackQuery) {
		self.api
			.send(callback.acknowledge())
			.await
			.expect("should be able to send to telegram");
	}
}
