use chrono::{prelude::*, DateTime, TimeZone};

/// A specifier for when something happens.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
		match self.specifier {
			Specifier::Exact(dt) if dt != &self.basis => {
				let next = dt.to_owned();
				self.basis = dt.to_owned();
				Some(next)
			}
			Specifier::Exact(dt) if dt == &self.basis => None,
			Specifier::Exact(_) => panic!(),
			Specifier::Weekly { day, time } => {
				let specifier_day: chrono::Weekday = day.parse().expect("invalid day specifier");
				let specifier_time: chrono::NaiveTime = NaiveTime::parse_from_str(time, "%H:%M")
					.or_else(|_| NaiveTime::parse_from_str(time, "%H:%M:%S"))
					.expect("invalid time specifier");

				// If the basis weekday is the same as the specifier, then return today's instance
				if self.basis.weekday() == specifier_day {
					let instance = self.basis.date().and_time(specifier_time).unwrap();

					self.basis = self.basis.clone() + chrono::Duration::weeks(1);

					Some(instance)
				} else {
					let basis_weekday: i64 = self.basis.date().weekday().num_days_from_monday().into();
					let next_instance_weekday: i64 = specifier_day.num_days_from_monday().into();
					let mut difference = next_instance_weekday - basis_weekday;

					// We must add days to get to the next updates.
					if difference < 0 {
						difference += 7;
					}

					let offset = chrono::Duration::days(difference);

					let instance = (self.basis.date() + offset)
						.and_time(specifier_time)
						.unwrap();

					self.basis = self.basis.to_owned() + chrono::Duration::weeks(1);

					Some(instance)
				}
			}
			Specifier::Daily { time } => {
				let specifier_time: chrono::NaiveTime = NaiveTime::parse_from_str(time, "%H:%M")
					.or_else(|_| NaiveTime::parse_from_str(time, "%H:%M:%S"))
					.expect("invalid time specifier");

				let instance = self.basis.date().and_time(specifier_time).unwrap();

				self.basis = self.basis.to_owned() + chrono::Duration::days(1);

				Some(instance)
			}
		}
	}
}

impl<Tz: TimeZone> Specifier<Tz> {
	pub fn instances(&self, basis: &DateTime<Tz>) -> Instances<Tz> {
		let specifier = self;
		Instances {
			specifier,
			basis: basis.to_owned(),
		}
	}

	fn next(&self, n: usize, basis: &DateTime<Tz>) -> Vec<DateTime<Tz>> {
		self.instances(basis).take(n).collect()
	}
}

#[cfg(test)]
mod tests {
	mod specifier {
		use crate::Specifier;
		use chrono::DateTime;

		#[test]
		fn instances_exact() {
			let now = chrono::Local::now();
			let s = Specifier::Exact(now);
			assert_eq!(
				s.instances(&chrono::Local::now())
					.collect::<Vec<DateTime<chrono::Local>>>(),
				vec![now]
			);
		}

		#[test]
		fn instances_daily() {
			let t_ref = DateTime::parse_from_rfc3339("2020-01-16T10:15:00-05:00").unwrap();
			let s = Specifier::Daily {
				time: "07:00".to_string(),
			};
			assert_eq!(
				s.instances(&t_ref)
					.take(3)
					.collect::<Vec<DateTime<chrono::FixedOffset>>>(),
				vec![
					DateTime::parse_from_rfc3339("2020-01-16T07:00:00-05:00").unwrap(),
					DateTime::parse_from_rfc3339("2020-01-17T07:00:00-05:00").unwrap(),
					DateTime::parse_from_rfc3339("2020-01-18T07:00:00-05:00").unwrap(),
				]
			);
		}

		#[test]
		fn instances_weekly_start_after_date() {
			let t_ref = DateTime::parse_from_rfc3339("2020-01-16T10:15:00-05:00").unwrap();
			let s = Specifier::Weekly {
				day: "Tuesday".to_string(),
				time: "07:00".to_string(),
			};
			assert_eq!(
				s.instances(&t_ref)
					.take(3)
					.collect::<Vec<DateTime<chrono::FixedOffset>>>(),
				vec![
					DateTime::parse_from_rfc3339("2020-01-21T07:00:00-05:00").unwrap(),
					DateTime::parse_from_rfc3339("2020-01-28T07:00:00-05:00").unwrap(),
					DateTime::parse_from_rfc3339("2020-02-04T07:00:00-05:00").unwrap(),
				]
			);
		}

		#[test]
		fn instances_weekly_start_on_same_date() {
			let t_ref = DateTime::parse_from_rfc3339("2020-01-14T10:15:00-05:00").unwrap();
			let s = Specifier::Weekly {
				day: "Tuesday".to_string(),
				time: "07:00".to_string(),
			};
			assert_eq!(
				s.instances(&t_ref)
					.take(3)
					.collect::<Vec<DateTime<chrono::FixedOffset>>>(),
				vec![
					DateTime::parse_from_rfc3339("2020-01-14T07:00:00-05:00").unwrap(),
					DateTime::parse_from_rfc3339("2020-01-21T07:00:00-05:00").unwrap(),
					DateTime::parse_from_rfc3339("2020-01-28T07:00:00-05:00").unwrap(),
				]
			);
		}
	}
}
