use thiserror::Error;
use std::borrow::Cow;

#[derive(Error, Debug, PartialEq)]
pub enum ASTMError {
    // data
    #[error("Error decoding ASCII frame. {0}")]
    DecodingASCIIFrame(Cow<'static, str>),
    #[error("Error encoding ASCII frame. {0}")]
    EncodingASCIIFrame(Cow<'static, str>),
    #[error("Error decoding WINDOWS-1251 frame. {0}")]
    DecodingWINDOWS1251Frame(Cow<'static, str>),
    #[error("Error encoding WINDOWS-1251 frame. {0}")]
    EncodingWINDOWS1251Frame(Cow<'static, str>),
    #[error("Error decoding UTF8 frame. {0}")]
    DecodingUTF8Frame(std::str::Utf8Error),
    #[error("Invalid frame start character <STX>.")]
    InvalidSTXCharacter,
    #[error("Missing frame start character <STX>.")]
    MissingSTXCharacter,
    #[error("Invalid frame number.")]
    InvalidFrameNumber,
    #[error("Missing frame number.")]
    MissingFrameNumber,
    #[error("Frame number is not a numeric char.")]
    InvalidFrameNumberNotNumeric,
    #[error("Missing C1 checksum value.")]
    MissingC1ChecksumValue,
    #[error("Invalid C1 checksum value.")]
    InvalidC1ChecksumValue,
    #[error("Missing C2 checksum value.")]
    MissingC2ChecksumValue,
    #[error("Invalid C2 checksum value.")]
    InvalidC2ChecksumValue,
    #[error("Invalid carriage return character <CR>.")]
    InvalidCRCharacter,
    #[error("Missing carriage return character <CR>.")]
    MissingCRCharacter,
    #[error("Invalid line feed character <LF>.")]
    InvalidLFCharacter,
    #[error("Missing line feed character <LF>.")]
    MissingLFCharacter,
    #[error("Oversized Message. The data content has more than 63993 chars.")]
    OversizedMessage,
    #[error("Defective frame. Checksum must be: {0}.")]
    DefectiveFrame(String),

    // records
    #[error("Invalid Processing ID value.")]
    InvalidProcessingIDValue,
    #[error("Invalid Patient Sex value.")]
    InvalidPatientSexValue,
    #[error("Missing Measurement value.")]
    MissingMeasurementValue,
    #[error("Invalid Marital Status value.")]
    InvalidMaritalStatusValue,
    #[error("Invalid Date and Time value. {0}")]
    InvalidDateAndTimeValue(chrono::format::ParseError),
    #[error("Invalid Date value. {0}")]
    InvalidDateValue(chrono::format::ParseError),
    #[error("Could not parse value into float number. {0}")]
    ParseFloatNumber(std::num::ParseFloatError),
    #[error("Could not parse value into int number. {0}")]
    ParseIntNumber(std::num::ParseIntError),
    #[error("Missing Sequence Number value.")]
    MissingSequenceNumberValue,

    // comms
    #[error("Error binding listener. {0}")]
    TcpBind(String),
    #[error("Error accepting incoming connection. {0}")]
    TcpAccept(String),
}