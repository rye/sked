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
		size: f32,
	},
	SetCharacterSpacing {
		spacing: f32,
	},
	SetWordSpacing {
		spacing: f32,
	},
	SetTextMatrixAndTextLineMatrix {
		a: f32,
		b: f32,
		c: f32,
		d: f32,
		e: f32,
		f: f32,
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
		t_x: f32,
		t_y: f32,
	},
	MoveTextPositionAndSetLeading {
		t_x: f32,
		t_y: f32,
	},
	MoveToStartOfNextLine,

	AppendRectangleToPath {
		x: f32,
		y: f32,
		width: f32,
		height: f32,
	},
	FillPathUsingNonzeroWindingNumberRule,
	FillPathUsingNonzeroWindingNumberRuleObsolete,
	FillPathUsingEvenOddRule,
	SetClippingPathUsingNonzeroWindingNumberRule,
	EndPathWithoutFillingOrStroking,
}

pub struct GraphicsState {
	current_transformation_matrix: [f64; 6],
	clipping_path: (),
	color_space: (),
	color: (),
	text_state: (),
	line_width: f64,
	line_cap: u32,
	line_join: u32,
	miter_limit: f64,
	//dash_pattern: ()
	rendering_intent: String,
	stroke_adjustment: bool,
	blend_mode: String,
	soft_mask: String,
	alpha_constant: f64,
}

pub struct CoordinateSpace {
	origin: [f32; 2],
	orientation: ([f64; 2], [f64; 2]),
	axis_length: (f64, f64),
}

/// A Coordinate Transformation Matrix (CTM)
///
/// A 3x3 matrix, but with only the first two columns configured.
/// The parameters are specified left-to-right, so that the subscript-to-cell
/// mapping is:
///
/// ```text
/// [  .0   .1  0.0 ]
/// [  .2   .3  0.0 ]
/// [  .4   .5  1.0 ]
/// ```
pub struct TransformationMatrix((f64, f64), (f64, f64), (f64, f64));
pub struct Coordinates(f64, f64);

impl Coordinates {
	fn transform(&self, ctm: &TransformationMatrix) -> Coordinates {
		let x_prime: f64 = ctm.0 .0 * self.0 + ctm.1 .0 * self.1 + ctm.2 .0;
		let y_prime: f64 = ctm.0 .1 * self.0 + ctm.1 .1 * self.1 + ctm.2 .1;
		Coordinates(x_prime, y_prime)
	}
}

impl CoordinateSpace {
	#[must_use]
	pub fn from_page_dictionary(dictionary: &lopdf::Object) -> Self {
		let dictionary: &lopdf::Dictionary = match dictionary {
			lopdf::Object::Dictionary(dictionary) => dictionary,

			// According to Section 7.7.3.3 of the PDF spec, the leaves of the page
			// tree are page objects, each of which is a dictionary specifying the
			// attributes of a single page of the document.  It is, therefore, an
			// error (logic or otherwise) to have anything besides a Dictionary object
			// stored in the leaves of the page tree, and therefore we should not
			// support that.
			//
			// TODO Consider, instead of unwinding via panic, returning a Result?
			_ => unreachable!(),
		};

		let media_box: Vec<f32> = dictionary
			.get(b"MediaBox")
			.map(|object| match object {
				Object::Array(array) => array
					.iter()
					.map(|element| match element {
						Object::Real(number) => number,
						_ => unreachable!(),
					})
					.copied()
					.collect(),
				_ => unreachable!(),
			})
			.ok()
			.unwrap();
		let crop_box: Vec<f32> = dictionary
			.get(b"CropBox")
			.map(|object| match object {
				Object::Array(array) => array
					.iter()
					.map(|element| match element {
						Object::Real(number) => number,
						_ => unreachable!(),
					})
					.copied()
					.collect(),
				_ => unreachable!(),
			})
			.unwrap_or(media_box);

		let origin: [f32; 2] = [crop_box[0], crop_box[1]];
		// TODO fix?
		let orientation: ([f64; 2], [f64; 2]) = ([1.0, 0.0], [0.0, 1.0]);
		let axis_length: (f64, f64) = (1.0, 1.0);

		Self {
			origin,
			orientation,
			axis_length,
		}
	}
}

impl core::convert::TryFrom<lopdf::content::Operation> for Operation {
	type Error = error::ParseError;

