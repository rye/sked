use super::{Exception, Part};
use chrono::{DateTime, TimeZone};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Schedule<Tz: TimeZone> {
	effective: Option<DateTime<Tz>>,
	expires: Option<DateTime<Tz>>,
	parts: Vec<Part<Tz>>,
	exceptions: Vec<Exception<Tz>>,
}

impl<Tz: TimeZone> Default for Schedule<Tz> {
	fn default() -> Self {
		Self {
			effective: None,
			expires: None,
			parts: Vec::new(),
			exceptions: Vec::new(),
		}
	}
}

#[allow(dead_code)]
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

	pub fn exception(mut self, exception: Exception<Tz>) -> Self {
		self.exceptions.push(exception);
		self
	}
}
