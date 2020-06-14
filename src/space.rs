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
	pub fn schedule(mut self, schedule: Schedule<'schedule, Tz>) -> Self {
		self.schedules.push(schedule);
		self
	}
}

impl<'schedule, Tz: TimeZone> Space<'schedule, Tz>
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
			.map(|schedule| schedule.parts())
			.flatten()
			.collect();

		let exceptions: Vec<&Exception<Tz>> = active_schedules
			.iter()
			.map(|schedule| schedule.exceptions())
			.flatten()
			.collect();

		let current_parts: Vec<&&Part<Tz>> = parts.iter().filter(|p| p.applies_at(time)).collect();

		let current_exceptions: Vec<&&Exception<Tz>> =
			exceptions.iter().filter(|e| e.applies_at(time)).collect();

		eprintln!("{}, x{}", current_parts.len(), current_exceptions.len());

		if current_exceptions.len() > 0 {
			let exception = current_exceptions[0];

			let effect = exception.effect();

			if let Some(effect) = effect {
				return (*effect).clone();
			}
		}

		if current_parts.len() > 0 {
			Status::Open(Reason::Part(None))
		} else {
			Status::Closed(Reason::Part(None))
		}
	}

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
