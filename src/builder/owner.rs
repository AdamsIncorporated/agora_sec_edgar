use crate::error::EDGARParserError;
use phf::{Map, phf_map};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum OwnerOptions {
    /// "include" means include all documents regardless of the source.
    INCLUDE,
    /// "exclude" means exclude documents related to the company's director or officer ownership.
    EXCLUDE,
    /// "only" means only show documents related to the company's director or officer ownership.
    ONLY,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(missing_docs)]
pub enum OwnerOption {
    INCLUDE,
    EXCLUDE,
    ONLY,
}

// Static map for string -> enum conversion
// Adapted from: https://github.com/tieje/rs_sec_edgar
// Original Author: Thomas James Francis
// License: MIT
static OWNER_TYPE_MAP: Map<&'static str, OwnerOption> = phf_map! {
    "include" => OwnerOption::INCLUDE,
    "exclude" => OwnerOption::EXCLUDE,
    "only" => OwnerOption::ONLY,
};

impl FromStr for OwnerOption {
    type Err = EDGARParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OWNER_TYPE_MAP
            .get(&s.to_uppercase() as &str)
            .copied()
            .ok_or(EDGARParserError::FilingTypeNotFound())
    }
}

impl fmt::Display for OwnerOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            OwnerOption::INCLUDE => "include".to_string(),
            OwnerOption::EXCLUDE => "exclude".to_string(),
            OwnerOption::ONLY => "only".to_string(),
        };
        write!(f, "{}", value)
    }
}

// Re-exported helpers for your API
pub fn filing_from_str(s: &str) -> Result<OwnerOption, EDGARParserError> {
    OwnerOption::from_str(s)
}

pub fn to_string(filing_type: OwnerOption) -> String {
    filing_type.to_string()
}

pub fn validate_filing_type_string(s: &str) -> Result<String, EDGARParserError> {
    let ft = filing_from_str(s)?;
    Ok(ft.to_string())
}
