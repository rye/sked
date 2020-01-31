#[derive(Debug)]
pub enum ParseError {
	UnknownOperator(String),
	MissingOperands,
	OperandType,
	Lopdf,
	Utf8(std::string::FromUtf8Error),
}

impl From<std::string::FromUtf8Error> for ParseError {
	fn from(e: std::string::FromUtf8Error) -> Self {
		Self::Utf8(e)
	}
}

pub type Result<T> = core::result::Result<T, ParseError>;
