use serde::Deserialize;
use crate::edgar::EdgarParser;
use crate::error::EDGARParserError;

#[derive(Debug, Deserialize, PartialEq)]
struct EdgarQueryBuilder {
    edgar_parser: EdgarParser,  
}