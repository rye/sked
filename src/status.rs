use chrono::{DateTime, TimeZone};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reason<'schedule, Tz: TimeZone> {
	Exception(Option<String>),
	Part(Option<&'schedule super::Part<Tz>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status<'schedule, Tz: TimeZone> {
	Open(Reason<'schedule, Tz>),
	Closed(Reason<'schedule, Tz>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatusChange<'schedule, Tz: TimeZone> {
	Opening(DateTime<Tz>, Reason<'schedule, Tz>),
	Closing(DateTime<Tz>, Reason<'schedule, Tz>),
}
