use crate::builder::filing::FilingTypeOption;
use crate::builder::owner::OwnerOption;
use crate::edgar::EdgarParser;
use crate::error::EDGARParserError;
use chrono::NaiveDate;
use url::Url;

#[derive(Debug, PartialEq)]
pub struct EdgarQueryBuilder {
    pub edgar_parser: EdgarParser,
    pub filing_type: FilingTypeOption,
    pub dateb: String,
    pub owner: OwnerOption,
    pub base_url: String,
    pub count: String,
    pub search_text: String,
}

impl EdgarQueryBuilder {
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

    pub fn build(&self) -> Result<Url, EDGARParserError> {
        let filing_type_string = self.filing_type.to_string();
        let owner_string = self.owner.to_string();
        let dateb_string = Self::set_and_validate_dateb(self.dateb.clone())?;

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
        let query = Url::parse(&url)?;
        Ok(query)
    }

    fn set_and_validate_dateb(dateb: String) -> Result<String, EDGARParserError> {
        if dateb.len() != 8 || !dateb.chars().all(|f| f.is_digit(10)) {
            return Err(EDGARParserError::InvalidDateFormat(dateb));
        } else {
            match NaiveDate::parse_from_str(&dateb, "%Y%m%d") {
                Ok(_) => Ok(dateb),
                Err(_) => Err(EDGARParserError::InvalidDateFormat(dateb)),
            }
        }
    }
}
