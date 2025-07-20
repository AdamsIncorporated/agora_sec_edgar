mod api;
use api::{CompanyTicker, CompanyTickers, fetch_cik_json_from_server, lookup_cik_from_ticker};

pub fn get_cik_from_ticker(ticker: &str) -> Option<String> {
    Some(ticker.to_ascii_uppercase())
}

fn fetch_company_tickers_json_document(ticker: &str) -> Option<String> {
    let company_tickers: api::CompanyTickers = fetch_cik_json_from_server().ok()?;
    let Some(ticker) = lookup_cik_from_ticker(&company_tickers, ticker) else {
        return None;
    };
    Some(ticker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_get_cik_from_ticker_returns_uppercase() {
        let ticker = "tsla";
        let expected_cik = Some(String::from("TSLA"));
        assert_eq!(get_cik_from_ticker(ticker), expected_cik);
    }

    #[test]
    fn test_get_cik_from_ticker_empty_string() {
        let ticker = "";
        let expected_cik = Some(String::from(""));
        assert_eq!(get_cik_from_ticker(ticker), expected_cik);
    }

    #[test]
    fn test_keys_match() {
        let json = r#"
        {
            "0": {
                "cik_str": 1045810,
                "ticker": "NVDA",
                "title": "NVIDIA CORP"
            },
            "1": {
                "cik_str": 320193,
                "ticker": "AAPL",
                "title": "APPLE INC"
            }
        }
        "#;

        let parsed: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(json).unwrap();

        // Expected keys:
        let expected_keys: HashSet<_> = ["0", "1"].iter().cloned().collect();

        let parsed_keys: HashSet<_> = parsed.keys().map(|k| k.as_str()).collect();

        assert_eq!(parsed_keys, expected_keys);
    }

    #[test]
    fn test_deserialize_company_tickers() {
        let json_data = r#"
        {
            "tickers": [
                {
                    "cik_str": 123456,
                    "ticker": "AAPL",
                    "title": "Apple Inc."
                },
                {
                    "cik_str": 789012,
                    "ticker": "GOOGL",
                    "title": "Alphabet Inc."
                }
            ]
        }
        "#;

        let parsed: CompanyTickers = serde_json::from_str(json_data).expect("Failed to parse JSON");

        let expected = CompanyTickers {
            tickers: vec![
                CompanyTicker {
                    cik_str: 123456,
                    ticker: "AAPL".to_string(),
                    title: "Apple Inc.".to_string(),
                },
                CompanyTicker {
                    cik_str: 789012,
                    ticker: "GOOGL".to_string(),
                    title: "Alphabet Inc.".to_string(),
                },
            ],
        };

        assert_eq!(parsed, expected);
    }
}
