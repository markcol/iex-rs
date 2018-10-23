/*! Market

## HTTP request

/market

This endpoint returns near real-time traded volume on the markets. Market data
is captured by the IEX system from approximately 7:45 a.m. to 5:15 p.m. ET.

## Parameters

| Parameter | Details |
format | • Parameter is optional |
| | • Value can only be csv |
| | • When parameter is not present, format defaults to JSON |

## Response

| Key | Description |
| mic | refers to the Market Identifier Code (MIC). |
| tapeId | refers to the tape id of the venue. |
| venueName | refers to name of the venue defined by IEX. |
| volume | refers to the amount of traded shares reported by the venue. |
| tapeA | refers to the amount of Tape A traded shares reported by the venue. |
| tapeB | refers to the amount of Tape B traded shares reported by the venue. |
| tapeC | refers to the amount of Tape C traded shares reported by the venue. |
| marketPercent | refers to the venue’s percentage of shares traded in the market. |
| lastUpdated | refers to the last update time of the data in milliseconds since midnight Jan 1, 1970. |

## HTTP request example

`GET /market`

The above example will return JSON with the following keys

```json
[
  {
    "mic": "TRF",
    "tapeId": "-",
    "venueName": "TRF Volume",
    "volume": 589171705,
    "tapeA": 305187928,
    "tapeB": 119650027,
    "tapeC": 164333750,
    "marketPercent": 0.37027,
    "lastUpdated": 1480433817317
  },
 ...
]
```
*/

use super::Result;

pub struct Markets;

pub type Market = Vec<MarketData>;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketData {
  /// Market Identifer Code (MIC)
  mic: String,
  /// Tape ID of the vanue.
  tape_id: String,
  /// Name of the venue defined by IEX.
  venue_name: String,
  /// Amount of traded shares reported by the venue.
  volume: u64,
  /// Amount of Tape A traded shares reported by the venue.
  tape_a: u64,
  /// Amount of Tape B traded shares reported by the venue.
  tape_b: u64,
  /// Amount of Tape C traded shares reported by the venue.
  tape_c: u64,
  /// Venue's percentage of shares traded in the market.
  market_percent: f64,
  /// Last update time of the data in milliseconds since midnigt Jan 1, 1970.
  last_updated: u64, // TODO(markcol): convert from UNIX Epoch to timestamp
}

impl Markets {
  pub fn market(&self) -> Result<Market> {
    Ok(Vec::new())
  }
}
