use chrono::{DateTime, TimeZone};

pub type Moment<Tz> = DateTime<Tz>;

/// A specifier for when something happens.
#[allow(dead_code)]
#[derive(Debug)]
pub enum TimeSpecifier<Tz: TimeZone> {
	/// A pattern of days and times which must be computed against to give a
	/// definitive answer.
	Weekly { day: String, time: String },

	/// A pattern of times
	Daily { time: String },

	///
	Exact(Moment<Tz>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Status {
	Open,
	Closed { reason: Option<String> },
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Override<Tz: TimeZone> {
	effect: Status,
	beginning: TimeSpecifier<Tz>,
	end: TimeSpecifier<Tz>,
	message: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Part<Tz: TimeZone> {
	open: Option<TimeSpecifier<Tz>>,
	close: Option<TimeSpecifier<Tz>>,
	notes: Vec<String>,
}

impl<Tz: TimeZone> Part<Tz> {
	pub fn new() -> Self {
		Self {
			open: None,
			close: None,
			notes: Vec::new(),
		}
	}

	pub fn open(mut self, ts: TimeSpecifier<Tz>) -> Self {
		self.open = Some(ts);
		self
	}

	pub fn close(mut self, ts: TimeSpecifier<Tz>) -> Self {
		self.close = Some(ts);
		self
	}

	pub fn note(mut self, note: &str) -> Self {
		self.notes.push(note.to_string());
		self
	}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Schedule<Tz: TimeZone> {
	effective: Option<DateTime<Tz>>,
	expires: Option<DateTime<Tz>>,
	parts: Vec<Part<Tz>>,
	overrides: Vec<Override<Tz>>,
}

impl<Tz: TimeZone> Default for Schedule<Tz> {
	fn default() -> Self {
		Self {
			effective: None,
			expires: None,
			parts: Vec::new(),
			overrides: Vec::new(),
		}
	}
}

impl<Tz: TimeZone> Schedule<Tz> {
	pub fn new() -> Schedule<Tz> {
		Default::default()
	}

	pub fn effective(mut self, date_time: DateTime<Tz>) -> Self {
		self.effective = Some(date_time);
		self
	}

	pub fn expires(mut self, date_time: DateTime<Tz>) -> Self {
		self.expires = Some(date_time);
		self
	}

	pub fn part(mut self, part: Part<Tz>) -> Self {
		self.parts.push(part);
		self
	}
}

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
	// TODO Make actually functional
	pub fn status(&self) -> Status {
		use chrono::offset::Local;
		let now: DateTime<Local> = Local::now();
		self.status_at(&DateTime::from(now))
	}

	/// Compute the status of the space at the given time
	// TODO Make actually functional
	pub fn status_at(&self, _time: &DateTime<Tz>) -> Status {
		Status::Closed { reason: None }
	}
}

#[cfg(test)]
mod tests {}
