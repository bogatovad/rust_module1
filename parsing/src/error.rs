/// errors for parsing
#[derive(Debug)]
pub enum ParsingError {
    ParseDateError(String),
    XmlError(String),
    IoError(String),
    MissingField(String),
    InvalidAmount(String),
    InvalidIndicator(String),
    ConversionError(String),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::ParseDateError(s) => write!(f, "Date parsing error: {}", s),
            ParsingError::XmlError(s) => write!(f, "XML error: {}", s),
            ParsingError::IoError(s) => write!(f, "IO error: {}", s),
            ParsingError::MissingField(s) => write!(f, "Missing field: {}", s),
            ParsingError::InvalidAmount(s) => write!(f, "Invalid amount: {}", s),
            ParsingError::InvalidIndicator(s) => write!(f, "Invalid indicator: {}", s),
            ParsingError::ConversionError(s) => write!(f, "Conversion error: {}", s),
        }
    }
}

impl std::error::Error for ParsingError {}

impl From<quick_xml::DeError> for ParsingError {
    fn from(err: quick_xml::DeError) -> Self {
        ParsingError::XmlError(err.to_string())
    }
}

impl From<quick_xml::SeError> for ParsingError {
    fn from(err: quick_xml::SeError) -> Self {
        ParsingError::XmlError(err.to_string())
    }
}

impl From<std::io::Error> for ParsingError {
    fn from(err: std::io::Error) -> Self {
        ParsingError::IoError(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for ParsingError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParsingError::InvalidAmount(err.to_string())
    }
}

impl From<chrono::format::ParseError> for ParsingError {
    fn from(err: chrono::format::ParseError) -> Self {
        ParsingError::ParseDateError(err.to_string())
    }
}