use chrono::{prelude::*, DateTime, TimeZone};

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

#[derive(Debug)]
pub struct Instances<'iteration, Tz: TimeZone> {
	specifier: &'iteration Specifier<Tz>,
	basis: DateTime<Tz>,
}

impl<'iteration, Tz: TimeZone> Iterator for Instances<'iteration, Tz> {
	type Item = chrono::DateTime<Tz>;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}
