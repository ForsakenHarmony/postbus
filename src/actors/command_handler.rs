use crate::actors::{commands::List, CallbackRouter, Scheduler, TelegramSender};
use futures::executor::block_on;
use std::collections::HashMap;
use telegram_bot_async::MessageChat;
use xtra::prelude::*;

pub struct DestroyCommand {}

impl Message for DestroyCommand {
	type Result = ();
}

#[spaad::entangled]
pub struct CommandHandler {
	telegram_sender: TelegramSender,
	callback_router: CallbackRouter,
	scheduler:       Scheduler,

	map: HashMap<MessageChat, HashMap<Command, Box<dyn MessageChannel<DestroyCommand>>>>,
}

#[spaad::entangled]
impl Actor for CommandHandler {}

#[spaad::entangled]
impl CommandHandler {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(
		telegram_sender: TelegramSender,
		callback_router: CallbackRouter,
		scheduler: Scheduler,
	) -> Self {
		CommandHandler {
			telegram_sender,
			callback_router,
			scheduler,

			map: HashMap::new(),
		}
	}

	#[spaad::handler(spawner = "tokio")]
	pub async fn telegram_command(&mut self, chat: MessageChat, command: String) {
		let chat_commands = self.map.entry(chat.clone()).or_insert_with(HashMap::new);
		let command = Command::parse(&command);

		match command {
			Command::List => {
				let cmd = List::new(
					self.telegram_sender.clone(),
					self.callback_router.clone(),
					self.scheduler.clone(),
				);

				cmd.create_command(chat.clone()).await;

				Some(Box::new(cmd.into_address()))
			}
			Command::Reminder => unimplemented!(),
			Command::_NoCommand_ => None,
		}
		.and_then(|ch| chat_commands.insert(command, ch))
		.map(|ch| block_on(ch.send(DestroyCommand {})).is_ok());
	}
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Command {
	List,
	Reminder,

	_NoCommand_,
}

impl Command {
	fn parse(command: &str) -> Self {
		match () {
			_ if command == "/list" => Command::List,
			_ if command == "/reminder" => Command::Reminder,
			_ => Command::_NoCommand_,
		}
	}
}
