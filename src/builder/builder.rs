// Import required modules and types from the project and external crates.
use crate::builder::filing::FilingTypeOption;
use crate::builder::owner::OwnerOption;
use crate::edgar::EdgarParser;
use crate::error::EDGARParserError;
use chrono::NaiveDate;
use url::Url;

/// `EdgarQueryBuilder` is a builder struct to construct a URL query for the SEC's EDGAR system.
#[derive(Debug, PartialEq)]
pub struct EdgarQueryBuilder {
    // Instance of EdgarParser that provides the CIK and potentially other metadata.
    pub edgar_parser: EdgarParser,

    // Type of filing to search for (e.g., 10-K, 8-K).
    pub filing_type: FilingTypeOption,

    // Date to search filings before, in the format YYYYMMDD.
    pub dateb: String,

    // Ownership option (e.g., include or exclude insider ownership).
    pub owner: OwnerOption,

    // Base URL for EDGAR search.
    pub base_url: String,

    // Number of filings to fetch.
    pub count: String,

    // Optional text to filter search results.
    pub search_text: String,
}

impl EdgarQueryBuilder {
    /// Constructs a new instance of `EdgarQueryBuilder` with default values and a provided `EdgarParser`.
    pub fn new(edgar_parser: EdgarParser) -> Self {
        Self {
            base_url: "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string(),
            filing_type: Default::default(),
            dateb: Default::default(),
            owner: Default::default(),
            count: "10".to_string(),
            search_text: Default::default(),
            edgar_parser,
        } 
    }

    /// Builds and returns a `Url` to query the EDGAR system based on the builder's state.
    /// Returns an error if any component is invalid (e.g., date format or URL parsing fails).
    pub fn build(&self) -> Result<Url, EDGARParserError> {
        // Convert enums to string representations.
        let filing_type_string = self.filing_type.to_string();
        let owner_string = self.owner.to_string();

        // Validate and extract the date string.
        let dateb_string = Self::set_and_validate_dateb(self.dateb.clone())?;

        // Format the full URL string with all parameters.
        let url = format!(
            "{base}CIK={cik}&type={filing_type_string}&dateb={dateb_string}&owner={owner_string}&count={count}&search_text={search_text}&output=atom",
            base = self.base_url,
            cik = self.edgar_parser.cik_str,
            filing_type_string = filing_type_string,
            dateb_string = dateb_string,
            owner_string = owner_string,
            count = self.count,
            search_text = self.search_text
        );

        // Parse the constructed string into a `Url` object.
        let query = Url::parse(&url)?;
        Ok(query)
    }

    /// Validates the `dateb` string to ensure it is exactly 8 digits and forms a valid date (YYYYMMDD).
    /// Returns the valid date string or an `EDGARParserError` if invalid.
    fn set_and_validate_dateb(dateb: String) -> Result<String, EDGARParserError> {
        // Check if the string has exactly 8 numeric characters.
        if dateb.len() != 8 || !dateb.chars().all(|f| f.is_digit(10)) {
            return Err(EDGARParserError::InvalidDateFormat(dateb));
        } else {
            // Attempt to parse the string as a date.
            match NaiveDate::parse_from_str(&dateb, "%Y%m%d") {
                Ok(_) => Ok(dateb),
                Err(_) => Err(EDGARParserError::InvalidDateFormat(dateb)),
            }
        }
    }
}
