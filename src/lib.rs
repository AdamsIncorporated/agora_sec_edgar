mod api;
use api::fetch_cik_json_from_server;

pub fn get_cik_from_ticker(ticker: &str) -> Option<String> {
    Some(ticker.to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_get_cik_json_document() {
        let json = fetch_cik_json_from_server().unwrap();
        assert!(!json.is_empty());
    }
}
