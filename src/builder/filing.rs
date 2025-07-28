use crate::error::EDGARParserError;
use phf::{Map, phf_map};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(missing_docs)]
pub enum FilingTypeOption {
    _1A,
    _1E,
    _1K,
    _1N,
    _1SA,
    _1U,
    _1Z,
    _10,
    _10D,
    _10K,
    _10M,
    _10Q,
    _11K,
    _12B25,
    _13F,
    _13H,
    _144,
    _15,
    _15F,
    _17H,
    _18,
    _18K,
    _19B4,
    _19B4E,
    _19B7,
    _2E,
    _20F,
    _24F2,
    _25,
    _3,
    _4,
    _40F,
    _5,
    _6K,
    _7M,
    _8A,
    _8K,
    _8M,
    _9M,
    ABS,
    ABS15G,
    ABSEE,
    ADV,
    ADVE,
    ADVH,
    ADVNR,
    ADVW,
    ATS,
    ATSN,
    ATSR,
    BD,
    BDN,
    BDW,
    C,
    CA1,
    CB,
    CFPORTAL,
    CRS,
    CUSTODY,
    D,
    F1,
    F10,
    F3,
    F4,
    F6,
    F7,
    F8,
    F80,
    FN,
    FX,
    ID,
    MA,
    MAI,
    MANR,
    MAW,
    MSD,
    MSDW,
    N14,
    N17D1,
    N17F1,
    N17F2,
    N18F1,
    N1A,
    N2,
    N23C3,
    N27D1,
    N3,
    N4,
    N5,
    N54A,
    N54C,
    N6,
    N6EI1,
    N6F,
    N8A,
    N8B2,
    N8B4,
    N8F,
    NCEN,
    NCR,
    NCSR,
    NMFP,
    NPORT,
    NPX,
    NQ,
    NRN,
    NRSRO,
    PF,
    PILOT,
    R31,
    S1,
    S11,
    S20,
    S3,
    S4,
    S6,
    S8,
    SBSE,
    SBSEA,
    SBSEBD,
    SBSEC,
    SBSEW,
    SCI,
    SD,
    SDR,
    SE,
    SF1,
    SF3,
    SIP,
    T1,
    T2,
    T3,
    T4,
    T6,
    TA1,
    TA2,
    TAW,
    TCR,
    TH,
    WBAPP,
    X17A19,
    X17A5,
    X17F1A,
}

