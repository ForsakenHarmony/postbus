use crate::{
	actors::{CallbackRouter, Database, TelegramSender},
	database::models::Event,
};
use chrono::{NaiveTime, Timelike, Utc};
use std::time::Duration;
use xtra::prelude::*;

pub struct HandleEvent {
	pub event: Event,
}

impl xtra::Message for HandleEvent {
	type Result = ();
}

#[spaad::entangled]
pub struct Scheduler {
	database:        Database,
	telegram_sender: TelegramSender,
	callback_router: CallbackRouter,
}

#[spaad::entangled]
impl Actor for Scheduler {}

#[spaad::entangled]
impl Scheduler {
	#[spaad::spawn(spawner = "tokio")]
	pub fn new(
		database: Database,
		telegram_sender: TelegramSender,
		callback_router: CallbackRouter,
	) -> Self {
		Scheduler {
			database,
			telegram_sender,
			callback_router,
		}
	}

	#[spaad::handler]
	pub async fn fetch_scheduled_events(&mut self) {
		let events: Vec<Event> = self.database.get_all_scheduled_events().await;

		for event in events {
			//                ctx.notify_immediately(ScheduleEvent { event });
		}
	}

	#[spaad::handler]
	pub async fn schedule_event(&mut self, event: Event, ctx: &mut Context<Scheduler>) {
		let time = NaiveTime::from_hms(event.hour as u32, event.minute as u32, 0);
		let dur = duration_until(time);

		let _ = ctx.notify_after(dur, HandleEvent { event });
	}

	#[spaad::handler]
	pub async fn create_event(&mut self) {
		unimplemented!();
	}

	#[spaad::handler]
	pub async fn schedule_tmp_event(&mut self) {
		unimplemented!();
	}

	#[spaad::handler]
	pub async fn remove_tmp_event(&mut self) {
		unimplemented!();
	}

	#[spaad::handler]
	pub async fn remove_event(&mut self) {
		unimplemented!();
	}

	#[spaad::handler]
	pub async fn get_events(&mut self, chat_id: i64) -> Vec<Event> {
		let events = self.database.get_scheduled_events(chat_id).await;

		events
	}

	#[spaad::handler(msg = "HandleEvent")]
	pub async fn handle_event(&mut self, msg: HandleEvent) {
		unimplemented!();
	}
}

fn duration_until(time: NaiveTime) -> Duration {
	let now = Utc::now();

	let secs_from_midnight = time.num_seconds_from_midnight();
	let now_secs_from_midnight = now.time().num_seconds_from_midnight();
	let secs_until_event = if now_secs_from_midnight > secs_from_midnight {
		secs_from_midnight + (86400 - now_secs_from_midnight)
	} else {
		secs_from_midnight - now_secs_from_midnight
	};

	Duration::from_secs(secs_until_event as u64)
}
