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
            .ok_or(EDGARParserError::OwnerTypeNotFound())
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
pub fn owner_from_str(s: &str) -> Result<OwnerOption, EDGARParserError> {
    OwnerOption::from_str(s)
}

pub fn to_string(owner_type: OwnerOption) -> String {
    owner_type.to_string()
}

pub fn validate_owner_type_string(s: &str) -> Result<String, EDGARParserError> {
    let ft = owner_from_str(s)?;
    Ok(ft.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid() {
        let ft: OwnerOption = OwnerOption::from_str("EXCLUDE").unwrap();
        assert_eq!(ft, OwnerOption::EXCLUDE);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        let ft: OwnerOption = OwnerOption::from_str("exclude").unwrap();
        assert_eq!(ft, OwnerOption::EXCLUDE);
    }

    #[test]
    fn test_from_str_invalid() {
        let result: Result<OwnerOption, EDGARParserError> = OwnerOption::from_str("INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_display_trait() {
        let ft: OwnerOption = OwnerOption::INCLUDE;
        assert_eq!(ft.to_string(), "include");
    }

    #[test]
    fn test_owner_from_str() {
        let ft: OwnerOption = owner_from_str("EXCLUDE").unwrap();
        assert_eq!(ft, OwnerOption::EXCLUDE);
    }

    #[test]
    fn test_to_string_wrapper() {
        let s: String = to_string(OwnerOption::EXCLUDE);
        assert_eq!(s, "EXCLUDE");
    }

    #[test]
    fn test_validate_owner_type_string_valid() {
        let s: String = validate_owner_type_string("EXCLUDE").unwrap();
        assert_eq!(s, "EXCLUDE");
    }

    #[test]
    fn test_validate_owner_type_string_invalid() {
        let s: Result<String, EDGARParserError> = validate_owner_type_string("WRONG-OWNER");
        assert!(s.is_err());
    }
}
