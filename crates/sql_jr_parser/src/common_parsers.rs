use nom::{bytes::complete::take_while1, combinator::map};

use crate::types::{ParserResult, RawSpan};

/// Parse a unquoted sql identifier
pub(crate) fn identifier(i: RawSpan) -> ParserResult<String> {
    let take_aplphanumeric = take_while1(|c: char| c.is_alphanumeric());
    let to_string = |s: RawSpan| s.fragment().to_string();
    map(take_aplphanumeric, to_string)(i)
}

#[cfg(test)]
mod test {
  use nom_locate::LocatedSpan;

use super::identifier;

  #[test]
    fn test_parse_identifier() {
        let (remaining, parsed) = identifier(LocatedSpan::new("aVariable10 = aValue")).unwrap();

        assert_eq!(parsed, "aVariable10".to_string());
        assert_eq!(*remaining.fragment(), " = aValue");
    }
}