// Static map for string -> enum conversion
// Adapted from: https://github.com/tieje/rs_sec_edgar
// Original Author: Thomas James Francis
// License: MIT
static FILING_TYPE_MAP: Map<&'static str, FilingTypeOption> = phf_map! {
        "1-A" => FilingTypeOption::_1A ,
        "1-E" => FilingTypeOption::_1E ,
        "1-K" => FilingTypeOption::_1K ,
        "1-N" => FilingTypeOption::_1N ,
        "1-SA" => FilingTypeOption::_1SA ,
        "1-U" => FilingTypeOption::_1U ,
        "1-Z" => FilingTypeOption::_1Z ,
        "10" => FilingTypeOption::_10 ,
        "10-D" => FilingTypeOption::_10D ,
        "10-K" => FilingTypeOption::_10K ,
        "10-M" => FilingTypeOption::_10M ,
        "10-Q" => FilingTypeOption::_10Q ,
        "11-K" => FilingTypeOption::_11K ,
        "12B-25" => FilingTypeOption::_12B25 ,
        "13F" => FilingTypeOption::_13F ,
        "13H" => FilingTypeOption::_13H ,
        "144" => FilingTypeOption::_144 ,
        "15" => FilingTypeOption::_15 ,
        "15F" => FilingTypeOption::_15F ,
        "17-H" => FilingTypeOption::_17H ,
        "18" => FilingTypeOption::_18 ,
        "18-K" => FilingTypeOption::_18K ,
        "19b-4" => FilingTypeOption::_19B4 ,
        "19b-4(E)" => FilingTypeOption::_19B4E ,
        "19b-7" => FilingTypeOption::_19B7 ,
        "2-E" => FilingTypeOption::_2E ,
        "20-F" => FilingTypeOption::_20F ,
        "24F-2" => FilingTypeOption::_24F2 ,
        "25" => FilingTypeOption::_25 ,
        "3" => FilingTypeOption::_3 ,
        "4" => FilingTypeOption::_4 ,
        "40-F" => FilingTypeOption::_40F ,
        "5" => FilingTypeOption::_5 ,
        "6-K" => FilingTypeOption::_6K ,
        "7-M" => FilingTypeOption::_7M ,
        "8-A" => FilingTypeOption::_8A ,
        "8-K" => FilingTypeOption::_8K ,
        "8-M" => FilingTypeOption::_8M ,
        "9-M" => FilingTypeOption::_9M ,
        "ABS" => FilingTypeOption::ABS ,
        "ABS-15G" => FilingTypeOption::ABS15G ,
        "ABS-EE" => FilingTypeOption::ABSEE ,
        "ADV" => FilingTypeOption::ADV ,
        "ADV-E" => FilingTypeOption::ADVE ,
        "ADV-H" => FilingTypeOption::ADVH ,
        "ADV-NR" => FilingTypeOption::ADVNR ,
        "ADV-W" => FilingTypeOption::ADVW ,
        "ATS" => FilingTypeOption::ATS ,
        "ATS-N" => FilingTypeOption::ATSN ,
        "ATS-R" => FilingTypeOption::ATSR ,
        "BD" => FilingTypeOption::BD ,
        "BD-N" => FilingTypeOption::BDN ,
        "BDW" => FilingTypeOption::BDW ,
        "C" => FilingTypeOption::C ,
        "CA-1" => FilingTypeOption::CA1 ,
        "CB" => FilingTypeOption::CB ,
        "CFPORTAL" => FilingTypeOption::CFPORTAL ,
        "CRS" => FilingTypeOption::CRS ,
        "CUSTODY" => FilingTypeOption::CUSTODY ,
        "D" => FilingTypeOption::D ,
        "F-1" => FilingTypeOption::F1 ,
        "F-10" => FilingTypeOption::F10 ,
        "F-3" => FilingTypeOption::F3 ,
        "F-4" => FilingTypeOption::F4 ,
        "F-6" => FilingTypeOption::F6 ,
        "F-7" => FilingTypeOption::F7 ,
        "F-8" => FilingTypeOption::F8 ,
        "F-80" => FilingTypeOption::F80 ,
        "F-N" => FilingTypeOption::FN ,
        "F-X" => FilingTypeOption::FX ,
        "ID" => FilingTypeOption::ID ,
        "MA" => FilingTypeOption::MA ,
        "MA-I" => FilingTypeOption::MAI ,
        "MA-NR" => FilingTypeOption::MANR ,
        "MA-W" => FilingTypeOption::MAW ,
        "MSD" => FilingTypeOption::MSD ,
        "MSDW" => FilingTypeOption::MSDW ,
        "N-14" => FilingTypeOption::N14 ,
        "N-17D-1" => FilingTypeOption::N17D1 ,
        "N-17F-1" => FilingTypeOption::N17F1 ,
        "N-17F-2" => FilingTypeOption::N17F2 ,
        "N-18F-1" => FilingTypeOption::N18F1 ,
        "N-1A" => FilingTypeOption::N1A ,
        "N-2" => FilingTypeOption::N2 ,
        "N-23C-3" => FilingTypeOption::N23C3 ,
        "N-27D-1" => FilingTypeOption::N27D1 ,
        "N-3" => FilingTypeOption::N3 ,
        "N-4" => FilingTypeOption::N4 ,
        "N-5" => FilingTypeOption::N5 ,
        "N-54A" => FilingTypeOption::N54A ,
        "N-54C" => FilingTypeOption::N54C ,
        "N-6" => FilingTypeOption::N6 ,
        "N-6EI-1" => FilingTypeOption::N6EI1 ,
        "N-6F" => FilingTypeOption::N6F ,
        "N-8A" => FilingTypeOption::N8A ,
        "N-8B-2" => FilingTypeOption::N8B2 ,
        "N-8B-4" => FilingTypeOption::N8B4 ,
        "N-8F" => FilingTypeOption::N8F ,
        "N-CEN" => FilingTypeOption::NCEN ,
        "N-CR" => FilingTypeOption::NCR ,
        "N-CSR" => FilingTypeOption::NCSR ,
        "N-MFP" => FilingTypeOption::NMFP ,
        "N-PORT" => FilingTypeOption::NPORT ,
        "N-PX" => FilingTypeOption::NPX ,
        "N-Q" => FilingTypeOption::NQ ,
        "N-RN" => FilingTypeOption::NRN ,
        "NRSRO" => FilingTypeOption::NRSRO ,
        "PF" => FilingTypeOption::PF ,
        "PILOT" => FilingTypeOption::PILOT ,
        "R31" => FilingTypeOption::R31 ,
        "S-1" => FilingTypeOption::S1 ,
        "S-11" => FilingTypeOption::S11 ,
        "S-20" => FilingTypeOption::S20 ,
        "S-3" => FilingTypeOption::S3 ,
        "S-4" => FilingTypeOption::S4 ,
        "S-6" => FilingTypeOption::S6 ,
        "S-8" => FilingTypeOption::S8 ,
        "SBSE" => FilingTypeOption::SBSE ,
        "SBSE-A" => FilingTypeOption::SBSEA ,
        "SBSE-BD" => FilingTypeOption::SBSEBD ,
        "SBSE-C" => FilingTypeOption::SBSEC ,
        "SBSE-W" => FilingTypeOption::SBSEW ,
        "SCI" => FilingTypeOption::SCI ,
        "SD" => FilingTypeOption::SD ,
        "SDR" => FilingTypeOption::SDR ,
        "SE" => FilingTypeOption::SE ,
        "SF-1" => FilingTypeOption::SF1 ,
        "SF-3" => FilingTypeOption::SF3 ,
        "SIP" => FilingTypeOption::SIP ,
        "T-1" => FilingTypeOption::T1 ,
        "T-2" => FilingTypeOption::T2 ,
        "T-3" => FilingTypeOption::T3 ,
        "T-4" => FilingTypeOption::T4 ,
        "T-6" => FilingTypeOption::T6 ,
        "TA-1" => FilingTypeOption::TA1 ,
        "TA-2" => FilingTypeOption::TA2 ,
        "TA-W" => FilingTypeOption::TAW ,
        "TCR" => FilingTypeOption::TCR ,
        "TH" => FilingTypeOption::TH ,
        "WB-APP" => FilingTypeOption::WBAPP ,
        "X-17A-19" => FilingTypeOption::X17A19 ,
        "X-17A-5" => FilingTypeOption::X17A5 ,
        "X-17F-1A" => FilingTypeOption::X17F1A ,
};

