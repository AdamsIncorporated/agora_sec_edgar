use crate::error::EDGARParserError;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct EdgarParser {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CompanyData {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CompanyDataList {
    pub tickers: Vec<CompanyData>,
}

impl EdgarParser {
    pub fn new(self, ticker: &str) -> Result<Self, EDGARParserError> {
        let edgar_parser = self.create_from_ticker(ticker)?;
        Ok(edgar_parser)
    }

    pub fn create_from_ticker(&self, ticker: &str) -> Result<EdgarParser, EDGARParserError> {
        let json_body =
            crate::api::get_http_response_body("www.sec.gov", "/edgar/cik-tickers.json")
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
            })
            .ok_or_else(|| EDGARParserError::NotFound(format!("Ticker {} not found", ticker)))
    }

    pub fn parse(&self, data: &str) -> Result<(), EDGARParserError> {
        Ok(())
    }
}
