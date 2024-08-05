use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    error::context,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};

use crate::{
    create_statement::CreateStatement, insert_statement::InsertStatement,
    select_statement::SelectStatement, types::Parse,
};

/// All possible commands
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    Insert(InsertStatement),
    Create(CreateStatement),
}

impl<'a> Parse<'a> for SqlQuery {
    fn parse(input: crate::types::RawSpan<'a>) -> crate::types::ParserResult<'a, Self> {
        let (rest, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                tuple((
                    alt((
                        map(SelectStatement::parse, SqlQuery::Select),
                        map(InsertStatement::parse, SqlQuery::Insert),
                        map(CreateStatement::parse, SqlQuery::Create),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                )),
            ),
        )(input)?;

        Ok((rest, query))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_select_query() {
        let (_, query) =
            SqlQuery::parse_from_raw("SELECT CustomerName, City FROM Customers;").unwrap();

        assert_eq!(
            query,
            SqlQuery::Select(
                SelectStatement::parse_from_raw("SELECT CustomerName, City FROM Customers;")
                    .unwrap()
                    .1
            )
        )
    }

    #[test]
    fn test_parse_insert_query() {
        let raw_query =
            "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country)
VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway');";

        let (_, query) = SqlQuery::parse_from_raw(&raw_query).unwrap();

        assert_eq!(
            query,
            SqlQuery::Insert(InsertStatement::parse_from_raw(raw_query).unwrap().1)
        )
    }



    #[test]
    fn test_parse_create_query() {
        let raw_query =
            "CREATE TABLE Persons (PersonID int, 
            LastName string);";

        let (_, query) = SqlQuery::parse_from_raw(&raw_query).unwrap();

        assert_eq!(
            query,
            SqlQuery::Create(CreateStatement::parse_from_raw(raw_query).unwrap().1)
        )
    }
}
