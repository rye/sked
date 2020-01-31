use chrono::{DateTime, TimeZone};

pub type Moment<Tz> = DateTime<Tz>;

/// A specifier for when something happens.
#[allow(dead_code)]
pub enum TimeSpecifier<Tz: TimeZone> {
	/// A pattern of days and times which must be computed against to give a
	/// definitive answer.
	Weekly {
		basis: Option<Moment<Tz>>,
		day: String,
		time: String,
	},

	/// A pattern of times
	Daily {
		basis: Option<Moment<Tz>>,
		time: String,
	},
}

#[allow(dead_code)]
pub enum Status {
	Open,
	Closed { reason: Option<String> },
}

#[allow(dead_code)]
pub struct ScheduleOverride<Tz: TimeZone> {
	title: String,
	description: String,
	effect: Status,
	beginning: TimeSpecifier<Tz>,
	end: TimeSpecifier<Tz>,
}

#[allow(dead_code)]
pub struct SchedulePart<Tz: TimeZone> {
	open: TimeSpecifier<Tz>,
	close: TimeSpecifier<Tz>,
	notes: Vec<String>,
}

#[allow(dead_code)]
pub struct Schedule<Tz: TimeZone> {
	effective: DateTime<Tz>,
	expires: DateTime<Tz>,
	parts: Vec<SchedulePart<Tz>>,
	overrides: Vec<Tz>,
}

#[allow(dead_code)]
pub struct Space<Tz: TimeZone> {
	name: String,
	schedules: Vec<Schedule<Tz>>,
}

#[cfg(test)]
mod tests {}
