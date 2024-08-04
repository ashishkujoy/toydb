use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    error::context,
    multi::separated_list0,
    sequence::{preceded, tuple},
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::{
    common_parsers::{identifier, parse_string},
    types::{Parse, ParserResult, RawSpan},
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    table: String,
    columns: Vec<String>,
    // TODO: Modify this to support other types of values
    values: Vec<String>,
}

fn parse_table_name(input: RawSpan<'_>) -> ParserResult<'_, String> {
    let insert_into = tuple((
        tag_no_case("insert"),
        multispace1,
        tag_no_case("into"),
        multispace1,
    ));
    preceded(insert_into, identifier.context("Table Name"))(input)
}

fn parse_column_names(input: RawSpan<'_>) -> ParserResult<'_, Vec<String>> {
    let column_names = map(
        tuple((
            char('('),
            multispace0,
            separated_list0(
                tuple((multispace0, char(','), multispace1)),
                identifier.context("Column Name"),
            ),
            multispace0,
            char(')'),
        )),
        |(_, _, columns, _, _)| columns,
    );

    let empty_col_parser = |_input: RawSpan<'_>| Ok((input, vec![]));

    context("Column Names", alt((column_names, empty_col_parser)))(input)
}

fn parse_column_values(input: RawSpan<'_>) -> ParserResult<'_, Vec<String>> {
    context(
        "Values",
        map(
            tuple((
                tag_no_case("values"),
                multispace0,
                char('('),
                multispace0,
                separated_list0(tuple((multispace0, char(','), multispace1)), parse_string),
                multispace0,
                char(')'),
            )),
            |(_, _, _, _, column_value, _, _)| column_value,
        ),
    )(input)
}

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self> {
        map(
            tuple((
                parse_table_name,
                multispace0,
                parse_column_names,
                multispace0,
                parse_column_values,
            )),
            |(table, _, columns, _, values)| InsertStatement {
                table,
                columns,
                values,
            },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use nom_locate::LocatedSpan;

    use super::*;

    #[test]
    fn test_parse_table_name() {
        let (_, parsed) = parse_table_name(LocatedSpan::new("insert into Person")).unwrap();

        assert_eq!(parsed, "Person".to_string());
    }

    #[test]
    fn test_parse_column_names() {
        let (_, parsed) =
            parse_column_names(LocatedSpan::new("(CustomerName, ContactName, Address)")).unwrap();

        assert_eq!(
            parsed,
            vec![
                "CustomerName".to_string(),
                "ContactName".to_string(),
                "Address".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_column_names_with_leading_and_trailing_spaces() {
        let (_, parsed) =
            parse_column_names(LocatedSpan::new("( CustomerName, ContactName, Address )")).unwrap();

        assert_eq!(
            parsed,
            vec![
                "CustomerName".to_string(),
                "ContactName".to_string(),
                "Address".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_column_values() {
        let (_, parsed) = parse_column_values(LocatedSpan::new(
            "VALUES ( \"CustomerName\", \"ContactName\", \"Address\" )",
        ))
        .unwrap();

        assert_eq!(
            parsed,
            vec![
                "CustomerName".to_string(),
                "ContactName".to_string(),
                "Address".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_insert_statement_with_column_names() {
        let (_, statement) = InsertStatement::parse_from_raw(
            "INSERT INTO Customers (CustomerName, ContactName, Address) VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21');",
        )
        .unwrap();

        assert_eq!(
            statement,
            InsertStatement {
                table: "Customers".to_string(),
                columns: vec![
                    "CustomerName".to_string(),
                    "ContactName".to_string(),
                    "Address".to_string()
                ],
                values: vec![
                    "Cardinal".to_string(),
                    "Tom B. Erichsen".to_string(),
                    "Skagen 21".to_string(),
                ]
            }
        )
    }

    #[test]
    fn test_parse_insert_statement_without_column_names() {
        let (_, statement) = InsertStatement::parse_from_raw(
            "INSERT INTO Customers VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21');",
        )
        .unwrap();

        assert_eq!(
            statement,
            InsertStatement {
                table: "Customers".to_string(),
                columns: vec![],
                values: vec![
                    "Cardinal".to_string(),
                    "Tom B. Erichsen".to_string(),
                    "Skagen 21".to_string(),
                ]
            }
        )
    }
}