	// TODO(rye): This is a very long method.  Can we make it smaller?
	#[allow(clippy::many_single_char_names)]
	fn try_from(operation: lopdf::content::Operation) -> error::Result<Operation> {
		/// Since `Object::as_f64` fails if `Object` is not a `Object::Real`, this
		/// function can coerce an `Object::Integer` to an `Object::Real`, and will
		/// unwrap into an `Option<f64>` if either of those variants are given.
		fn to_f32(object: &Object) -> Option<f32> {
			match object {
				Object::Real(x) => Some(*x),
				Object::Integer(x) => todo!(),
				_ => None,
			}
		}

		match (operation.operator.as_str(), operation.operands) {
			("BDC", _) => Ok(Self::BeginMarkedContentSequenceWithPropertyList),
			("EMC", _) => Ok(Self::EndMarkedContentSequence),

			("BT", _) => Ok(Self::BeginTextObject),
			("ET", _) => Ok(Self::EndTextObject),

			("CS", opds) => match opds.first() {
				Some(Object::Name(name)) => {
					Ok(Self::SetColorSpaceForStrokingOperations { name: name.clone() })
				}
				_ => todo!(),
			},
			("cs", opds) => match opds.first() {
				Some(Object::Name(name)) => {
					Ok(Self::SetColorSpaceForNonstrokingOperations { name: name.clone() })
				}
				_ => todo!(),
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

			#[allow(clippy::get_first)]
			("Tf", opds) => match (opds.get(0), opds.get(1).and_then(to_f32)) {
				(Some(Object::Name(name)), Some(size)) => Ok(Self::SetTextFontAndSize {
					name: name.clone(),
					size,
				}),
				_ => todo!(),
			},
			("Tc", opds) => match opds.first().and_then(to_f32) {
				Some(spacing) => Ok(Self::SetCharacterSpacing { spacing }),
				_ => Err(error::ParseError::OperandType),
			},
			("Tw", opds) => match opds.first().and_then(to_f32) {
				Some(spacing) => Ok(Self::SetWordSpacing { spacing }),
				_ => Err(error::ParseError::OperandType),
			},
			("Tm", opds) => match (
				#[allow(clippy::get_first)]
				opds.get(0).and_then(to_f32),
				opds.get(1).and_then(to_f32),
				opds.get(2).and_then(to_f32),
				opds.get(3).and_then(to_f32),
				opds.get(4).and_then(to_f32),
				opds.get(5).and_then(to_f32),
			) {
				// N.B. In the PDF spec these use the names a, b, c, d, e, and f; these
				// are used as generic parameters in the 3x3 transformation matrices,
				// filling the first two columns column-wise.
				(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => {
					Ok(Self::SetTextMatrixAndTextLineMatrix { a, b, c, d, e, f })
				}
				_ => Err(error::ParseError::OperandType),
			},
			("TJ", opds) => match opds.first() {
				Some(Object::Array(array)) => {
					let body: error::Result<String> = array
						.iter()
						.map(|element: &Object| -> error::Result<String> {
							match element {
								Object::String(bytes, _format) => {
									String::from_utf8(bytes.clone()).map_err(Into::into)
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
			("Tj", opds) => match opds.first() {
				Some(Object::String(bytes, _format)) => {
					let body = String::from_utf8(bytes.clone()).map_err(Into::into);
					body.map(|body: String| Self::ShowText { body })
				}
				_ => Err(error::ParseError::OperandType),
			},

			("q", _) => Ok(Self::SaveGraphicsState),
			("Q", _) => Ok(Self::RestoreGraphicsState),

			#[allow(clippy::get_first)]
			("Td", opds) => match (opds.get(0).and_then(to_f32), opds.get(1).and_then(to_f32)) {
				(Some(t_x), Some(t_y)) => Ok(Self::MoveTextPosition { t_x, t_y }),
				_ => todo!(),
			},
			#[allow(clippy::get_first)]
			("TD", opds) => match (opds.get(0).and_then(to_f32), opds.get(1).and_then(to_f32)) {
				(Some(t_x), Some(t_y)) => Ok(Self::MoveTextPositionAndSetLeading { t_x, t_y }),
				_ => todo!(),
			},
			("T*", _) => Ok(Self::MoveToStartOfNextLine),

			#[allow(clippy::get_first)]
			("re", opds) => match (
				opds.get(0).and_then(to_f32),
				opds.get(1).and_then(to_f32),
				opds.get(2).and_then(to_f32),
				opds.get(3).and_then(to_f32),
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
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn version(mut self, version: &str) -> Self {
		self.version = Some(version.to_string());
		self
	}
}
