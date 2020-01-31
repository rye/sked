use chrono::{DateTime, TimeZone};

pub type Moment<Tz> = DateTime<Tz>;

/// A specifier for when something happens.
#[allow(dead_code)]
#[derive(Debug)]
pub enum TimeSpecifier<Tz: TimeZone> {
	/// A pattern of days and times which must be computed against to give a
	/// definitive answer.
	Weekly { day: String, time: String },

	/// A pattern of times
	Daily { time: String },

	///
	Exact(Moment<Tz>),
}

mod exception;
pub use exception::*;

mod part;
pub use part::*;

mod schedule;
pub use schedule::*;

mod status;
pub use status::*;

mod space;
pub use space::*;

#[cfg(test)]
mod tests {}
