// https://github.com/Geal/nom/issues/1289
use nom::{error, IResult, Parser};

pub enum FoldWhile<T, E> {
    Continue(T),
    Done(T),
    Throw(E),
}

// copied and modified from https://docs.rs/nom/6.1.2/src/nom/multi/mod.rs.html#723-760
pub fn fold_many0_while<I, O, E, F, G, H, R>(
    mut parser: F,
    mut init: H,
    mut f: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + PartialEq,
    F: Parser<I, O, E>,
    E: error::ParseError<I>,
    G: FnMut(R, O) -> FoldWhile<R, nom::Err<E>>,
    H: FnMut() -> R,
{
    use nom::error::ErrorKind;
    use nom::Err;
    move |i: I| {
        let mut res = init();
        let mut input = i;

        loop {
            let i_ = input.clone();
            match parser.parse(i_) {
                Ok((i, o)) => {
                    // loop trip must always consume (otherwise infinite loops)
                    if i == input {
                        return Err(Err::Error(E::from_error_kind(input, ErrorKind::Many0)));
                    }

                    res = match f(res, o) {
                        FoldWhile::Continue(v) => v,
                        FoldWhile::Done(v) => return Ok((i, v)),
                        FoldWhile::Throw(e) => return Err(e),
                    };
                    input = i;
                }
                Err(Err::Error(_)) => {
                    return Ok((input, res));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
