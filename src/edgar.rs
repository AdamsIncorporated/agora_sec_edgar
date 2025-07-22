use crate::api::get_http_response_body;
use crate::error::EDGARParserError;
use serde::Deserialize;

/// Represents a company record with CIK, ticker, title, and a zero-padded CIK string.
///
/// # Fields
/// - `cik_str`: The raw Central Index Key (CIK) number.
/// - `ticker`: The stock ticker symbol.
/// - `title`: Company name/title.
/// - `leading_zero_cik`: Zero-padded string version of `cik_str`, exactly 10 digits.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EdgarParser {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String,

    #[serde(deserialize_with = "pad_cik")]
    pub leading_zero_cik: String,
}

/// Custom deserializer to convert a `u32` CIK into a zero-padded 10-digit string.
fn pad_cik<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let cik: u32 = Deserialize::deserialize(deserializer)?;
    Ok(format!("{:010}", cik))
}

/// Represents raw company data from the SEC without a padded CIK.
#[derive(Debug, Deserialize, PartialEq)]
pub struct CompanyData {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String,
}

/// Wrapper struct for parsing a list of companies from SEC JSON.
#[derive(Debug, Deserialize, PartialEq)]
pub struct CompanyDataList {
    pub tickers: Vec<CompanyData>,
}

impl EdgarParser {
    /// Creates a new `EdgarParser` by querying the SEC company list for a given ticker symbol.
    ///
    /// # Arguments
    /// - `ticker`: The ticker symbol to look up (e.g., "AAPL").
    ///
    /// # Errors
    /// Returns `EDGARParserError::HttpError`, `EDGARParserError::JSONParseError`, or `EDGARParserError::NotFound`
    pub fn new(self, ticker: &str) -> Result<Self, EDGARParserError> {
        let edgar_parser = self.create_from_ticker(ticker)?;
        Ok(edgar_parser)
    }

    /// Internal helper to create an `EdgarParser` by searching the ticker list.
    pub fn create_from_ticker(&self, ticker: &str) -> Result<EdgarParser, EDGARParserError> {
        let json_body = get_http_response_body("www.sec.gov", "/files/company_tickers.json")
            .map_err(EDGARParserError::HttpError)?;

        let tickers: CompanyDataList =
            serde_json::from_str(&json_body).map_err(EDGARParserError::JSONParseError)?;

        tickers
            .tickers
            .iter()
            .find(|&c| c.ticker == ticker)
            .map(|c| EdgarParser {
                cik_str: c.cik_str,
                ticker: c.ticker.clone(),
                title: c.title.clone(),
                leading_zero_cik: format!("{:010}", c.cik_str),
            })
            .ok_or_else(|| EDGARParserError::NotFound(format!("Ticker {} not found", ticker)))
    }

    /// Fetches the SEC Company Facts XBRL JSON for the current company.
    ///
    /// # Errors
    /// Returns `EDGARParserError::HttpError` or `EDGARParserError::JSONParseError` if the request fails.
    pub fn fetch_company_facts(&self) -> Result<serde_json::Value, EDGARParserError> {
        let body_response = get_http_response_body(
            "data.sec.gov",
            &format!("/api/xbrl/companyfacts/CIK{}.json", self.leading_zero_cik),
        )
        .map_err(EDGARParserError::HttpError)?;

        let json_response: serde_json::Value =
            serde_json::from_str(&body_response).map_err(EDGARParserError::JSONParseError)?;

        Ok(json_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_pad_cik_function() {
        let result = pad_cik(serde_json::json!(123456).into_deserializer()).unwrap();
        assert_eq!(result, "0000123456");
    }

    #[test]
    fn test_deserialize_edgar_parser_with_padding() {
        let json = r#"
            {
                "cik_str": 1045810,
                "ticker": "AAPL",
                "title": "Apple Inc.",
                "leading_zero_cik": 1045810
            }
        "#;

        let parsed: EdgarParser = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.leading_zero_cik, "0001045810");
    }

    #[test]
    fn test_company_data_list_deserialize() {
        let json = r#"
            {
                "tickers": [
                    {
                        "cik_str": 1045810,
                        "ticker": "AAPL",
                        "title": "Apple Inc."
                    },
                    {
                        "cik_str": 320193,
                        "ticker": "MSFT",
                        "title": "Microsoft Corp"
                    }
                ]
            }
        "#;

        let list: CompanyDataList = serde_json::from_str(json).unwrap();
        assert_eq!(list.tickers.len(), 2);
        assert_eq!(list.tickers[0].ticker, "AAPL");
    }

    // Example test using mock (requires dependency injection refactor to be fully testable)
    #[test]
    fn test_create_from_ticker_mocked() {
        let dummy = EdgarParser {
            cik_str: 0,
            ticker: "".to_string(),
            title: "".to_string(),
            leading_zero_cik: "".to_string(),
        };

        // You'd need to allow injecting `get_http_response_body` for a real unit test.
        // This is more of a design suggestion reminder than executable code.
        assert!(dummy.create_from_ticker("AAPL").is_err());
    }
}
