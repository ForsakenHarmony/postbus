#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate diesel;

use crate::actors::{
	CallbackRouter,
	CommandHandler,
	Database,
	Scheduler,
	TelegramSender,
	UpdateRouter,
};
use dotenv::dotenv;
use futures::{FutureExt, StreamExt};
use std::env;
use telegram_bot_async::DefaultApi;

mod actors;
mod database;

// pub fn create_actor<S, F, A>(handle: &SysHandle, state: S, actor_fn: A) -> Actor
// where
// 	S: Send + Sync + 'static,
// 	F: Future<Output = Result<S, ActorError>> + Send + 'static,
// 	A: ActorFn<S, F> + 'static,
// {
// 	let build = BuildActorOp::new(None, state, actor_fn);
// 	block_on(handle.new_actor(build)).unwrap()
// }

#[tokio::main]
async fn main() {
	dotenv().ok();

	let telegram_token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set");
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let sub = tracing_subscriber::FmtSubscriber::builder()
		.with_max_level(tracing::Level::INFO)
		.finish();

	let _ = tracing::subscriber::set_global_default(sub);

	let rt = tokio::runtime::Handle::current();
	//
	// let cfg = ActorSystemConfig::default();
	// let sys = ActorSystemBuilder::new(cfg, rt).start().await.unwrap();
	//
	// let handle = sys.get_handle();

	let api = DefaultApi::new_default(telegram_token).unwrap();

	let db = database::Database::connect(database_url);

	let database = Database::new(db);

	let telegram_sender = TelegramSender::new(api.clone());
	let callback_router = CallbackRouter::new(telegram_sender.clone());
	let scheduler = Scheduler::new(
		database.clone(),
		telegram_sender.clone(),
		callback_router.clone(),
	);
	let command_handler = CommandHandler::new(
		telegram_sender.clone(),
		callback_router.clone(),
		scheduler.clone(),
	);
	let mut update_router = UpdateRouter::new(
		telegram_sender.clone(),
		callback_router.clone(),
		command_handler.clone(),
	);

	// dbg!(block_on(database.send(GetAllScheduledEvents {})).unwrap());

	api.into_stream()
		.updates()
		.for_each(|mb_update| update_router.update(mb_update.unwrap()).map(|_| ()))
		.await;

	// api.into_stream().updates().map(|update|  Ok(Update { update: update.unwrap() })).forward(upd_addr.send).map(|_| ()).await;

	// sys.await_shutdown(Duration::from_secs(1));
}
