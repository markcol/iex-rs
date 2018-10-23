/*! Reference Data

Returns various reference data.

 */

use super::{from_bool_str, from_str, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde_aux::prelude::*;

pub struct ReferenceData;

// TODO(markcol): need to deserialze from string form to enum value.
pub enum CommonIssueType {
    ADR,
    REIT,
    ClosedEndFund,
    SecondaryIssue,
    LimitedPartnership,
    CommonStock,
    ETF,
    NA, // Not applicable
}

pub type Symbols = Vec<SymbolData>;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolData {
    /// The symbol represented in Nasdaq Integrated symbology (INET).
    symbol: String,
    /// The name of the company or security.
    name: String,
    /// The date the symbol reference data was generated.
    date: NaiveDate,
    /// True if the symbol is enabled for trading on IEX.
    is_enabled: bool,
    /// The common issue type.
    #[serde(rename = "type")]
    issue_type: String, // TODO(markcol): Convert to use CommonIssueType
    /// Unique ID applied by IEX to track securities through symbol changes.
    #[serde(default, deserialize_with = "from_str")]
    iex_id: u64,
}

pub type CorporateActions = Vec<CorporateActionsData>;

// TODO(markcol): convert record_update_time and daily_list_timestamp to use
// Chrono datetime.
#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct CorporateActionsData {
    #[serde(rename = "RecordID")]
    record_id: String,
    daily_list_timestamp: String,
    effective_date: NaiveDate,
    issue_event: String,
    #[serde(rename = "CurrentSymbolinINETSymbology")]
    current_symbol_in_inets_symbology: String,
    #[serde(rename = "CurrentSymbolinCQSSymbology")]
    current_symbol_in_cqss_symbology: String,
    #[serde(rename = "CurrentSymbolinCMSSymbology")]
    current_symbol_in_cmss_symbology: String,
    #[serde(rename = "NewSymbolinINETSymbology")]
    new_symbol_in_inets_symbology: String,
    #[serde(rename = "NewSymbolinCQSSymbology")]
    new_symbol_in_cqss_symbology: String,
    #[serde(rename = "NewSymbolinCMSSymbology")]
    new_symbol_in_cmss_symbology: String,
    current_security_name: String,
    new_security_name: String,
    current_company_name: String,
    new_company_name: String,
    current_listing_center: String,
    #[serde(default, deserialize_with = "from_str")]
    new_round_lot_size: u64,
    #[serde(
        rename = "CurrentLULDTierIndicator",
        default,
        deserialize_with = "from_str"
    )]
    current_luld_tier_indicator: u64,
    #[serde(
        rename = "NewLULDTierIndicator",
        default,
        deserialize_with = "from_str"
    )]
    new_luld_tier_indicator: u64,
    #[serde(default, deserialize_with = "from_str")]
    expiration_date: u64,
    #[serde(default, deserialize_with = "from_str")]
    separation_date: u64,
    #[serde(default, deserialize_with = "from_str")]
    settlement_date: u64,
    #[serde(default, deserialize_with = "from_str")]
    maturity_date: u64,
    #[serde(default, deserialize_with = "from_str")]
    redemption_date: u64,
    current_financial_status: String,
    new_financial_status: String,
    #[serde(deserialize_with = "from_bool_str")]
    when_issued_flag: bool,
    #[serde(deserialize_with = "from_bool_str")]
    when_distributed_flag: bool,
    #[serde(rename = "IPOFlag", deserialize_with = "from_bool_str")]
    ipo_flag: bool,
    #[serde(rename = "NotesforEachEntry")]
    notes_for_each_entry: String,
    record_update_time: String,
}

