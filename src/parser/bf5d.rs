use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::anychar,
    combinator::{map, map_res, recognize},
    error::{ParseError, VerboseError},
    sequence::{pair, tuple},
    IResult,
};

use super::types::*;
use super::utils::{fold_many0_while, FoldWhile};

impl Token {
    fn from(input: char) -> Result<Token, () /* TODO: error message */> {
        use self::JumpType::*;
        use self::MoveDirection::*;
        use self::Token::*;
        use self::UpdateType::*;

        match input {
            '<' => Ok(Move(Left)),
            '>' => Ok(Move(Right)),
            '^' => Ok(Move(Up)),
            'v' => Ok(Move(Down)),
            '+' => Ok(Update(Increment)),
            '-' => Ok(Update(Decrement)),
            '[' => Ok(Jump {
                type_: IfZero,
                index: 0,
            }),
            ']' => Ok(Jump {
                type_: IfNotZero,
                index: 0,
            }),
            '.' => Ok(Write),
            ',' => Ok(Read),
            '~' => Ok(Rewind),
            '(' => Ok(Spawn { index: 0 }),
            ')' => Ok(Kill),
            '@' => Ok(Await),
            _ => Err(()),
        }
    }
}

// fn integer(input: &str) -> IResult<&str, usize> {
//     map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
//         usize::from_str_radix(out, 10)
//     })(input)
// }

pub fn c_comment(i: &str) -> IResult<&str, &str, BF5DParseError> {
    alt((
        recognize(tuple((tag("/*"), take_until("*/"), tag("*/")))),
        recognize(pair(tag("//"), is_not("\n\r"))),
    ))(i)
}

pub fn parse<'a>(input: &str) -> Result<Vec<Token>, BF5DParseError> {
    enum Temp {
        Token(Token),
        Comment(String),
    }

    use nom::Err::*;

    match fold_many0_while(
        alt((
            map(map_res(anychar, Token::from), Temp::Token),
            map(c_comment, |c| Temp::Comment(c.to_string())),
            map(anychar, |c| Temp::Comment(c.to_string())),
        )),
        // map_res(anychar, Token::from),
        || {
            (
                Vec::new(), /* tokens */
                Vec::new(), /* bracket stack */
                Vec::new(), /* parens stack */
                0usize,     /* index */
            )
        },
        |(mut tokens, mut brackets, mut parens, i), token| {
            if let Temp::Token(token) = token {
                match token {
                    Token::Jump {
                        type_: JumpType::IfZero,
                        index: _,
                    } => {
                        brackets.push(i);
                        tokens.push(token);
                    }
                    Token::Spawn { index: _ } => {
                        parens.push(i);
                        tokens.push(token);
                    }
                    Token::Jump {
                        type_: JumpType::IfNotZero,
                        index: _,
                    } => {
                        if let Some(last) = brackets.pop() {
                            tokens.push(Token::Jump {
                                type_: JumpType::IfNotZero,
                                index: last,
                            });
                            tokens[last] = Token::Jump {
                                type_: JumpType::IfZero,
                                index: i,
                            };
                        } else {
                            return FoldWhile::Throw(Failure(BF5DParseError::new(
                                "unmatched bracket",
                                i,
                            )));
                        }
                    }
                    Token::Kill => {
                        if let Some(last) = parens.pop() {
                            tokens.push(token);
                            tokens[last] = Token::Spawn { index: i };
                        } else {
                            return FoldWhile::Throw(Failure(BF5DParseError::new(
                                "unmatched parentheses",
                                i,
                            )));
                        }
                    }
                    token => {
                        tokens.push(token);
                    }
                }
                FoldWhile::Continue((tokens, brackets, parens, i + 1))
            } else {
                FoldWhile::Continue((tokens, brackets, parens, i))
            }
        },
    )(input)
    {
        Ok((_, (tokens, brackets, parens, _))) => {
            match (!brackets.is_empty(), !parens.is_empty()) {
                (true, _) => Err(BF5DParseError::new(
                    "unmatched parentheses",
                    *parens.last().unwrap(),
                )),
                (_, true) => Err(BF5DParseError::new(
                    "unmatched parentheses",
                    *parens.last().unwrap(),
                )),
                _ => Ok(tokens),
            }
        }
        Err(nom::Err::Error(e)) => Err(e),
        _ => panic!("this should never happen"),
    }
}
