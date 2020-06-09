use lopdf::Object;

pub mod error;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Operation {
	BeginMarkedContentSequenceWithPropertyList,
	EndMarkedContentSequence,

	BeginTextObject,
	EndTextObject,

	SetColorSpaceForStrokingOperations {
		name: Vec<u8>,
	},
	SetColorSpaceForNonstrokingOperations {
		name: Vec<u8>,
	},
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
	ShowText {
		body: String,
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
	type Error = error::ParseError;

	#[allow(clippy::many_single_character_name)]
	fn try_from(operation: lopdf::content::Operation) -> error::Result<Operation> {
		/// Since Object::as_f64 fails if Object is not a Object::Real, this
		/// function can coerce an Object::Integer to an Object::Real, and will
		/// unwrap into an Option<f64> if either of those variants are given.
		fn to_f64(object: &Object) -> Option<f64> {
			match object {
				Object::Real(x) => Some(*x),
				Object::Integer(x) => Some(*x as f64),
				_ => None,
			}
		}

		match (operation.operator.as_str(), operation.operands) {
			("BDC", _) => Ok(Self::BeginMarkedContentSequenceWithPropertyList),
			("EMC", _) => Ok(Self::EndMarkedContentSequence),

			("BT", _) => Ok(Self::BeginTextObject),
			("ET", _) => Ok(Self::EndTextObject),

			("CS", opds) => match opds.get(0) {
				Some(Object::Name(name)) => Ok(Self::SetColorSpaceForStrokingOperations {
					name: name.to_vec(),
				}),
				_ => unimplemented!(),
			},
			("cs", opds) => match opds.get(0) {
				Some(Object::Name(name)) => Ok(Self::SetColorSpaceForNonstrokingOperations {
					name: name.to_vec(),
				}),
				_ => unimplemented!(),
			},
			("scn", _) => {
				// Same as SCN, but for Nonstroking operations.
				//
				// SCN: Operands are the same as SC, but also supports Pattern,
				// Separation, DeviceN, and ICCBased colour spaces.
				//
				// SC: Set the color to use for stroking operations in a device,
				// CIE-based (other than ICCBased), or Indexed colour space.  The number
				// of operands required and their interpretation depends on the current
				// stroking colour space.
				//
				// - For DeviceGray, CalGray, and Indexed colour spaces, one operand
				//   shall be required.
				//
				// - For DeviceRGB, CalRGB, and Lab colour spaces, three operands shall
				//   be required.
				//
				// - For DeviceCMYK, four operands shall be required.
				//
				// TODO implement
				Ok(Self::SetColorForNonstrokingOperations)
			}

			("Tf", opds) => match (opds.get(0), opds.get(1).map(to_f64).flatten()) {
				(Some(Object::Name(name)), Some(size)) => Ok(Self::SetTextFontAndSize {
					name: name.to_vec(),
					size,
				}),
				_ => unimplemented!(),
			},
			("Tc", opds) => match opds.get(0).map(to_f64).flatten() {
				Some(spacing) => Ok(Self::SetCharacterSpacing { spacing }),
				_ => Err(error::ParseError::OperandType),
			},
			("Tw", opds) => match opds.get(0).map(to_f64).flatten() {
				Some(spacing) => Ok(Self::SetWordSpacing { spacing }),
				_ => Err(error::ParseError::OperandType),
			},
			("Tm", opds) => match (
				opds.get(0).map(to_f64).flatten(),
				opds.get(1).map(to_f64).flatten(),
				opds.get(2).map(to_f64).flatten(),
				opds.get(3).map(to_f64).flatten(),
				opds.get(4).map(to_f64).flatten(),
				opds.get(5).map(to_f64).flatten(),
			) {
				(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => {
					Ok(Self::SetTextMatrixAndTextLineMatrix { a, b, c, d, e, f })
				}
				_ => Err(error::ParseError::OperandType),
			},
			("TJ", opds) => match opds.get(0) {
				Some(Object::Array(array)) => {
					let body: error::Result<String> = array
						.iter()
						.map(|element: &Object| -> error::Result<String> {
							match element {
								Object::String(bytes, _format) => {
									String::from_utf8(bytes.to_vec()).map_err(Into::into)
								}
								Object::Real(_f) => Ok("".to_string()),
								Object::Integer(_f) => Ok("".to_string()),
								_ => Err(error::ParseError::OperandType),
							}
						})
						.collect::<error::Result<Vec<String>>>()
						.map(|strings| strings.concat());

					body.map(|body: String| Self::ShowTextAllowingIndividualGlyphPositioning { body })
				}
				None => Ok(Self::ShowTextAllowingIndividualGlyphPositioning {
					body: "".to_string(),
				}),
				_ => Err(error::ParseError::OperandType),
			},
			("Tj", opds) => match opds.get(0) {
				Some(Object::String(bytes, _format)) => {
					let body = String::from_utf8(bytes.to_vec()).map_err(Into::into);
					body.map(|body: String| Self::ShowText { body })
				}
				_ => Err(error::ParseError::OperandType),
			},

			("q", _) => Ok(Self::SaveGraphicsState),
			("Q", _) => Ok(Self::RestoreGraphicsState),

			("Td", opds) => match (
				opds.get(0).map(to_f64).flatten(),
				opds.get(1).map(to_f64).flatten(),
			) {
				(Some(t_x), Some(t_y)) => Ok(Self::MoveTextPosition { t_x, t_y }),
				_ => todo!(),
			},
			("TD", opds) => match (
				opds.get(0).map(to_f64).flatten(),
				opds.get(1).map(to_f64).flatten(),
			) {
				(Some(t_x), Some(t_y)) => Ok(Self::MoveTextPositionAndSetLeading { t_x, t_y }),
				_ => todo!(),
			},
			("T*", _) => Ok(Self::MoveToStartOfNextLine),

			("re", opds) => match (
				opds.get(0).map(to_f64).flatten(),
				opds.get(1).map(to_f64).flatten(),
				opds.get(2).map(to_f64).flatten(),
				opds.get(3).map(to_f64).flatten(),
			) {
				(Some(x), Some(y), Some(width), Some(height)) => Ok(Self::AppendRectangleToPath {
					x,
					y,
					width,
					height,
				}),
				_ => Err(error::ParseError::OperandType),
			},

			("f", _) => Ok(Self::FillPathUsingNonzeroWindingNumberRule),
			("F", _) => Ok(Self::FillPathUsingNonzeroWindingNumberRuleObsolete),
			("f*", _) => Ok(Self::FillPathUsingEvenOddRule),
			("W", _) => Ok(Self::SetClippingPathUsingNonzeroWindingNumberRule),
			("n", _) => Ok(Self::EndPathWithoutFillingOrStroking),

			(op, _) => Err(error::ParseError::UnknownOperator(op.to_string())),
		}
	}
}

#[derive(Debug, Default)]
pub struct Pdf {
	version: Option<String>,
}

impl Pdf {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn version(mut self, version: &str) -> Self {
		self.version = Some(version.to_string());
		self
	}
}
