use chrono::{DateTime, TimeZone};

pub type Moment<Tz> = DateTime<Tz>;

/// A specifier for when something happens.
pub enum TimeSpecifier {
	/// A pattern of days and times which must be computed against to give a
	/// definitive answer.
	Weekly { day: String, time: String },

	/// A pattern of times
	Daily { time: String },
}

pub enum Status {
	Open,
	Closed { reason: Option<String> },
}

pub struct ScheduleOverride {
	title: String,
	description: String,
	effect: Status,
	beginning: TimeSpecifier,
	end: TimeSpecifier,
}

pub struct SchedulePart {
	open: TimeSpecifier,
	close: TimeSpecifier,
	notes: Vec<String>,
}

pub struct Schedule<Tz: TimeZone> {
	effective: DateTime<Tz>,
	expires: DateTime<Tz>,
	parts: Vec<SchedulePart>,
	overrides: Vec<Tz>,
}

pub struct Space<Tz: TimeZone> {
	name: String,
	schedules: Vec<Schedule<Tz>>,
}

#[cfg(test)]
mod tests {
}
