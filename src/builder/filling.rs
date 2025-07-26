use std::str::FromStr;
use std::fmt;
use phf::phf_map;

use crate::error::EDGARError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(missing_docs)]
pub enum FilingTypeOption {
    _10K,
    _10Q,
    _8K,
    F1,
    F3,
    S1,
    S3,
    SF1,
    X17F1A,
    // Add the rest here
}

// Static map for string -> enum conversion
static FILING_TYPE_MAP: phf::Map<&'static str, FilingTypeOption> = phf_map! {
    "10-K" => FilingTypeOption::_10K,
    "10-Q" => FilingTypeOption::_10Q,
    "8-K"  => FilingTypeOption::_8K,
    "F-1"  => FilingTypeOption::F1,
    "F-3"  => FilingTypeOption::F3,
    "S-1"  => FilingTypeOption::S1,
    "S-3"  => FilingTypeOption::S3,
    "SF-1" => FilingTypeOption::SF1,
    "X-17F-1A" => FilingTypeOption::X17F1A,
    // Add more as needed...
};

impl FromStr for FilingTypeOption {
    type Err = EDGARError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FILING_TYPE_MAP
            .get(&s.to_uppercase() as &str)
            .copied()
            .ok_or(EDGARError::FilingTypeNotFound)
    }
}

impl fmt::Display for FilingTypeOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            FilingTypeOption::_10K => "10-K",
            FilingTypeOption::_10Q => "10-Q",
            FilingTypeOption::_8K  => "8-K",
            FilingTypeOption::F1   => "F-1",
            FilingTypeOption::F3   => "F-3",
            FilingTypeOption::S1   => "S-1",
            FilingTypeOption::S3   => "S-3",
            FilingTypeOption::SF1  => "SF-1",
            FilingTypeOption::X17F1A => "X-17F-1A",
        };
        write!(f, "{}", value)
    }
}

// Re-exported helpers for your API
pub fn filing_from_str(s: &str) -> Result<FilingTypeOption, EDGARError> {
    FilingTypeOption::from_str(s)
}

pub fn to_string(filing_type: FilingTypeOption) -> String {
    filing_type.to_string()
}

pub fn validate_filing_type_string(s: &str) -> Result<String, EDGARError> {
    let ft = filing_from_str(s)?;
    Ok(ft.to_string())
}