impl FromStr for FilingTypeOption {
    type Err = EDGARParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FILING_TYPE_MAP
            .get(&s.to_uppercase() as &str)
            .copied()
            .ok_or(EDGARParserError::FilingTypeNotFound())
    }
}

impl Default for FilingTypeOption {
    fn default() -> Self {
        FilingTypeOption::_1U
    }
}

impl fmt::Display for FilingTypeOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            FilingTypeOption::_1A => "1-A".to_string(),
            FilingTypeOption::_1E => "1-E".to_string(),
            FilingTypeOption::_1K => "1-K".to_string(),
            FilingTypeOption::_1N => "1-N".to_string(),
            FilingTypeOption::_1SA => "1-SA".to_string(),
            FilingTypeOption::_1U => "1-U".to_string(),
            FilingTypeOption::_1Z => "1-Z".to_string(),
            FilingTypeOption::_10 => "10".to_string(),
            FilingTypeOption::_10D => "10-D".to_string(),
            FilingTypeOption::_10K => "10-K".to_string(),
            FilingTypeOption::_10M => "10-M".to_string(),
            FilingTypeOption::_10Q => "10-Q".to_string(),
            FilingTypeOption::_11K => "11-K".to_string(),
            FilingTypeOption::_12B25 => "12B-25".to_string(),
            FilingTypeOption::_13F => "13F".to_string(),
            FilingTypeOption::_13H => "13H".to_string(),
            FilingTypeOption::_144 => "144".to_string(),
            FilingTypeOption::_15 => "15".to_string(),
            FilingTypeOption::_15F => "15F".to_string(),
            FilingTypeOption::_17H => "17-H".to_string(),
            FilingTypeOption::_18 => "18".to_string(),
            FilingTypeOption::_18K => "18-K".to_string(),
            FilingTypeOption::_19B4 => "19b-4".to_string(),
            FilingTypeOption::_19B4E => "19b-4(E)".to_string(),
            FilingTypeOption::_19B7 => "19b-7".to_string(),
            FilingTypeOption::_2E => "2-E".to_string(),
            FilingTypeOption::_20F => "20-F".to_string(),
            FilingTypeOption::_24F2 => "24F-2".to_string(),
            FilingTypeOption::_25 => "25".to_string(),
            FilingTypeOption::_3 => "3".to_string(),
            FilingTypeOption::_4 => "4".to_string(),
            FilingTypeOption::_40F => "40-F".to_string(),
            FilingTypeOption::_5 => "5".to_string(),
            FilingTypeOption::_6K => "6-K".to_string(),
            FilingTypeOption::_7M => "7-M".to_string(),
            FilingTypeOption::_8A => "8-A".to_string(),
            FilingTypeOption::_8K => "8-K".to_string(),
            FilingTypeOption::_8M => "8-M".to_string(),
            FilingTypeOption::_9M => "9-M".to_string(),
            FilingTypeOption::ABS => "ABS".to_string(),
            FilingTypeOption::ABS15G => "ABS-15G".to_string(),
            FilingTypeOption::ABSEE => "ABS-EE".to_string(),
            FilingTypeOption::ADV => "ADV".to_string(),
            FilingTypeOption::ADVE => "ADV-E".to_string(),
            FilingTypeOption::ADVH => "ADV-H".to_string(),
            FilingTypeOption::ADVNR => "ADV-NR".to_string(),
            FilingTypeOption::ADVW => "ADV-W".to_string(),
            FilingTypeOption::ATS => "ATS".to_string(),
            FilingTypeOption::ATSN => "ATS-N".to_string(),
            FilingTypeOption::ATSR => "ATS-R".to_string(),
            FilingTypeOption::BD => "BD".to_string(),
            FilingTypeOption::BDN => "BD-N".to_string(),
            FilingTypeOption::BDW => "BDW".to_string(),
            FilingTypeOption::C => "C".to_string(),
            FilingTypeOption::CA1 => "CA-1".to_string(),
            FilingTypeOption::CB => "CB".to_string(),
            FilingTypeOption::CFPORTAL => "CFPORTAL".to_string(),
            FilingTypeOption::CRS => "CRS".to_string(),
            FilingTypeOption::CUSTODY => "CUSTODY".to_string(),
            FilingTypeOption::D => "D".to_string(),
            FilingTypeOption::F1 => "F-1".to_string(),
            FilingTypeOption::F10 => "F-10".to_string(),
            FilingTypeOption::F3 => "F-3".to_string(),
            FilingTypeOption::F4 => "F-4".to_string(),
            FilingTypeOption::F6 => "F-6".to_string(),
            FilingTypeOption::F7 => "F-7".to_string(),
            FilingTypeOption::F8 => "F-8".to_string(),
            FilingTypeOption::F80 => "F-80".to_string(),
            FilingTypeOption::FN => "F-N".to_string(),
            FilingTypeOption::FX => "F-X".to_string(),
            FilingTypeOption::ID => "ID".to_string(),
            FilingTypeOption::MA => "MA".to_string(),
            FilingTypeOption::MAI => "MA-I".to_string(),
            FilingTypeOption::MANR => "MA-NR".to_string(),
            FilingTypeOption::MAW => "MA-W".to_string(),
            FilingTypeOption::MSD => "MSD".to_string(),
            FilingTypeOption::MSDW => "MSDW".to_string(),
            FilingTypeOption::N14 => "N-14".to_string(),
            FilingTypeOption::N17D1 => "N-17D-1".to_string(),
            FilingTypeOption::N17F1 => "N-17F-1".to_string(),
            FilingTypeOption::N17F2 => "N-17F-2".to_string(),
            FilingTypeOption::N18F1 => "N-18F-1".to_string(),
            FilingTypeOption::N1A => "N-1A".to_string(),
            FilingTypeOption::N2 => "N-2".to_string(),
            FilingTypeOption::N23C3 => "N-23C-3".to_string(),
            FilingTypeOption::N27D1 => "N-27D-1".to_string(),
            FilingTypeOption::N3 => "N-3".to_string(),
            FilingTypeOption::N4 => "N-4".to_string(),
            FilingTypeOption::N5 => "N-5".to_string(),
            FilingTypeOption::N54A => "N-54A".to_string(),
            FilingTypeOption::N54C => "N-54C".to_string(),
            FilingTypeOption::N6 => "N-6".to_string(),
            FilingTypeOption::N6EI1 => "N-6EI-1".to_string(),
            FilingTypeOption::N6F => "N-6F".to_string(),
            FilingTypeOption::N8A => "N-8A".to_string(),
            FilingTypeOption::N8B2 => "N-8B-2".to_string(),
            FilingTypeOption::N8B4 => "N-8B-4".to_string(),
            FilingTypeOption::N8F => "N-8F".to_string(),
            FilingTypeOption::NCEN => "N-CEN".to_string(),
            FilingTypeOption::NCR => "N-CR".to_string(),
            FilingTypeOption::NCSR => "N-CSR".to_string(),
            FilingTypeOption::NMFP => "N-MFP".to_string(),
            FilingTypeOption::NPORT => "N-PORT".to_string(),
            FilingTypeOption::NPX => "N-PX".to_string(),
            FilingTypeOption::NQ => "N-Q".to_string(),
            FilingTypeOption::NRN => "N-RN".to_string(),
            FilingTypeOption::NRSRO => "NRSRO".to_string(),
            FilingTypeOption::PF => "PF".to_string(),
            FilingTypeOption::PILOT => "PILOT".to_string(),
            FilingTypeOption::R31 => "R31".to_string(),
            FilingTypeOption::S1 => "S-1".to_string(),
            FilingTypeOption::S11 => "S-11".to_string(),
            FilingTypeOption::S20 => "S-20".to_string(),
            FilingTypeOption::S3 => "S-3".to_string(),
            FilingTypeOption::S4 => "S-4".to_string(),
            FilingTypeOption::S6 => "S-6".to_string(),
            FilingTypeOption::S8 => "S-8".to_string(),
            FilingTypeOption::SBSE => "SBSE".to_string(),
            FilingTypeOption::SBSEA => "SBSE-A".to_string(),
            FilingTypeOption::SBSEBD => "SBSE-BD".to_string(),
            FilingTypeOption::SBSEC => "SBSE-C".to_string(),
            FilingTypeOption::SBSEW => "SBSE-W".to_string(),
            FilingTypeOption::SCI => "SCI".to_string(),
            FilingTypeOption::SD => "SD".to_string(),
            FilingTypeOption::SDR => "SDR".to_string(),
            FilingTypeOption::SE => "SE".to_string(),
            FilingTypeOption::SF1 => "SF-1".to_string(),
            FilingTypeOption::SF3 => "SF-3".to_string(),
            FilingTypeOption::SIP => "SIP".to_string(),
            FilingTypeOption::T1 => "T-1".to_string(),
            FilingTypeOption::T2 => "T-2".to_string(),
            FilingTypeOption::T3 => "T-3".to_string(),
            FilingTypeOption::T4 => "T-4".to_string(),
            FilingTypeOption::T6 => "T-6".to_string(),
            FilingTypeOption::TA1 => "TA-1".to_string(),
            FilingTypeOption::TA2 => "TA-2".to_string(),
            FilingTypeOption::TAW => "TA-W".to_string(),
            FilingTypeOption::TCR => "TCR".to_string(),
            FilingTypeOption::TH => "TH".to_string(),
            FilingTypeOption::WBAPP => "WB-APP".to_string(),
            FilingTypeOption::X17A19 => "X-17A-19".to_string(),
            FilingTypeOption::X17A5 => "X-17A-5".to_string(),
            FilingTypeOption::X17F1A => "X-17F-1A".to_string(),
        };
        write!(f, "{}", value)
    }
}

