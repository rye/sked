use super::{Reason, Schedule, Status, StatusChange};
use chrono::{DateTime, TimeZone};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Space<Tz: TimeZone> {
	name: String,
	schedules: Vec<Schedule<Tz>>,
}

impl<Tz: TimeZone> Default for Space<Tz> {
	fn default() -> Self {
		Self {
			name: String::new(),
			schedules: Vec::new(),
		}
	}
}

impl<Tz: TimeZone> Space<Tz> {
	pub fn schedule(mut self, schedule: Schedule<Tz>) -> Self {
		self.schedules.push(schedule);
		self
	}
}

impl<Tz: TimeZone> Space<Tz>
where
	DateTime<Tz>: core::convert::From<DateTime<chrono::offset::Local>>,
{
	pub fn new(name: &str) -> Space<Tz> {
		Space {
			name: name.to_string(),
			..Default::default()
		}
	}

	/// Compute the status of the space at the current time
	pub fn status(&self) -> Status {
		use chrono::offset::Local;
		let now: DateTime<Local> = Local::now();
		self.status_at(&DateTime::from(now))
	}

	/// Compute the status of the space at the given time
	// TODO Make actually functional
	pub fn status_at(&self, _time: &DateTime<Tz>) -> Status {
		Status::Closed(Reason::Part(None))
	}

	pub fn next_status_change(&self) -> Option<StatusChange<Tz>> {
		use chrono::offset::Local;
		let now: DateTime<Local> = Local::now();
		self.next_status_change_at(&DateTime::from(now))
	}

	// TODO Make actually functional
	pub fn next_status_change_at(&self, _time: &DateTime<Tz>) -> Option<StatusChange<Tz>> {
		None
	}
}