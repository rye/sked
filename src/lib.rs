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
pub struct Exception<Tz: TimeZone> {
	effect: Status,
	effective: Option<TimeSpecifier<Tz>>,
	expires: Option<TimeSpecifier<Tz>>,
}

impl<Tz: TimeZone> Default for Exception<Tz> {
	fn default() -> Self {
		Self {
			effect: Status::Closed { reason: None },
			effective: None,
			expires: None,
		}
	}
}

impl<Tz: TimeZone> Exception<Tz> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn effect(mut self, effect: Status) -> Self {
		self.effect = effect;
		self
	}

	pub fn effective(mut self, effective: TimeSpecifier<Tz>) -> Self {
		self.effective = Some(effective);
		self
	}

	pub fn expires(mut self, expires: TimeSpecifier<Tz>) -> Self {
		self.expires = Some(expires);
		self
	}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Part<Tz: TimeZone> {
	open: Option<TimeSpecifier<Tz>>,
	close: Option<TimeSpecifier<Tz>>,
	notes: Vec<String>,
}

impl<Tz: TimeZone> Default for Part<Tz> {
	fn default() -> Self {
		Self {
			open: None,
			close: None,
			notes: Vec::new(),
		}
	}
}

impl<Tz: TimeZone> Part<Tz> {
	pub fn new() -> Self {
		Self::default()
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

mod schedule;
pub use schedule::*;

mod space;
pub use space::*;

#[cfg(test)]
mod tests {}
