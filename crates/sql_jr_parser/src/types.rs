use nom::IResult;
use nom_locate::LocatedSpan;

// Use nom_locate's LocatedSpan as a wrapper around a string input
pub type RawSpan<'a> = LocatedSpan<&'a str>;

// the result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that latter
pub type ParserResult<'a, T> = IResult<RawSpan<'a>, T>;

pub trait Parse<'a>: Sized {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self>;
    fn parse_from_raw(input: &'a str) -> ParserResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }
}
