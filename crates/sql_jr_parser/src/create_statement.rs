use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::common_parsers::identifier;
use crate::types::{Parse, ParserResult, RawSpan};

/// A colum's type
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SqlTypeInfo {
    // these are basic for now. Will add more + size max later on
    String,
    Int,
}

impl<'a> Parse<'a> for SqlTypeInfo {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self> {
        context(
            "Column Type",
            // alt will try each passed parser and return what ever succeeds
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        )(input)
    }
}

/// A column's name + type
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub type_info: SqlTypeInfo,
}

// parses "<colName> <colType>"
impl<'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self> {
        context(
            "Create Column",
            map(
                separated_pair(
                    identifier.context("Column Name"),
                    multispace1,
                    SqlTypeInfo::parse,
                ),
                |(name, type_info)| Self { name, type_info },
            ),
        )(input)
    }
}

/// The table and its columns to create
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

// parses a comma seperated list of column definitions contained in parens
fn column_definitions(input: RawSpan<'_>) -> ParserResult<'_, Vec<Column>> {
    context(
        "Column Definitions",
        map(
            tuple((
                char('('),
                separated_list1(tuple((multispace0, char(','), multispace0)), Column::parse),
                char(')'),
            )),
            |(_, cols, _)| cols,
        ),
    )(input)
}

impl<'a> Parse<'a> for CreateStatement {
    fn parse(input: RawSpan<'a>) -> ParserResult<'a, Self> {
        map(
            separated_pair(
                preceded(
                    tuple((
                        tag_no_case("create"),
                        multispace1,
                        tag_no_case("table"),
                        multispace1,
                    )),
                    identifier.context("Table Name"),
                ),
                multispace1,
                column_definitions,
            )
            .context("Create Table"),
            |(table, columns)| Self { table, columns },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use nom_locate::LocatedSpan;

    use super::*;

    #[test]
    fn test_parse_sql_type_info_int() {
        let (_, type_info) = SqlTypeInfo::parse(LocatedSpan::new("int")).unwrap();

        assert_eq!(type_info, SqlTypeInfo::Int);
    }

    #[test]
    fn test_parse_sql_type_info_string() {
        let (_, type_info) = SqlTypeInfo::parse(LocatedSpan::new("string")).unwrap();

        assert_eq!(type_info, SqlTypeInfo::String);
    }

    #[test]
    fn test_parse_sql_column_of_type_int() {
        let (_, column) = Column::parse_from_raw("age int,").unwrap();

        assert_eq!(
            column,
            Column {
                name: "age".to_string(),
                type_info: SqlTypeInfo::Int
            }
        );
    }

    #[test]
    fn test_parse_sql_column_of_type_string() {
        let (_, column) = Column::parse_from_raw("address string,").unwrap();

        assert_eq!(
            column,
            Column {
                name: "address".to_string(),
                type_info: SqlTypeInfo::String
            }
        );
    }

    #[test]
    fn test_parse_sql_columns_definitions() {
        let (_, column) =
            column_definitions(LocatedSpan::new("(address string, age int)")).unwrap();

        assert_eq!(
            column,
            vec![
                Column {
                    name: "address".to_string(),
                    type_info: SqlTypeInfo::String
                },
                Column {
                    name: "age".to_string(),
                    type_info: SqlTypeInfo::Int
                }
            ]
        );
    }

    #[test]
    fn test_parse_create_statement() {
        let (_, statement) =
            CreateStatement::parse_from_raw("create table Person (name string, age int)").unwrap();

        assert_eq!(
            statement,
            CreateStatement {
                table: "Person".to_string(),
                columns: vec![
                    Column {
                        name: "name".to_string(),
                        type_info: SqlTypeInfo::String
                    },
                    Column {
                        name: "age".to_string(),
                        type_info: SqlTypeInfo::Int
                    }
                ]
            }
        )
    }
}