impl ReferenceData {
    /// Returns an array of symbols IEX supports for trading. This list is
    /// updated daily as of 7:45 a.m. ET. Symbols may be added or removed by
    /// IEX after the list was produced.
    ///
    /// ## HTTP request
    ///
    /// /ref-data/symbols
    ///
    /// ## Parameters
    ///
    /// | Parameter	| Details |
    /// | format	| • Parameter is optional |
    /// | | • Value can only be csv |
    /// | | • When parameter is not present, format defaults to JSON |
    ///
    /// ## Response
    ///
    /// | Key | Description |
    /// | symbol | refers to the symbol represented in Nasdaq Integrated symbology (INET). |
    /// | name	| refers to the name of the company or security. |
    /// | date	| refers to the date the symbol reference data was generated. |
    /// | isEnabled	| will be true if the symbol is enabled for trading on IEX. |
    /// | type | refers to the common issue type:
    /// | | AD - ADR |
    /// | | RE - REIT |
    /// | | CE - Closed end fund |
    /// | | SI - Secondary Issue |
    /// | | LP - Limited Partnerships |
    /// | | CS - Common Stock |
    /// | | ET - ETF |
    /// | iexId | unique ID applied by IEX to track securities through symbol changes. |

    pub fn symbols(&self) -> Result<Symbols> {
        Ok(Vec::new())
    }

    /// corporate_actions returns an array of new issues, symbol and name
    /// changes, and deleted issues, as well as new firms, name changes, and
    /// deleted firms for IEX-listed securities.
    ///
    /// Records are added once known by the Exchange and will be removed when the
    /// Effective Date is in the past.
    ///
    /// Updates are posted once per hour from 8:00 a.m. to 6:00 p.m. ET on each
    /// trading day
    ///
    /// ## HTTP request
    ///
    /// /ref-data/daily-list/corporate-actions
    /// /ref-data/daily-list/corporate-actions/20171210
    /// /ref-data/daily-list/corporate-actions/sample
    ///
    /// ## Options
    ///
    /// | Range | Description | Source |
    /// | date | Specific date | Daily list data for a specified date in the format YYYYMMDD,if available, or sample. If sample, a sample file will be returned. |
    ///
    /// ## Parameters
    ///
    /// | Parameter	| Details |
    /// | format | • Parameter is optional |
    /// | | • Value can be csv or psv |
    /// | | • When parameter is not present, format defaults to JSON |
    /// | token	| • Parameter is optional |
    /// | | • Value is the API token from your IEX user account |
    /// | | • If you have been permissioned for CUSIP information you’ll receive a CUSIP field, othewise data defaults to exclude CUSIP. |
    ///
    /// ## Response
    ///
    /// Refer to the
    /// [Daily list specification](https://iextrading.com/docs/IEX%20Daily%20List%20File%20Specification.pdf)
    /// for futher details.
    ///
    /// ## HTTP request example
    ///
    /// `GET /ref-data/daily-list/symbol-directory`
    ///
    /// The above example will return JSON with the following keys
    ///
    /// ```json
    /// [
    ///   {
    ///     "RecordID": " CA20171108153808144",
    ///     "DailyListTimestamp": "2017-11-08T17:00:00",
    ///     "EffectiveDate": "2017-11-10",
    ///     "IssueEvent": "AA",
    ///     "CurrentSymbolinINETSymbology": "ZEXIT-",
    ///     "CurrentSymbolinCQSSymbology": "ZEXITp",
    ///     "CurrentSymbolinCMSSymbology": "ZEXIT PR",
    ///     "NewSymbolinINETSymbology": "",
    ///     "NewSymbolinCQSSymbology": "",
    ///     "NewSymbolinCMSSymbology": "",
    ///     "CurrentSecurityName": "ZEXIT Preffered Stock",
    ///     "NewSecurityName": "",
    ///     "CurrentCompanyName": "ZEXIT Test Company",
    ///     "NewCompanyName": "",
    ///     "CurrentListingCenter": "",
    ///     "NewListingCenter": "V",
    ///     "DelistingReason": "",
    ///     "CurrentRoundLotSize": "100",
    ///     "NewRoundLotSize": "",
    ///     "CurrentLULDTierIndicator": "0",
    ///     "NewLULDTierIndicator": "",
    ///     "ExpirationDate": "0",
    ///     "SeparationDate": "0",
    ///     "SettlementDate": "0",
    ///     "MaturityDate": "0",
    ///     "RedemptionDate": "0",
    ///     "CurrentFinancialStatus": "0",
    ///     "NewFinancialStatus": "",
    ///     "WhenIssuedFlag": "N",
    ///     "WhenDistributedFlag": "N",
    ///     "IPOFlag": "N",
    ///     "NotesforEachEntry": "New preferred ZIEXT security",
    ///     "RecordUpdateTime": "2017-11-08T16:34:43"
    ///   },
    ///   {...}
    /// ]
    /// ```
    pub fn corporate_actions(&self) -> Result<CorporateActions> {
        Ok(Vec::new())
    }

