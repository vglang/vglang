/// Error returns by `parser` mod
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum ParseError {
    #[error("Syntax error of literal number, {0}")]
    Uint(UnitKind),
    #[error("Syntax error of property, {0}")]
    Prop(PropKind),

    #[error("Syntax error of call exp, {0}")]
    Call(CallKind),

    #[error("Syntax error of call exp, {0}")]
    Type(TypeKind),
}

/// Error kind of unit parsing.
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum TypeKind {
    #[error("unrecognized array length.")]
    Uint,

    #[error("miss `;` for array declaration.")]
    Semicolons,

    #[error("miss array/list start tag `[`")]
    SquareBracketStart,

    #[error("miss array/list end tag `]`")]
    SquareBracketEnd,

    #[error("miss data name.")]
    Data,
}

/// Error kind of unit parsing.
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum CallKind {
    #[error("expect call expr parameter list end tag `)`")]
    ParamEnd,
}

/// Error kind of unit parsing.
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum PropKind {
    #[error("expect property end tag `]`")]
    MissEnd,
}

/// Error kind of unit parsing.
#[derive(Debug, thiserror::Error, PartialEq, PartialOrd)]
pub enum UnitKind {
    #[error("miss hexadecimal body.")]
    MissBody,
}
