use chrono::{DateTime, FixedOffset};
use sked::{Exception, Part, Reason, Schedule, Space, Specifier, Status};

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! check_space_at_time {
		($test_name:ident, $time:literal, $expected:expr) => {
			#[test]
			fn $test_name() {
				let space: Space<FixedOffset> = generate_space("asdf");
				let time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339($time).unwrap();
				assert_eq!(space.status_at(&time), $expected);
			}
		};
	}

	fn generate_space(name: &str) -> Space<FixedOffset> {
		let mut exception = Exception::new()
			.effective(Specifier::Weekly {
				day: "Thursday".to_string(),
				time: "10:15".to_string(),
			})
			.expires(Specifier::Weekly {
				day: "Thursday".to_string(),
				time: "11:00".to_string(),
			});
		*exception.effect_mut() = Some(Status::Closed(Reason::Exception(Some(
			"Closed for lunch.".to_string(),
		))));

		let mut schedule: Schedule<FixedOffset> = Schedule::new()
			.part(
				Part::new()
					.open(Specifier::Weekly {
						day: "Thursday".to_string(),
						time: "07:00".to_string(),
					})
					.close(Specifier::Weekly {
						day: "Thursday".to_string(),
						time: "17:00".to_string(),
					}),
			)
			.exception(exception);

		*schedule.effective_mut() =
			Some(DateTime::parse_from_rfc3339("2020-01-01T00:00:00-06:00").unwrap());
		*schedule.expires_mut() =
			Some(DateTime::parse_from_rfc3339("2020-02-01T00:00:00-06:00").unwrap());

		Space::new(name).schedule(schedule)
	}

	mod before_effective {
		use super::*;

		check_space_at_time!(
			is_closed_with_correct_reason,
			"2020-01-16T06:00:00-06:00",
			Status::Closed(Reason::Part(None))
		);
	}

	mod at_effective {
		use super::*;

		check_space_at_time!(
			is_open,
			"2020-01-16T07:00:00-06:00",
			Status::Open(Reason::Part(None))
		);
	}

	mod while_effective {
		use super::*;

		check_space_at_time!(
			is_open,
			"2020-01-16T10:00:00-06:00",
			Status::Open(Reason::Part(None))
		);
	}

	mod before_exception_effective {
		use super::*;

		check_space_at_time!(
			is_open,
			"2020-01-16T10:14:59-06:00",
			Status::Open(Reason::Part(None))
		);
	}

	mod at_exception_effective {
		use super::*;

		check_space_at_time!(
			is_closed_with_correct_reason,
			"2020-01-16T10:15:00-06:00",
			Status::Closed(Reason::Exception(Some("Closed for lunch.".to_string())))
		);
	}

	mod while_exception_effective {
		use super::*;

		check_space_at_time!(
			is_closed_with_correct_reason,
			"2020-01-16T10:35:00-06:00",
			Status::Closed(Reason::Exception(Some("Closed for lunch.".to_string())))
		);
	}

	mod before_exception_expires {
		use super::*;

		check_space_at_time!(
			is_closed_with_correct_reason,
			"2020-01-16T10:59:59-06:00",
			Status::Closed(Reason::Exception(Some("Closed for lunch.".to_string())))
		);
	}

	mod after_exception_expires {
		use super::*;

		check_space_at_time!(
			is_open,
			"2020-01-16T11:00:00-06:00",
			Status::Open(Reason::Part(None))
		);
	}

	mod before_expires {
		use super::*;

		check_space_at_time!(
			is_open,
			"2020-01-16T16:59:59-06:00",
			Status::Open(Reason::Part(None))
		);
	}

	mod at_expires {
		use super::*;

		check_space_at_time!(
			is_closed_no_reason,
			"2020-01-16T17:00:00-06:00",
			Status::Closed(Reason::Part(None))
		);
	}

	mod after_expires {
		use super::*;

		check_space_at_time!(
			is_closed_no_reason,
			"2020-01-16T18:00:00-06:00",
			Status::Closed(Reason::Part(None))
		);
	}
}
