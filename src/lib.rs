/*!

## Attribution

If you redistribute IEX API data:

Cite IEX using the following text and link: “Data provided for free by [IEX].
View [IEX’s Terms of Use](https://iextrading.com/api-exhibit-a/).”
Additionally, if you display our TOPS price data, cite
“[IEX Real-Time Price](https://iextrading.com/developer)” near the price.

## Support

If you find any issues with the IEX API or have any questions, please file an
issue on IEX's [Github](https://github.com/iexg/IEX-API).

## Authentication

The IEX API is currently open and does not require authentication to access its
data.

## Endpoints

* All endpoints are prefixed with: `https://api.iextrading.com/1.0`
* We support [JSONP] for all endpoints.

## SSL

IEX provides a valid, signed certificate for the API methods. Be sure your
connection library supports HTTPS with the
[SNI extension](https://en.wikipedia.org/wiki/Server_Name_Indication).

## HTTP methods

The IEX API only supports **GET** requests at this time.

## Parameters

* Parameter values must be comma-delimited when requesting multiple.
    * (i.e. `?symbols=SNAP,fb` is correct.)
* Casing does not matter when passing values to a parameter.
    * (i.e. Both `?symbols=fb` and `?symbols=FB` will work.)
* Be sure to url-encode the values you pass to your parameter.
    * (i.e. `?symbols=AIG+` encoded is `?symbols=AIG%2b.`)

## Filter results

All HTTP request endpoints support a `filter` parameter to return a subset of
data. Pass a comma-delimited list of field names to filter. Field names are
case-sensitive and are found in the **Reference** section of each endpoint.

Example: `?filter=symbol,volume,lastSalePrice` will return only the three
fields specified.

## WebSockets

WebSocket support is limited at this time to Node.js server clients and
socket.io browser clients. IEX uses [socket.io] for its WebSocket server.
The WebSocket examples in the IEX documentation assume a socket.io browser
client is being used.

* For socket.io clients, use: `https://ws-api.iextrading.com/1.0`

WebSockets example that shows a connection to the tops channel and a
subscribtion to `snap,fb,aig+` topics

```javascript
// Import socket.io with a connection to a channel (i.e. tops)
const socket = require('socket.io-client')('https://ws-api.iextrading.com/1.0/tops')

// Listen to the channel's messages
socket.on('message', message => console.log(message))

// Connect to the channel
socket.on('connect', () => {

  // Subscribe to topics (i.e. appl,fb,aig+)
  socket.emit('subscribe', 'snap,fb,aig+')

  // Unsubscribe from topics (i.e. aig+)
  socket.emit('unsubscribe', 'aig+')
})

// Disconnect from the channel
socket.on('disconnect', () => console.log('Disconnected.'))
```

## Related APIS

* C#: https://www.codepoc.io/blog/web-api/5297/get-stock-historical-data-based-on-it-stock-symbol-iextrading-api-c
* Go: https://github.com/timpalpant/go-iex
*

[IEX]: https://iextrading.com/developer
[JSONP]: https://en.wikipedia.org/wiki/JSONP
[socket.io]: http://socket.io/
 */

#![allow(dead_code, missing_docs, unused_imports)]

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_aux;
extern crate serde_json;

use chrono::prelude::*;
use failure::{Backtrace, Context, Fail, ResultExt};
use serde::de::{self, Deserialize, Deserializer};
use serde_aux::prelude::*;
use serde_json::Value;
use std::fmt::{self, Display, Formatter};
use std::result;
use std::str::FromStr;

mod market_data;
mod markets;
mod reference;
mod stats;
mod stocks;
mod types;

pub use self::market_data::*;
pub use self::markets::*;
pub use self::reference::*;
pub use self::stats::*;
pub use self::stocks::*;
pub use self::types::*;

#[derive(Debug)]
pub struct MyError {
    inner: Context<String>,
}

impl Fail for MyError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

// Allows writing `MyError::from("oops"))?`
impl From<&'static str> for MyError {
    fn from(msg: &'static str) -> MyError {
        MyError {
            inner: Context::new(msg.into()),
        }
    }
}

// Allows adding more context via a String
impl From<Context<String>> for MyError {
    fn from(inner: Context<String>) -> MyError {
        MyError { inner }
    }
}