    pub fn dividends(&self) -> Result<()> {
        Ok(())
    }

    pub fn next_day_ex_date(&self) -> Result<()> {
        Ok(())
    }

    pub fn listed_symbol_directory(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_date_deserialization() {
        let json_data = r#"{
            "symbol":"A",
            "name":"Agilent Technologies Inc.",
            "date":"2018-10-23",
            "isEnabled":true,
            "type":"cs",
            "iexId":"2"
        }"#;

        let s: SymbolData = serde_json::from_str(&json_data).unwrap();
        assert_eq!(s.date, NaiveDate::from_ymd(2018, 10, 23));
        assert_eq!(s.iex_id, 2);
    }

    #[test]
    fn symbol_empty_iex_id_deserialization() {
        let json_data = r#"{
            "symbol":"A",
            "name":"Agilent Technologies Inc.",
            "date":"2018-10-23",
            "isEnabled":true,
            "type":"cs",
            "iexId":""
        }"#;

        let s: SymbolData = serde_json::from_str(&json_data).unwrap();
        assert_eq!(s.iex_id, 0);
    }

    #[test]
    fn corporate_actions_deserialization() {
        let json_data = r#"{
            "RecordID": " CA20171108153808144",
            "DailyListTimestamp": "2017-11-08T17:00:00",
            "EffectiveDate": "2017-11-10",
            "IssueEvent": "AA",
            "CurrentSymbolinINETSymbology": "ZEXIT-",
            "CurrentSymbolinCQSSymbology": "ZEXITp",
            "CurrentSymbolinCMSSymbology": "ZEXIT PR",
            "NewSymbolinINETSymbology": "",
            "NewSymbolinCQSSymbology": "",
            "NewSymbolinCMSSymbology": "",
            "CurrentSecurityName": "ZEXIT Preffered Stock",
            "NewSecurityName": "",
            "CurrentCompanyName": "ZEXIT Test Company",
            "NewCompanyName": "",
            "CurrentListingCenter": "",
            "NewListingCenter": "V",
            "DelistingReason": "",
            "CurrentRoundLotSize": "100",
            "NewRoundLotSize": "",
            "CurrentLULDTierIndicator": "0",
            "NewLULDTierIndicator": "",
            "ExpirationDate": "0",
            "SeparationDate": "0",
            "SettlementDate": "0",
            "MaturityDate": "0",
            "RedemptionDate": "0",
            "CurrentFinancialStatus": "0",
            "NewFinancialStatus": "",
            "WhenIssuedFlag": "N",
            "WhenDistributedFlag": "N",
            "IPOFlag": "N",
            "NotesforEachEntry": "New preferred ZIEXT security",
            "RecordUpdateTime": "2017-11-08T16:34:43"
        }"#;

        let ca: CorporateActionsData = serde_json::from_str(&json_data).unwrap();
        assert_eq!(ca.ipo_flag, false);
        assert_eq!(ca.effective_date, NaiveDate::from_ymd(2017, 11, 10));
    }
}