// Re-exported helpers for your API
pub fn filing_from_str(s: &str) -> Result<FilingTypeOption, EDGARParserError> {
    FilingTypeOption::from_str(s)
}

pub fn to_string(filing_type: FilingTypeOption) -> String {
    filing_type.to_string()
}

pub fn validate_filing_type_string(s: &str) -> Result<String, EDGARParserError> {
    let ft = filing_from_str(s)?;
    Ok(ft.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid() {
        let ft: FilingTypeOption = FilingTypeOption::from_str("10-K").unwrap();
        assert_eq!(ft, FilingTypeOption::_10K);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        let ft: FilingTypeOption = FilingTypeOption::from_str("10-k").unwrap();
        assert_eq!(ft, FilingTypeOption::_10K);
    }

    #[test]
    fn test_from_str_invalid() {
        let result: Result<FilingTypeOption, EDGARParserError> =
            FilingTypeOption::from_str("INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_display_trait() {
        let ft: FilingTypeOption = FilingTypeOption::_10K;
        assert_eq!(ft.to_string(), "10-K");
    }

    #[test]
    fn test_filing_from_str() {
        let ft: FilingTypeOption = filing_from_str("S-1").unwrap();
        assert_eq!(ft, FilingTypeOption::S1);
    }

    #[test]
    fn test_to_string_wrapper() {
        let s: String = to_string(FilingTypeOption::S1);
        assert_eq!(s, "S-1");
    }

    #[test]
    fn test_validate_filing_type_string_valid() {
        let s: String = validate_filing_type_string("S-1").unwrap();
        assert_eq!(s, "S-1");
    }

    #[test]
    fn test_validate_filing_type_string_invalid() {
        let s: Result<String, EDGARParserError> = validate_filing_type_string("WRONG-FORM");
        assert!(s.is_err());
    }
}
