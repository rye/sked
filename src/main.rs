use lopdf::content::*;
use lopdf::*;

use std::env;
use std::path;
//use std::path::PathBuf;

#[derive(Debug)]
pub enum PdfParseError {
	UnknownOperator(String),
	MissingOperands,
	OperandType,
	Lopdf,
	Utf8(std::string::FromUtf8Error),
}

impl From<std::string::FromUtf8Error> for PdfParseError {
	fn from(e: std::string::FromUtf8Error) -> Self {
		Self::Utf8(e)
	}
}

pub type PdfParseResult<T> = core::result::Result<T, PdfParseError>;

#[derive(Debug, PartialEq)]
enum Operation {
	BeginMarkedContentSequenceWithPropertyList,
	EndMarkedContentSequence,

	BeginTextObject,
	EndTextObject,

	SetColorSpaceForStrokingOperations,
	SetColorSpaceForNonstrokingOperations,
	SetColorForNonstrokingOperations,

	SetTextFontAndSize {
		name: Vec<u8>,
		size: f64,
	},
	SetCharacterSpacing {
		spacing: f64,
	},
	SetWordSpacing {
		spacing: f64,
	},
	SetTextMatrixAndTextLineMatrix {
		a: f64,
		b: f64,
		c: f64,
		d: f64,
		e: f64,
		f: f64,
	},
	ShowTextAllowingIndividualGlyphPositioning {
		body: String,
	},

	SaveGraphicsState,
	RestoreGraphicsState,

	MoveTextPosition {
		t_x: f64,
		t_y: f64,
	},
	MoveTextPositionAndSetLeading {
		t_x: f64,
		t_y: f64,
	},
	MoveToStartOfNextLine,

	AppendRectangleToPath {
		x: f64,
		y: f64,
		width: f64,
		height: f64,
	},
	FillPathUsingNonzeroWindingNumberRule,
	FillPathUsingNonzeroWindingNumberRuleObsolete,
	FillPathUsingEvenOddRule,
	SetClippingPathUsingNonzeroWindingNumberRule,
	EndPathWithoutFillingOrStroking,
}

impl core::convert::TryFrom<lopdf::content::Operation> for Operation {
	type Error = PdfParseError;

	fn try_from(operation: lopdf::content::Operation) -> PdfParseResult<Operation> {
		fn to_f64(object: &Object) -> Option<f64> {
			match object {
				Object::Real(x) => Some(*x),
				Object::Integer(x) => Some(*x as f64),
				_ => None,
			}
		}

		let operator: &str = operation.operator.as_str();
		match operator {
			"BDC" => Ok(Self::BeginMarkedContentSequenceWithPropertyList),
			"EMC" => Ok(Self::EndMarkedContentSequence),

			"BT" => Ok(Self::BeginTextObject),
			"ET" => Ok(Self::EndTextObject),

			"CS" => Ok(Self::SetColorSpaceForStrokingOperations),
			"cs" => Ok(Self::SetColorSpaceForNonstrokingOperations),
			"scn" => Ok(Self::SetColorForNonstrokingOperations),

			"Tf" => {
				match (operation.operands.get(0), operation.operands.get(1).map(to_f64).flatten()) {
					(Some(Object::Name(name)), Some(size)) => Ok(Self::SetTextFontAndSize { name: name.to_vec(), size }),
					_ => unimplemented!(),
				}
			},
			"Tc" => match operation.operands.get(0).map(to_f64).flatten() {
				Some(spacing) => Ok(Self::SetCharacterSpacing { spacing }),
				_ => Err(PdfParseError::OperandType),
			},
			"Tw" => match operation.operands.get(0).map(to_f64).flatten() {
				Some(spacing) => Ok(Self::SetWordSpacing { spacing }),
				_ => Err(PdfParseError::OperandType),
			},
			"Tm" => match (
				operation.operands.get(0).map(to_f64).flatten(),
				operation.operands.get(1).map(to_f64).flatten(),
				operation.operands.get(2).map(to_f64).flatten(),
				operation.operands.get(3).map(to_f64).flatten(),
				operation.operands.get(4).map(to_f64).flatten(),
				operation.operands.get(5).map(to_f64).flatten(),
			) {
				(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => {
					Ok(Self::SetTextMatrixAndTextLineMatrix { a, b, c, d, e, f })
				}
				_ => Err(PdfParseError::OperandType),
			},
			"TJ" => match operation.operands.get(0) {
				Some(Object::Array(array)) => {
					let body: PdfParseResult<String> = array
						.iter()
						.map(|element: &Object| -> PdfParseResult<String> {
							match element {
								Object::String(bytes, format) => {
									String::from_utf8(bytes.to_vec()).map_err(Into::into)
								}
								Object::Real(f) => Ok("".to_string()),
								Object::Integer(f) => Ok("".to_string()),
								_ => Err(PdfParseError::OperandType),
							}
						})
						.collect::<PdfParseResult<Vec<String>>>()
						.map(|strings| strings.concat());

					body.map(|body: String| Self::ShowTextAllowingIndividualGlyphPositioning { body })
				}
				None => Ok(Self::ShowTextAllowingIndividualGlyphPositioning {
					body: "".to_string(),
				}),
				_ => Err(PdfParseError::OperandType),
			},

			"q" => Ok(Self::SaveGraphicsState),
			"Q" => Ok(Self::RestoreGraphicsState),

			"Td" => match (
				operation.operands.get(0).map(to_f64).flatten(),
				operation.operands.get(1).map(to_f64).flatten(),
			) {
				(Some(t_x), Some(t_y)) => Ok(Self::MoveTextPosition { t_x, t_y }),
				_ => todo!(),
			},
			"TD" => match (
				operation.operands.get(0).map(to_f64).flatten(),
				operation.operands.get(1).map(to_f64).flatten(),
			) {
				(Some(t_x), Some(t_y)) => Ok(Self::MoveTextPositionAndSetLeading { t_x, t_y }),
				_ => todo!(),
			},
			"T*" => Ok(Self::MoveToStartOfNextLine),

			"re" => match (
				operation.operands.get(0).map(to_f64).flatten(),
				operation.operands.get(1).map(to_f64).flatten(),
				operation.operands.get(2).map(to_f64).flatten(),
				operation.operands.get(3).map(to_f64).flatten(),
			) {
				(Some(x), Some(y), Some(width), Some(height)) => Ok(Self::AppendRectangleToPath {
					x,
					y,
					width,
					height,
				}),
				_ => Err(PdfParseError::OperandType),
			},

			"f" => Ok(Self::FillPathUsingNonzeroWindingNumberRule),
			"F" => Ok(Self::FillPathUsingNonzeroWindingNumberRuleObsolete),
			"f*" => Ok(Self::FillPathUsingEvenOddRule),
			"W" => Ok(Self::SetClippingPathUsingNonzeroWindingNumberRule),
			"n" => Ok(Self::EndPathWithoutFillingOrStroking),

			op => Err(PdfParseError::UnknownOperator(op.to_string())),
		}
	}
}

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
