use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::map,
    sequence::delimited,
};

use crate::types::{ParserResult, RawSpan};

/// Parse a unquoted sql identifier
pub(crate) fn identifier(i: RawSpan) -> ParserResult<String> {
    let take_aplphanumeric = take_while1(|c: char| c.is_alphanumeric());
    let to_string = |s: RawSpan| s.fragment().to_string();
    map(take_aplphanumeric, to_string)(i)
}

fn is_not_quote(c: char) -> bool {
    c != '\'' && c != '"'
}

pub(crate) fn parse_single_quote_str(i: RawSpan) -> ParserResult<String> {
    map(
        delimited(tag("'"), take_while(|c| c != '\''), tag("'")),
        |s: RawSpan| s.fragment().to_string(),
    )(i)
}

pub(crate) fn parse_double_quote_str(i: RawSpan) -> ParserResult<String> {
    map(
        delimited(tag("\""), take_while(|c| c != '"'), tag("\"")),
        |s: RawSpan| s.fragment().to_string(),
    )(i)
}

pub(crate) fn parse_string(i: RawSpan) -> ParserResult<String> {
    alt((parse_single_quote_str, parse_double_quote_str))(i)
}

#[cfg(test)]
mod test {
    use nom_locate::LocatedSpan;

    use super::*;

    #[test]
    fn test_parse_identifier() {
        let (remaining, parsed) = identifier(LocatedSpan::new("aVariable10 = aValue")).unwrap();

        assert_eq!(parsed, "aVariable10".to_string());
        assert_eq!(*remaining.fragment(), " = aValue");
    }

    #[test]
    fn test_parse_single_quote_string() {
        let (_, parsed) = parse_single_quote_str(LocatedSpan::new("'First', 'Second'")).unwrap();

        assert_eq!(parsed, "First".to_string())
    }

    #[test]
    fn test_parse_single_quote_string_containing_multiple_words() {
        let (_, parsed) =
            parse_single_quote_str(LocatedSpan::new("'First And, Only', 'Second'")).unwrap();

        assert_eq!(parsed, "First And, Only".to_string())
    }

    #[test]
    fn test_parse_double_quote_string() {
        let (_, parsed) =
            parse_double_quote_str(LocatedSpan::new("\"First\", \"Second\"")).unwrap();

        assert_eq!(parsed, "First".to_string())
    }

    #[test]
    fn test_parse_double_quote_string_containing_multiple_words() {
        let (_, parsed) =
            parse_double_quote_str(LocatedSpan::new("\"First And, Only\", \"Second\"")).unwrap();

        assert_eq!(parsed, "First And, Only".to_string())
    }
}
