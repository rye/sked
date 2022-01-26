use super::{Exception, Part};
use chrono::{DateTime, TimeZone};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Schedule<'schedule, Tz: TimeZone> {
	effective: Option<DateTime<Tz>>,
	expires: Option<DateTime<Tz>>,
	parts: Vec<Part<Tz>>,
	exceptions: Vec<Exception<'schedule, Tz>>,
}

impl<'schedule, Tz: TimeZone> Default for Schedule<'schedule, Tz> {
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
impl<'schedule, Tz: TimeZone> Schedule<'schedule, Tz> {
	pub fn new() -> Schedule<'schedule, Tz> {
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

	pub fn exceptions_mut(&mut self) -> &mut Vec<Exception<'schedule, Tz>> {
		&mut self.exceptions
	}

	pub fn exception(mut self, exception: Exception<'schedule, Tz>) -> Self {
		self.exceptions.push(exception);
		self
	}
}
