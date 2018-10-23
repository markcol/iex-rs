/*!
 */

use super::Result;
use std::collections::HashMap;

pub type Auctions = HashMap<String, AuctionData>;

/// DEEP broadcasts an Auction Information message every one second between the
/// Lock-in Time and the auction match for Opening and Closing Auctions, and
/// during the Display Only Period for IPO, Halt, and Volatility Auctions.
/// Only IEX listed securities are eligible for IEX Auctions.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionData {
    /// Auction type (Open, Close, Halt, Volatility, IPO).
    auction_type: String,
    /// Number of shares paired at the reference price using orders on the
    /// auction book.
    paired_shares: u64,
    /// Number of unpaired shares at the reference price using orders on the
    /// auction book.
    imbalance_shares: u64,
    /// Which side is unbalanced using orders on the auction book.
    imbalance_side: String,
    /// Clearing price at or within the reference price range using orders on
    /// the auction book
    reference_price: u64,
    /// Clearing price using eligible auction orders.
    indicative_price: u64,
    /// Clearing price using orders on the auction book.
    auction_book_price: u64,
    /// Reference price used for the auction collar, if any.
    collar_reference_price: u64,
    /// Lower threshold price of the auction collar, if any.
    lower_collar_price: f64,
    /// Upper threshold price of the auction collar, if any.
    upper_collar_price: f64,
    /// Number of extensions an auction has received.
    extension_number: u64,
    /// Projected time of the auction match. Formatted as HH:MM:SS.
    start_time: u64,
    /// Timestamp of the auction information.
    timestamp: u64,
}

pub struct MarketData;

impl MarketData {
    pub fn tops(&self) -> Result<()> {
        Ok(())
    }

    pub fn last(&self) -> Result<()> {
        Ok(())
    }

    pub fn hist(&self) -> Result<()> {
        Ok(())
    }

    pub fn deep(&self) -> Result<()> {
        Ok(())
    }

    pub fn book(&self) -> Result<()> {
        Ok(())
    }

    pub fn trades(&self) -> Result<()> {
        Ok(())
    }

    pub fn system_event(&self) -> Result<()> {
        Ok(())
    }

    pub fn trading_status(&self) -> Result<()> {
        Ok(())
    }

    pub fn operational_halt_status(&self) -> Result<()> {
        Ok(())
    }

    pub fn short_sale_price_test_status(&self) -> Result<()> {
        Ok(())
    }

    pub fn security_event(&self) -> Result<()> {
        Ok(())
    }

    pub fn trade_break(&self) -> Result<()> {
        Ok(())
    }

    pub fn auction(&self) -> Result<()> {
        Ok(())
    }

    pub fn official_price(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_auction_hash() {
        let json_data = r#"{
            "ZIEXT": {
                "auctionType":"Close",
                "pairedShares":2000,
                "imbalanceShares":0,
                "imbalanceSide":"None",
                "referencePrice":1,
                "indicativePrice":1,
                "auctionBookPrice":1,
                "collarReferencePrice":1,
                "lowerCollarPrice":0.5,
                "upperCollarPrice":1.5,
                "extensionNumber":0,
                "startTime":1540324800000,
                "timestamp":1540324799126
            }
        }"#;

        let m: Auctions = serde_json::from_str(&json_data).unwrap();
        let ad = &m["ZIEXT"];
        assert_eq!(ad.auction_type, "Close");
    }
}
