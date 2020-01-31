use lopdf::content::*;
use lopdf::*;

use std::env;
use std::path;

use sked::pdf::Operation;

fn main() {
	let file = env::args().nth(1).unwrap();
	println!("Loading from {}...", file);

	let path = path::Path::new(&file);

	let doc = Document::load(path).unwrap();

	for page in doc.page_iter() {
		let object_ids: Vec<ObjectId> = doc.get_page_contents(page);

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

						use crate::Operation::*;

						match op {
							MoveTextPosition { .. }
							| MoveTextPositionAndSetLeading { .. }
							| ShowText { .. }
							| ShowTextAllowingIndividualGlyphPositioning { .. }
							| AppendRectangleToPath { .. }
							| FillPathUsingNonzeroWindingNumberRule
							| SetTextMatrixAndTextLineMatrix { .. }
							| SetCharacterSpacing { .. }
							| SetWordSpacing { .. }
							| EndPathWithoutFillingOrStroking
							| BeginMarkedContentSequenceWithPropertyList
							| SetClippingPathUsingNonzeroWindingNumberRule
							| SetTextFontAndSize { .. }
							| MoveToStartOfNextLine
							| EndMarkedContentSequence
							| BeginTextObject
							| SaveGraphicsState
							| RestoreGraphicsState
							| SetColorSpaceForNonstrokingOperations { .. }
							| SetColorSpaceForStrokingOperations { .. }
							| EndTextObject => {}
							_ => println!("{:?}", op),
						}
					}
				}
				_ => todo!(),
			}
		}
	}
}
