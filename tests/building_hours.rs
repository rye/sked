use chrono::{DateTime, FixedOffset};
use sked::{Exception, Part, Schedule, Space, Status, TimeSpecifier};

#[cfg(test)]
mod tests {
	use super::*;

	fn generate_space(name: &str) -> Space<FixedOffset> {
		let schedule: Schedule<FixedOffset> = Schedule::new()
			.effective(DateTime::parse_from_rfc3339("2020-01-01T00:00:00-06:00").unwrap())
			.expires(DateTime::parse_from_rfc3339("2020-02-01T00:00:00-06:00").unwrap())
			.part(
				Part::new()
					.open(TimeSpecifier::Weekly {
						day: "Thursday".to_string(),
						time: "07:00".to_string(),
					})
					.close(TimeSpecifier::Weekly {
						day: "Thursday".to_string(),
						time: "17:00".to_string(),
					}),
			)
			.exception(
				Exception::new()
					.effective(TimeSpecifier::Weekly {
						day: "Thursday".to_string(),
						time: "10:15".to_string(),
					})
					.expires(TimeSpecifier::Weekly {
						day: "Thursday".to_string(),
						time: "11:00".to_string(),
					})
					.effect(Status::Closed { reason: Some("Closed for lunch.".to_string()) })
			);

		Space::new(name).schedule(schedule)
	}

	mod before_effective {
		use super::*;

		#[test]
		fn space_is_marked_as_closed_with_correct_reason() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T06:00:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Closed { reason: None });
		}
	}

	mod at_effective {
		use super::*;

		#[test]
		fn space_is_marked_as_open() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T07:00:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Open);
		}
	}

	mod while_effective {
		use super::*;

		#[test]
		fn space_is_marked_as_open() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T10:00:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Open);
		}
	}

	mod before_exception_effective {
		use super::*;

		#[test]
		fn space_is_marked_as_open() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T10:14:59-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Open);
		}
	}

	mod at_exception_effective {
		use super::*;

		#[test]
		fn space_is_marked_as_closed_with_correct_reason() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T10:15:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Closed { reason: Some("Closed for lunch.".to_string()) });
		}
	}

	mod while_exception_effective {
		use super::*;

		#[test]
		fn space_is_marked_as_closed_with_correct_reason() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T10:35:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Closed { reason: Some("Closed for lunch.".to_string()) });
		}
	}

	mod before_exception_expires {
		use super::*;

		#[test]
		fn space_is_marked_as_closed_with_correct_reason() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T10:59:59-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Closed { reason: Some("Closed for lunch.".to_string()) });
		}
	}

	mod after_exception_expires {
		use super::*;

		#[test]
		fn space_is_marked_as_open() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T11:00:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Open);
		}
	}

	mod before_expires {
		use super::*;

		#[test]
		fn space_is_marked_as_open() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T16:59:59-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Open);
		}
	}

	mod at_expires {
		use super::*;


		#[test]
		fn space_is_marked_as_closed() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T17:00:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Closed { reason: None });
		}
	}

	mod after_expires {
		use super::*;

		#[test]
		fn space_is_marked_as_closed() {
			let space: Space<FixedOffset> = generate_space("asdf");
			let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2020-01-16T18:00:00-06:00").unwrap();
			assert_eq!(space.status_at(&time), Status::Closed { reason: None });
		}
	}
}