// Allows adding more context via a &str
impl From<Context<&'static str>> for MyError {
    fn from(inner: Context<&'static str>) -> MyError {
        MyError {
            inner: inner.map(|s| s.to_string()),
        }
    }
}

pub type Result<T> = result::Result<T, failure::Error>;

/// IEX JSON Endpoint
const IEX_ENDPOINT: &str = "https://api.iextrading.com/1.0";

/// IEX Webscoket Endpoint
const IEX_WEBSOCKET_ENDPOINT: &str = "https://ws-api.iextrading.com/1.0";

/// `Client` acts as a Handler for the `Response` enum.
#[derive(Default)]
pub struct Client;

impl Client {
    /// Create a new Client.
    pub fn new() -> Self {
        Client
    }

    /// stocks_request is the main entry-point to the IEX Stocks API.
    pub fn stocks_request<S>(&self, symbol: S, req: StocksEndpoint) -> Result<Response>
    where
        S: Into<String>,
    {
        let url = format!(
            "{base}/stock/{symbol}/{endpoint}",
            base = IEX_ENDPOINT,
            symbol = symbol.into(),
            endpoint = req.to_endpoint()
        );

        Ok(reqwest::get(&url)?.json()?)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response(pub Value);

impl Response {
    pub fn try_into<T>(self) -> Result<T>
    // TEMP(Response): Keep until try_from trait becomes stable rust feature.
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        Ok(serde_json::from_value(self.0)?)
    }
}

pub trait Endpoint {
    fn to_endpoint(self) -> String;
}

pub fn from_str<'de, T, D>(deserializer: D) -> result::Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    if s.len() == 0 {
        s = "0".to_string();
    }
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn from_bool_str<'de, T, D>(deserializer: D) -> result::Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    if s == "N" || s == "F" || s == "" {
        s = "false".to_string();
    } else {
        s = "true".to_string();
    }
    T::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    static CLIENT: Client = Client;
    #[allow(non_upper_case_globals)]
    static symbol: &'static str = "aapl";
    #[allow(non_upper_case_globals)]
    static duration: Duration = Duration::OneDay;

    #[test]
    fn client_request_book() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Book).is_ok());
    }

    #[test]
    fn client_request_chart() {
        assert!(CLIENT
            .stocks_request(
                symbol,
                StocksEndpoint::Chart {
                    duration,
                    params: None
                }
            )
            .is_ok());
    }

    #[test]
    fn client_request_company() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Company)
            .is_ok());
    }

    #[test]
    fn client_request_delayed_quote() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::DelayedQuote)
            .is_ok());
    }

    #[test]
    fn client_request_dividends() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Dividends { duration })
            .is_ok());
    }

    #[test]
    fn client_request_earnings() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Earnings)
            .is_ok());
    }

    #[test]
    fn client_request_effective_spread() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::EffectiveSpread)
            .is_ok());
    }

    #[test]
    fn client_request_financials() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Financials)
            .is_ok());
    }

    #[test]
    fn client_request_list() {
        assert!(CLIENT
            .stocks_request(
                "market",
                StocksEndpoint::List {
                    param: ListParam::Gainers
                }
            )
            .is_ok());
    }

    #[test]
    fn client_request_logo() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Logo).is_ok());
    }

    #[test]
    fn client_request_news() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::News { range: None })
            .is_ok());
    }

    #[test]
    fn client_request_ohlc() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Ohlc).is_ok());
    }

    #[test]
    fn client_request_peers() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Peers).is_ok());
    }

    #[test]
    fn client_request_previous() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Previous)
            .is_ok());
    }

    #[test]
    fn client_request_price() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Price).is_ok());
    }

    #[test]
    fn client_request_quote() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Quote).is_ok());
    }

    #[test]
    fn client_request_relevant() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Relevant)
            .is_ok());
    }

    #[test]
    fn client_request_splits() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::Splits { duration })
            .is_ok());
    }

    #[test]
    fn client_request_stats() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Stats).is_ok());
    }

    #[test]
    fn client_request_threshold_securities() {
        assert!(CLIENT
            .stocks_request("market", StocksEndpoint::ThresholdSecurities { date: None })
            .is_ok());
    }

    #[test]
    fn client_request_volume_by_venue() {
        assert!(CLIENT
            .stocks_request(symbol, StocksEndpoint::VolumeByVenue)
            .is_ok());
    }
}
