use super::{Specifier, Status};
use chrono::{DateTime, TimeZone};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Exception<'schedule, Tz: TimeZone> {
	effect: Option<Status<'schedule, Tz>>,
	effective: Option<Specifier<Tz>>,
	expires: Option<Specifier<Tz>>,
}

impl<'schedule, Tz: TimeZone> Default for Exception<'schedule, Tz> {
	fn default() -> Self {
		Self {
			effect: None,
			effective: None,
			expires: None,
		}
	}
}

impl<'schedule, Tz: TimeZone> Exception<'schedule, Tz> {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	pub fn effect_mut(&mut self) -> &mut Option<Status<'schedule, Tz>> {
		&mut self.effect
	}

	pub fn effect(&self) -> &Option<Status<'schedule, Tz>> {
		&self.effect
	}

	#[must_use]
	pub fn effective(mut self, effective: Specifier<Tz>) -> Self {
		self.effective = Some(effective);
		self
	}

	#[must_use]
	pub fn expires(mut self, expires: Specifier<Tz>) -> Self {
		self.expires = Some(expires);
		self
	}

	pub fn applies_at(&self, time: &DateTime<Tz>) -> bool {
		match (self.effective.as_ref(), self.expires.as_ref()) {
			(Some(open), Some(close)) => {
				let open = open.instances(time).next().unwrap();
				let close = close.instances(time).next().unwrap();

				(open..close).contains(time)
			}
			(None, Some(_)) => true,
			(Some(_), None) => true,
			(None, None) => true,
		}
	}
}
