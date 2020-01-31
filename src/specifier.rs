use chrono::{DateTime, TimeZone};

/// A specifier for when something happens.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Specifier<Tz: TimeZone> {
	/// A pattern of days and times which must be computed against to give a
	/// definitive answer.
	Weekly { day: String, time: String },

	/// A pattern of times
	Daily { time: String },

	/// An exact time
	Exact(DateTime<Tz>),
}
