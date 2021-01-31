use lopdf::{Document, Object, ObjectId};

#[cfg(feature = "simple_logger")]
use simple_logger::SimpleLogger;

#[allow(unused)]
use log::{debug, error, info, log, trace, warn};

use std::env;
use std::path;

use std::collections::BTreeSet;

use sked::pdf::Pdf;

fn main() {
	#[cfg(feature = "simple_logger")]
	SimpleLogger::new()
		.init()
		.expect("couldn't init simple_logger");

	let file = env::args().nth(1).unwrap();
	println!("Loading from {}...", file);

	let path = path::Path::new(&file);

	let mut doc = Document::load(path).unwrap();

	let unvisited_object_ids: BTreeSet<ObjectId> = doc.objects.keys().cloned().collect();

	let pdf: Pdf = Pdf::new().version(&doc.version);

	let walked: BTreeSet<ObjectId> = doc
		.traverse_objects(|object: &mut Object| {
			info!("Traversing {:?}", object);
		})
		.iter()
		.cloned()
		.collect();

	println!(
		"Leftovers: {:?}",
		unvisited_object_ids
			.difference(&walked)
			.collect::<Vec<&ObjectId>>()
	);
}
