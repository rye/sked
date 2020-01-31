use lopdf::content::*;
use lopdf::*;

use std::env;
use std::path;

use std::collections::BTreeSet;

use sked::pdf::Operation;

fn main() {
	let file = env::args().nth(1).unwrap();
	println!("Loading from {}...", file);

	let path = path::Path::new(&file);

	let doc = Document::load(path).unwrap();

	let mut all_object_ids: BTreeSet<ObjectId> = doc.objects.keys().cloned().collect();

	println!("Starting with {} ids", all_object_ids.len());

	for page in doc.page_iter() {
		let object_ids: Vec<ObjectId> = doc.get_page_contents(page);

		let page_object_ids: BTreeSet<ObjectId> = object_ids.iter().cloned().collect();
		all_object_ids = all_object_ids.symmetric_difference(&page_object_ids).cloned().collect();

		for object_id in object_ids {
			let object: &Object = doc
				.get_object(object_id)
				.expect("couldn't dereference object");

			match object {
				Object::Stream(stream) => {
					println!("Decompressing stream...");

					let content: Vec<u8> = stream
						.decompressed_content()
						.expect("couldn't decompress stream");
					let content: Content = Content::decode(&content).expect("couldn't decode stream content");
					let operations: Vec<lopdf::content::Operation> = content.operations;

					for operation in operations {
						use core::convert::TryInto;

						let op: crate::Operation = operation.try_into().expect("couldn't convert operation");

						println!("{:?}", op);
					}
				}
				_ => todo!(),
			}
		}
	}

	println!("Still have {} ids", all_object_ids.len());

	for leftover_id in all_object_ids {
		match doc.get_object(leftover_id) {
			Ok(Object::Null) => {},
			Ok(Object::Boolean(boolean)) => println!("{:?}: Boolean {{ {:?} }}", leftover_id, boolean),
			Ok(Object::Integer(integer)) => println!("{:?}: Integer {{ {:?} }}", leftover_id, integer),
			Ok(Object::Real(real)) => println!("{:?}: Real {{ {:?} }}", leftover_id, real),
			Ok(Object::Name(vec)) => println!("{:?}: Name {{ {:?} }}", leftover_id, vec),
			Ok(Object::String(bytes, format)) => println!("{:?}: String {{ {:?}, {:?} }}", leftover_id, bytes, format),
			Ok(Object::Array(objects)) => println!("{:?}: Array {{ {:?} }}", leftover_id, objects),
			Ok(Object::Dictionary(dictionary)) => println!("{:?}: Dictionary {{ {:?} }}", leftover_id, dictionary),
			Ok(Object::Stream(stream)) => println!("{:?}: Stream {{ {:?} }}", leftover_id, stream),
			Ok(Object::Reference(object_id)) => println!("{:?}: Reference {{ {:?} }}", leftover_id, object_id),
			Err(e) => println!("Error dereferencing object with id {:?}: {:?}", leftover_id, e),
		}
	}

	println!("Document trailer: {:?}", doc.trailer);
}
