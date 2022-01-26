use super::{Exception, Part, Reason, Schedule, Status, StatusChange};
use chrono::{DateTime, TimeZone};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Space<'schedule, Tz: TimeZone> {
	name: String,
	schedules: Vec<Schedule<'schedule, Tz>>,
}

impl<'schedule, Tz: TimeZone> Default for Space<'schedule, Tz> {
	fn default() -> Self {
		Self {
			name: String::new(),
			schedules: Vec::new(),
		}
	}
}

impl<'schedule, Tz: TimeZone> Space<'schedule, Tz> {
	#[must_use]
	pub fn schedule(mut self, schedule: Schedule<'schedule, Tz>) -> Self {
		self.schedules.push(schedule);
		self
	}
}

impl<'schedule, Tz: TimeZone> Space<'schedule, Tz>
where
	DateTime<Tz>: core::convert::From<DateTime<chrono::offset::Local>>,
{
	#[must_use]
	pub fn new(name: &str) -> Space<Tz> {
		Space {
			name: name.to_string(),
			..Default::default()
		}
	}

	/// Compute the status of the space at the current time
	#[must_use]
	pub fn status(&'schedule self) -> Status<'schedule, Tz> {
		use chrono::offset::Local;
		let now: DateTime<Local> = Local::now();
		self.status_at(&DateTime::from(now))
	}

	/// Compute the status of the space at the given time
	// TODO Make actually functional
	pub fn status_at(&'schedule self, time: &DateTime<Tz>) -> Status<'schedule, Tz> {
		let active_schedules: Vec<&Schedule<'schedule, Tz>> = self
			.schedules
			.iter()
			.filter(
				|schedule| match (schedule.effective(), schedule.expires()) {
					(Some(start), None) => start < time,
					(Some(start), Some(end)) => start < time && time < end,
					(None, Some(end)) => time < end,
					(None, None) => true,
				},
			)
			.collect();

		// TODO consider selecting the "most specific" schedule?

		let parts: Vec<&Part<Tz>> = active_schedules
			.iter()
			.flat_map(|schedule| schedule.parts())
			.collect();

		let exceptions: Vec<&Exception<Tz>> = active_schedules
			.iter()
			.flat_map(|schedule| schedule.exceptions())
			.collect();

		let current_parts: Vec<&Part<Tz>> = parts
			.iter()
			.copied()
			.filter(|p| p.applies_at(time))
			.collect();

		let current_exceptions: Vec<&Exception<Tz>> = exceptions
			.iter()
			.copied()
			.filter(|e| e.applies_at(time))
			.collect();

		if !current_exceptions.is_empty() {
			let exception = current_exceptions[0];

			let effect = exception.effect();

			if let Some(effect) = effect {
				return (*effect).clone();
			}
		}

		if !current_parts.is_empty() {
			let part = current_parts[0];
			Status::Open(Reason::Part(Some(part)))
		} else {
			Status::Closed(Reason::Part(None))
		}
	}

	#[must_use]
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
