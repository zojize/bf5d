use nom::error::{ErrorKind, FromExternalError, ParseError};

#[derive(Debug, Clone, Copy)]
pub enum MoveDirection {
    Left,  // '<'
    Right, // '>'
    Up,    // '^'
    Down,  // 'v'
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateType {
    Increment, // '+'
    Decrement, // '-'
               // potentially more
}

#[derive(Debug, Clone, Copy)]
pub enum JumpType {
    IfZero,    // '['
    IfNotZero, // ']'
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Move(MoveDirection),                    // '<', '>', '^', 'v'
    Update(UpdateType),                     // '+', '-'
    Jump { type_: JumpType, index: usize }, // '[', ']'
    Write,                                  // '.'
    Read,                                   // ','
    Rewind,                                 // '~'
    Spawn { index: usize },                 // '('
    Kill,                                   // ')'
    Await,                                  // '@'
}
#[derive(Debug, PartialEq)]
pub struct BF5DParseError {
    message: String,
    location: usize,
}

impl BF5DParseError {
    pub fn new(message: &str, location: usize) -> Self {
        BF5DParseError {
            message: message.to_string(),
            location,
        }
    }
}

impl<I> ParseError<I> for BF5DParseError {
    fn from_error_kind(_: I, _: ErrorKind) -> Self {
        BF5DParseError::new("", 0)
    }
    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I, E> FromExternalError<I, E> for BF5DParseError {
    /// Create a new error from an input position and an external error
    fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
        BF5DParseError {
            message: "".to_string(),
            location: 0,
        }
    }
}
