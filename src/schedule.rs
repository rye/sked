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

	pub fn effective(&self) -> &Option<DateTime<Tz>> {
		&self.effective
	}

	pub fn effective_mut(&mut self) -> &mut Option<DateTime<Tz>> {
		&mut self.effective
	}

	pub fn expires(&self) -> &Option<DateTime<Tz>> {
		&self.expires
	}

	pub fn expires_mut(&mut self) -> &mut Option<DateTime<Tz>> {
		&mut self.expires
	}

	pub fn parts(&self) -> &Vec<Part<Tz>> {
		&self.parts
	}

	pub fn parts_mut(&mut self) -> &mut Vec<Part<Tz>> {
		&mut self.parts
	}

	pub fn part(mut self, part: Part<Tz>) -> Self {
		self.parts.push(part);
		self
	}

	pub fn exceptions(&self) -> &Vec<Exception<Tz>> {
		&self.exceptions
	}

	pub fn exceptions_mut(&mut self) -> &mut Vec<Exception<Tz>> {
		&mut self.exceptions
	}

	pub fn exception(mut self, exception: Exception<Tz>) -> Self {
		self.exceptions.push(exception);
		self
	}
}
