#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

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
    ExpectedMemoryOperand,
    ExpectedScaleFactor,
    ExpectedLabel,
    ExpectedRegisterOrMemory,
    ExpectedRegisterOrImmediate,
    ExpectedRegisterMemoryOrImmediate,
    ExpectedPlusOrMinusOrRBracket,
    ExpectedOperand,
}
