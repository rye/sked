use chrono::{DateTime, TimeZone};

#[derive(Debug, PartialEq)]
pub enum Reason {
	Exception(Option<String>),
	Part(Option<String>),
}

#[derive(Debug, PartialEq)]
pub enum Status {
	Open(Reason),
	Closed(Reason),
}

#[derive(Debug, PartialEq)]
pub enum StatusChange<Tz: TimeZone> {
	Opening(DateTime<Tz>, Reason),
	Closing(DateTime<Tz>, Reason),
}
