use crate::api::fetch_http_body_over_tcp;
use crate::edgar::EdgarParser;
use crate::error::EDGARParserError;
use crate::filing_type_builder::filing::FilingTypeOption;
use crate::filing_type_builder::owner::OwnerOption;
use chrono::NaiveDate;
use url::Url;

/// `EdgarFilingQueryBuilder` is a builder struct to construct a URL query for the SEC's EDGAR system.
#[derive(Debug, PartialEq)]
pub struct EdgarFilingQueryBuilder {
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

impl EdgarFilingQueryBuilder {
    /// Constructs a new instance of `EdgarFilingQueryBuilder` with default values and a provided `EdgarParser`.
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

    pub async fn fetch_filing_type(&self) -> Result<String, EDGARParserError> {
        let url = self.build()?;
        let url_string = url.to_string();
        let body = fetch_http_body_over_tcp(&url_string).await?;
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edgar::EdgarParser;
    use crate::error::EDGARParserError;
    use crate::filing_type_builder::filing::FilingTypeOption;
    use crate::filing_type_builder::owner::OwnerOption;

    async fn sample_parser() -> Result<EdgarParser, EDGARParserError> {
        Ok(EdgarParser::create_from_ticker("AAPL").await?)
    }

    #[tokio::test]
    async fn test_new_builder_defaults() {
        let parser = sample_parser().await.unwrap();
        let builder = EdgarFilingQueryBuilder::new(parser);
        let cik_raw_num = builder.edgar_parser.cik_str;

        assert_eq!(cik_raw_num, 320193);
        assert_eq!(
            builder.base_url,
            "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&"
        );
        assert_eq!(builder.count, "10");
        assert_eq!(builder.dateb, "");
        assert_eq!(builder.search_text, "");
    }

    #[test]
    fn test_set_and_validate_dateb_valid() {
        let date = "20240101".to_string();
        let result = EdgarFilingQueryBuilder::set_and_validate_dateb(date.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), date);
    }

    #[test]
    fn test_set_and_validate_dateb_invalid_format() {
        let bad_date = "2024011".to_string(); // only 7 digits
        let err = EdgarFilingQueryBuilder::set_and_validate_dateb(bad_date.clone()).unwrap_err();
        assert!(matches!(err, EDGARParserError::InvalidDateFormat(d) if d == bad_date));
    }

    #[test]
    fn test_set_and_validate_dateb_invalid_date() {
        let bad_date = "20241301".to_string(); // month 13
        let err = EdgarFilingQueryBuilder::set_and_validate_dateb(bad_date.clone()).unwrap_err();
        assert!(matches!(err, EDGARParserError::InvalidDateFormat(d) if d == bad_date));
    }

    #[tokio::test]
    async fn test_build_url_success() {
        let parser = sample_parser().await.unwrap();
        let mut builder = EdgarFilingQueryBuilder::new(parser);
        builder.filing_type = FilingTypeOption::_10K;
        builder.owner = OwnerOption::INCLUDE;
        builder.dateb = "20231231".to_string();
        builder.count = "25".to_string();
        builder.search_text = "apple".to_string();

        let url = builder.build().unwrap();
        let url_str = url.as_str();

        assert!(url_str.contains("CIK=0000320193"));
        assert!(url_str.contains("type=10-K"));
        assert!(url_str.contains("dateb=20231231"));
        assert!(url_str.contains("owner=include"));
        assert!(url_str.contains("count=25"));
        assert!(url_str.contains("search_text=apple"));
    }

    #[tokio::test]
    async fn test_build_url_invalid_date() {
        let parser = sample_parser().await.unwrap();
        let mut builder = EdgarFilingQueryBuilder::new(parser);
        builder.dateb = "20231301".to_string(); // invalid month

        let result = builder.build();
        assert!(matches!(
            result,
            Err(EDGARParserError::InvalidDateFormat(_))
        ));
    }

    #[tokio::test]
    async fn test_fetch_filing_type_invalid_url() {
        let parser = sample_parser().await.unwrap();
        let mut builder = EdgarFilingQueryBuilder::new(parser);
        builder.dateb = "invalid".to_string(); // Will cause build() to fail

        let result = builder.fetch_filing_type().await;
        assert!(matches!(
            result,
            Err(EDGARParserError::InvalidDateFormat(_))
        ));
    }

    // You can optionally test real fetches with `#[ignore]`
    // Run with: `cargo test -- --ignored`
    #[tokio::test]
    #[ignore]
    async fn test_fetch_filing_type_real() {
        let parser = sample_parser().await.unwrap();
        let mut builder = EdgarFilingQueryBuilder::new(parser);
        builder.dateb = "20231231".to_string();
        builder.filing_type = FilingTypeOption::_10K;
        builder.owner = OwnerOption::INCLUDE;

        let result = builder.fetch_filing_type().await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("entry")); // Atom XML entries
    }
}
