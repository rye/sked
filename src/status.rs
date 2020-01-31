#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Status {
	Open,
	Closed { reason: Option<String> },
}
