#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::operands::OperandConversionError;

// This encompasses all errors that happen during lexing or parsing.
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SyntaxError {
    pub line: usize,
    pub error: SyntaxErrorKind,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyntaxErrorKind {
    InvalidSublabelName,
    NumberOutOfRange,
    UnknownCharacter,
    InconsistentLabelRedefinition,
    PrematureEndOfTokens,
    UnexpectedToken,
    IncompleteMemoryOperand,
    InvalidBaseRegister,
    BaseRegisterAlreadyExists,
    IndexRegisterAlreadyExists,
    RspCannotBeUsedAsIndexRegister,
    InvalidScaleFactor,
    DisplacementOutOfRange,
    DisplacementExceedsSignedDwordBounds,
    UnclosedMemoryOperand,
    InvalidLabelName,
    NoSuchLabel,
    OperandSizeMismatch,
    SourceDoesNotFitIntoDestination,
    ValueOutOfRangeForQword,
    ValueOutOfRangeForDword,
    ExpectedComma,
    ExpectedNewline,
    ExpectedScaleFactor,
    ExpectedLabel,
    ExpectedPlusOrMinusOrRBracket,
    ExpectedOperand,
    InvalidOperands,
}

impl From<OperandConversionError> for SyntaxErrorKind {
    fn from(value: OperandConversionError) -> Self {
        match value {
            OperandConversionError::WrongOperand => Self::InvalidOperands,
            OperandConversionError::ImmediateOutOfRangeForDword => Self::ValueOutOfRangeForDword,
            OperandConversionError::ImmediateOutOfRangeForQword => Self::ValueOutOfRangeForQword,
        }
    }
}
