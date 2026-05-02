import { SimulatorError, SyntaxErrorKind } from "s86-lib"

export const SYNTAX_ERRORS: Record<SyntaxErrorKind, string> = {
    [SyntaxErrorKind.InvalidSublabelName]: "invalid sublabel name",
    [SyntaxErrorKind.NumberOutOfRange]: "number out of range",
    [SyntaxErrorKind.InconsistentLabelRedefinition]: "inconsistent label redefinition",
    [SyntaxErrorKind.UnexpectedToken]: "unexpected token",
    [SyntaxErrorKind.IncompleteMemoryOperand]: "incomplete memory operand",
    [SyntaxErrorKind.InvalidBaseRegister]: "invalid base register",
    [SyntaxErrorKind.BaseRegisterAlreadyExists]: "base register already given",
    [SyntaxErrorKind.IndexRegisterAlreadyExists]: "index register already given",
    [SyntaxErrorKind.RspCannotBeUsedAsIndexRegister]: "rsp cannot be used as index register",
    [SyntaxErrorKind.InvalidScaleFactor]: "scale factor may only be 1, 2, 4 or 8",
    [SyntaxErrorKind.DisplacementOutOfRange]: "displacement exceeds signed dword bounds",
    [SyntaxErrorKind.UnclosedMemoryOperand]: "unclosed memory operand",
    [SyntaxErrorKind.InvalidLabelName]: "invalid label name",
    [SyntaxErrorKind.NoSuchLabel]: "no such label found",
    [SyntaxErrorKind.OperandSizeMismatch]: "mismatched operand sizes",
    [SyntaxErrorKind.SourceDoesNotFitIntoDestination]: "source does not fit into destination",
    [SyntaxErrorKind.ValueOutOfRangeForQword]: "value out of range for qword",
    [SyntaxErrorKind.ValueOutOfRangeForDword]: "value out of range for dword",
    [SyntaxErrorKind.ExpectedComma]: "expected comma",
    [SyntaxErrorKind.ExpectedNewline]: "expected newline after instruction",
    [SyntaxErrorKind.ExpectedScaleFactor]: "expected scale factor",
    [SyntaxErrorKind.ExpectedLabel]: "expected label",
    [SyntaxErrorKind.ExpectedPlusOrMinusOrRBracket]: "expected plus, minus or bracket",
    [SyntaxErrorKind.ExpectedOperand]: "expected operand",
    [SyntaxErrorKind.InvalidOperands]: "invalid operand",
}

export const SIMULATOR_ERRORS: Record<SimulatorError, string> = {
    [SimulatorError.InvalidMemAccess]: "invalid memory access",
}