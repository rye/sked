use super::{Status, TimeSpecifier};
use chrono::TimeZone;

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
