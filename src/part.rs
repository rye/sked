use super::TimeSpecifier;
use chrono::TimeZone;

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
