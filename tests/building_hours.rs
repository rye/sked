use chrono::{DateTime, FixedOffset};
use sked::{Part, Schedule, Space, Status, TimeSpecifier};

#[test]
fn asdf() {
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
					time: "12:00".to_string(),
				}),
		)
		.part(
			Part::new()
				.open(TimeSpecifier::Weekly {
					day: "Thursday".to_string(),
					time: "13:00".to_string(),
				})
				.close(TimeSpecifier::Weekly {
					day: "Thursday".to_string(),
					time: "17:00".to_string(),
				}),
		);

	let space: Space<FixedOffset> = Space::new("asdf").schedule(schedule);

	assert_eq!(space.status(), Status::Closed { reason: None });
}
