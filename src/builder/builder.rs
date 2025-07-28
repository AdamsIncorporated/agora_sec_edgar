use crate::builder::filing::FilingTypeOption;
use crate::edgar::EdgarParser;
use crate::error::EDGARParserError;
use url::Url;

#[derive(Debug, PartialEq)]
struct EdgarQueryBuilder {
    base_url: String,
    filing_type: FilingTypeOption,
    dateb: String,
    owner: String,
    count: String,
    search_text: String,
    edgar_parser: EdgarParser,
}

impl EdgarQueryBuilder {
    pub fn new(edgar_parser: EdgarParser) -> Self {
        Self {
            base_url: "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&".to_string(),
            filing_type: Default::default(),
            dateb: Default::default(),
            owner: "include".to_string(),
            count: "10".to_string(),
            search_text: Default::default(),
            edgar_parser,
        }
    }

    pub fn build(&self) -> Result<Url, EDGARParserError> {
        let url = format!(
            "{base}CIK={cik}&type={filing_type}&dateb={dateb}&owner={owner}&count={count}&search_text={search_text}&output=atom",
            base = self.base_url,
            cik = self.edgar_parser.cik_str,
            filing_type = self.filing_type,
            dateb = self.dateb,
            owner = self.owner,
            count = self.count,
            search_text = self.search_text
        );
        let query = Url::parse(&url)?;
        Ok(query)
    }
}
