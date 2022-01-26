use super::Specifier;
use chrono::{DateTime, TimeZone};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct Part<Tz: TimeZone> {
	open: Option<Specifier<Tz>>,
	close: Option<Specifier<Tz>>,
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
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn open(mut self, ts: Specifier<Tz>) -> Self {
		self.open = Some(ts);
		self
	}

	#[must_use]
	pub fn close(mut self, ts: Specifier<Tz>) -> Self {
		self.close = Some(ts);
		self
	}

	#[must_use]
	pub fn note(mut self, note: &str) -> Self {
		self.notes.push(note.to_string());
		self
	}

	pub fn applies_at(&self, time: &DateTime<Tz>) -> bool {
		match (self.open.as_ref(), self.close.as_ref()) {
			(Some(open), Some(close)) => {
				let open = open.instances(time).next().unwrap();
				let close = close.instances(time).next().unwrap();

				(open..close).contains(time)
			}
			(_, _) => true,
		}
	}
}
