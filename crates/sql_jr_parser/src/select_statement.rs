use nom::{
    bytes::complete::tag_no_case,
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::{
    common_parsers::identifier,
    types::{Parse, ParserResult, RawSpan},
};

/// The table and its columns to select
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub(crate) struct SelectStatement {
    table: String,
    columns: Vec<String>,
}

fn parse_table_name(input: RawSpan<'_>) -> ParserResult<'_, String> {
    preceded(
        tuple((multispace1, tag_no_case("from"), multispace1)),
        identifier.context("Table Name"),
    )(input)
}

fn parse_column_names(input: RawSpan<'_>) -> ParserResult<'_, Vec<String>> {
    preceded(
        tuple((tag_no_case("select"), multispace1)),
        separated_list1(
            tuple((char(','), multispace0)),
            identifier.context("Column Name"),
        ),
    )(input)
}

impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self> {
        map(
            tuple((parse_column_names, parse_table_name)),
            |(columns, table)| Self { table, columns },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use nom_locate::LocatedSpan;

    use super::*;

    #[test]
    fn test_parse_table_name() {
        let (_, parsed) = parse_table_name(LocatedSpan::new(" FROM PERSON")).unwrap();

        assert_eq!(parsed, "PERSON".to_string());
    }

    #[test]
    fn test_parse_table_name_ended_with_semicolon() {
        let (_, parsed) = parse_table_name(LocatedSpan::new(" FROM PERSON;")).unwrap();

        assert_eq!(parsed, "PERSON".to_string());
    }

    #[test]
    fn test_parse_column_names() {
        let (_, parsed) = parse_column_names(LocatedSpan::new(
            "SELECT CustomerName, City FROM Customers;",
        ))
        .unwrap();

        assert_eq!(parsed, vec!["CustomerName".to_string(), "City".to_string()]);
    }

    #[test]
    fn test_parse_select_statement() {
        let (_, statement) =
            SelectStatement::parse_from_raw("SELECT CustomerName, City FROM Customers;").unwrap();

        assert_eq!(
            statement,
            SelectStatement {
                table: "Customers".to_string(),
                columns: vec!["CustomerName".to_string(), "City".to_string()]
            }
        )
    }
}
