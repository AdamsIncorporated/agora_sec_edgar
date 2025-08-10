use crate::api::fetch_http_body;
use crate::error::EDGARParserError;
use serde::Deserialize;
use std::collections::HashMap;

/// Represents a company record with CIK, ticker, title, and a zero-padded CIK string.
///
/// # Fields
/// - `cik_str`: The raw Central Index Key (CIK) number.
/// - `ticker`: The stock ticker symbol.
/// - `title`: Company name/title.
/// - `leading_zero_cik`: Zero-padded string version of `cik_str`, exactly 10 digits.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EdgarParser {
    pub cik_str: Option<u32>,
    pub ticker: Option<String>,
    pub title: Option<String>,
    pub submissions: Option<serde_json::Value>,
    pub company_facts: Option<serde_json::Value>,

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

impl EdgarParser {
    /// Creates a new `EdgarParser` by querying the SEC company list for a given ticker symbol.
    ///
    /// # Arguments
    /// - `ticker`: The ticker symbol to look up (e.g., "AAPL").
    ///
    /// # Errors
    /// Returns `EDGARParserError::HttpError`, `EDGARParserError::JSONParseError`, or `EDGARParserError::NotFound`
    pub async fn new(ticker: &str) -> Result<Self, EDGARParserError> {
        let edgar_parser = Self::create_from_ticker(ticker).await?;
        Ok(edgar_parser)
    }

    /// Internal helper to create an `EdgarParser` by searching the ticker list.
    pub async fn create_from_ticker(ticker: &str) -> Result<EdgarParser, EDGARParserError> {
        let json_body = fetch_http_body("https://www.sec.gov/files/company_tickers.json")
            .await
            .map_err(|op: Box<dyn std::error::Error>| EDGARParserError::HttpError(op))?;

        // Deserialize JSON into a hashmap
        let tickers: HashMap<String, CompanyData> = serde_json::from_str(&json_body)?;

        tickers
            .iter()
            .find(|(_, c)| c.ticker == ticker)
            .map(|(_, c)| EdgarParser {
                cik_str: Some(c.cik_str),
                ticker: Some(c.ticker.clone()),
                title: Some(c.title.clone()),
                leading_zero_cik: format!("{:010}", c.cik_str),
                submissions: None,
                company_facts: None,
            })
            .ok_or_else(|| EDGARParserError::NotFound(format!("Ticker {} not found", ticker)))
    }

    /// Fetches the SEC Company Facts XBRL JSON for the current company.
    ///
    /// # Errors
    /// Returns `EDGARParserError::HttpError` or `EDGARParserError::JSONParseError` if the request fails.
    pub async fn fetch_company_facts(&mut self) -> Result<serde_json::Value, EDGARParserError> {
        if self.leading_zero_cik.is_empty() {
            return Err(EDGARParserError::NotFound(
                "Leading zero CIK is not set. Call create_from_ticker first.".to_string(),
            ));
        }

        let body_response = fetch_http_body(&format!(
            "data.sec.gov/api/xbrl/companyfacts/CIK{}.json",
            self.leading_zero_cik
        ))
        .await
        .map_err(|op: Box<dyn std::error::Error>| EDGARParserError::HttpError(op))?;

        let json_response: serde_json::Value =
            serde_json::from_str(&body_response).map_err(EDGARParserError::JSONParseError)?;

        // Store the company facts data in the struct
        self.company_facts = Some(json_response.clone());

        Ok(json_response)
    }

    /// Fetches the SEC Company Submissions JSON for the current company. This JSON data structure contains metadata such as current name,
    /// former name, and stock exchanges and ticker symbols of publicly-traded companies. The object’s property path contains at least one year’s of
    /// filing or to 1,000 (whichever is more) of the most recent filings in a compact columnar data array. If the entity has additional filings, files
    /// will contain an array of additional JSON files and the date range for the filings each one contains.
    ///
    /// # Errors
    /// Returns `EDGARParserError::HttpError` or `EDGARParserError::JSONParseError` if the request fails.
    pub async fn fetch_submissions(&mut self) -> Result<serde_json::Value, EDGARParserError> {
        if self.leading_zero_cik.is_empty() {
            return Err(EDGARParserError::NotFound(
                "Leading zero CIK is not set. Call create_from_ticker first.".to_string(),
            ));
        }

        let body_response = fetch_http_body(&format!(
            "data.sec.gov/submissions/CIK{}.json",
            self.leading_zero_cik
        ))
        .await
        .map_err(|op: Box<dyn std::error::Error>| EDGARParserError::HttpError(op))?;

        let json_response: serde_json::Value =
            serde_json::from_str(&body_response).map_err(EDGARParserError::JSONParseError)?;

        // Store the submissions data in the struct
        self.submissions = Some(json_response.clone());

        Ok(json_response)
    }

    /// The xbrl/frames API aggregates one fact for each reporting entity that is
    /// last filed and most closely fits the calendrical period requested. This API
    /// supports annual, quarterly, and instantaneous data:
    ///
    /// Where the units of measure specified in the XBRL contain a numerator and
    /// a denominator, these are separated by “-per-” such as “USD-per-shares”.
    /// Note that the default unit in XBRL is “pure”.
    ///
    /// The period format is:
    /// - CY#### for annual data (duration 365 days +/- 30 days),
    /// - CY####Q# for quarterly data (duration 91 days +/- 30 days), and
    /// - CY####Q#I for instantaneous data.
    ///
    /// Because company financial calendars can start and end on any month or day
    /// and even change in length from quarter to quarter according to the day of
    /// the week, the frame data is assembled by the dates that best align with a
    /// calendar quarter or year.
    ///
    /// Data users should be mindful of different reporting start and end dates for
    /// facts contained in a frame.
    pub async fn fetch_xbrl_frames(
        fact: &str,
        unit: &str,
        year: &u16,
        quarter: &u8,
    ) -> Result<serde_json::Value, EDGARParserError> {
        let path = format!(
            "data.sec.gov/api/xbrl/frames/us-gaap/{}/{}/CY{}{}I.json",
            fact, unit, year, quarter,
        );

        let body_response = fetch_http_body(&path)
            .await
            .map_err(|op: Box<dyn std::error::Error>| EDGARParserError::HttpError(op))?;

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

    #[tokio::test]
    async fn test_new_success() {
        let ticker = "AAPL";
        let result = EdgarParser::new(ticker).await;
        assert!(
            result.is_ok(),
            "Expected Ok result but got Err: {:?}",
            result.err()
        );
        result.unwrap();
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

    #[tokio::test]
    async fn test_fetch_xbrl_frames_success() {
        let fact: &'static str = "Assets";
        let unit: &'static str = "USD";
        let year: u16 = 2020;
        let quarter: u8 = 1;
        let result = EdgarParser::fetch_xbrl_frames(fact, unit, &year, &quarter);
        assert!(result.await.is_ok());

        // let json = result.unwrap();
        // assert_eq!(json["label"], "Accounts Payable, Current");
    }
}
