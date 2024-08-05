use nom::Finish;
use nom::{combinator::all_consuming, IResult};
use nom_locate::LocatedSpan;
use nom_supreme::error::ErrorTree;

use crate::error::{format_parse_error, FormattedError};

pub type MyParseError<'a> = ErrorTree<RawSpan<'a>>;

// Use nom_locate's LocatedSpan as a wrapper around a string input
pub type RawSpan<'a> = LocatedSpan<&'a str>;

// the result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that latter
pub type ParserResult<'a, T> = IResult<RawSpan<'a>, T, MyParseError<'a>>;

pub trait Parse<'a>: Sized {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self>;
    fn parse_from_raw(input: &'a str) -> ParserResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }
    fn parse_format_error(i: &'a str) -> Result<Self, FormattedError<'a>> {
        let input = LocatedSpan::new(i);
        match all_consuming(Self::parse)(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(i, e)),
        }
    }
}
