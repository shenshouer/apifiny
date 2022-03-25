use super::{
    http_error,
    utils::{get_http_client, signature_req},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// OTHER
const BASE_URL_REST_OTHER: &str = "https://api.apifiny.com";
// const BASE_URL_FIX_OTHER: &str = "fix.api.apifiny.com:1443";

// https://doc.apifiny.com/connect/#rest-api
pub struct RestClient {
    conf: crate::ApiFiny,
    venue: Option<super::Venue>,
}

impl RestClient {
    pub fn new(conf: super::ApiFiny, venue: Option<super::Venue>) -> RestClient {
        RestClient { conf, venue }
    }

    // Base Information

    pub async fn list_venue_info(&self, venue: &str) -> Result<ApiFinyResponse<Vec<VenueInfo>>> {
        let req_url = format!(
            "{}/ac/v2/{}/utils/listVenueInfo",
            BASE_URL_REST_OTHER, venue
        );
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    pub async fn list_currency(&self, venue: &str) -> Result<ApiFinyResponse<Vec<CurrencyInfo>>> {
        let req_url = format!("{}/ac/v2/{}/utils/listCurrency", BASE_URL_REST_OTHER, venue);
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    pub async fn list_symbol_info(&self, venue: &str) -> Result<ApiFinyResponse<Vec<SymbolInfo>>> {
        let req_url = format!(
            "{}/ac/v2/{}/utils/listSymbolInfo",
            BASE_URL_REST_OTHER, venue
        );
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    pub async fn current_time_millis(&self, venue: &str) -> Result<ApiFinyResponse<i64>> {
        let req_url = format!(
            "{}/ac/v2/{}/utils/currentTimeMillis",
            BASE_URL_REST_OTHER, venue
        );
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    // Market Data
    // Rate Limit: We throttle public endpoints by IP address 1 request per second

    pub async fn order_book(&self, symbol: &str, venue: &str) -> Result<OrderBook> {
        let req_url = format!(
            "{}/md/orderbook/v1/{}/{}",
            BASE_URL_REST_OTHER, symbol, venue
        );
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    pub async fn trade(&self, symbol: &str, venue: &str) -> Result<Vec<TradeOrder>> {
        let req_url = format!("{}/md/trade/v1/{}/{}", BASE_URL_REST_OTHER, symbol, venue);
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    // VENUE	Venue name
    // BASE	Base asset
    // QUOTE	Quote Asset
    // PERIOD	The period of each candle bar, 1m, 5m, 15m, 30m, 1h, 4h, 1d, 1w, 1M
    // startTime	The start time period
    // endTime	The end time period
    pub async fn kline(
        &self,
        venue: &str,
        base: &str,
        quote: &str,
        period: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<KLine>> {
        let req_url = format!(
            "{}/md/kline/v1/{}/{}/{}/{}",
            BASE_URL_REST_OTHER, venue, base, quote, period
        );
        let mut query = None;
        if start_time.is_some() && end_time.is_some() {
            query = Some(json!({
                "startTime": start_time.unwrap().timestamp_subsec_millis(),
                "endTime": end_time.unwrap().timestamp_subsec_millis(),
            }))
        }
        Ok(self
            .do_http(reqwest::Method::GET, req_url, query, None)
            .await?
            .json()
            .await?)
    }

    pub async fn ticker(&self, symbol: &str, venue: &str) -> Result<Ticker> {
        let req_url = format!("{}/md/ticker/v1/{}/{}", BASE_URL_REST_OTHER, symbol, venue);
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    pub async fn consolidated_order_book(&self, symbol: &str) -> Result<ConsolidatedOrderBook> {
        let req_url = format!("{}/md/cob/v1/{}", BASE_URL_REST_OTHER, symbol);
        // let s = self.do_http(req_url, None, None).await?.text().await?;
        // println!("==>{}", s);
        // Ok(())

        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    // Account

    pub async fn query_account_info(&self) -> Result<()> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/account/queryAccountInfo", venue.rest);

            let query = Some(json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
            }));
            let s = self
                .do_http(reqwest::Method::GET, req_url, query, None)
                .await?
                .text()
                .await?;
            println!("==>{}", s);
            return Ok(());

            // return Ok(self.do_http(req_url, query, None).await?.json().await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // do real http request
    async fn do_http(
        &self,
        method: reqwest::Method,
        req_url: String,
        query: Option<Value>,
        body: Option<Value>,
    ) -> Result<reqwest::Response> {
        let client = get_http_client()?;

        let mut req_builder = client
            .get(&req_url)
            .header(reqwest::header::CONTENT_TYPE, "application/json");

        // set query params
        if let Some(query) = query {
            req_builder = req_builder.query(&query);
        }

        // set body params
        if let Some(body) = body {
            req_builder = req_builder.json(&body);
        }

        let mut req = req_builder.build()?;
        signature_req(&self.conf, &mut req)?;

        // change method
        let m = req.method_mut();
        *m = method;

        let resp = client.execute(req).await?;

        let status_code = resp.status();
        if !status_code.is_success() {
            let msg = resp.text().await?;
            return Err(http_error(req_url, status_code, msg));
        }

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiFinyResponse<T> {
    result: Option<T>,
    error: Option<ApiFinyError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiFinyError {
    code: i64,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VenueInfo {
    exchange: String,
    status: VenueStatus,
    spt_instant: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum VenueStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    currency: String,
    currency_precision: i64,
    status: CurrencyStatus,
    withdraw_max_amount: f64,
    withdraw_min_amount: f64,
    withdraw_min_fee: f64,
    instant_fee_rate: String,
    coin: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum CurrencyStatus {
    DepositWithdraw,
    NotDepositNotWithdraw,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    symbol: String,
    base_asset: String,
    base_asset_precision: i64,
    quote_asset: String,
    quote_precision: i64,
    min_price: Option<f64>,
    max_price: Option<f64>,
    min_quantity: f64,
    max_quantity: f64,
    tick_size: f64,
    step_size: f64,
    min_notional: f64,
    max_notional: f64,
    status: VenueStatus,
}

use chrono::{serde::ts_milliseconds, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    symbol: String,
    #[serde(with = "ts_milliseconds")]
    updated_at: DateTime<Utc>,
    asks: Vec<PriceSizePair>,
    bids: Vec<PriceSizePair>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceSizePair(i64, i64);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOrder {
    symbol: String,
    provider: String,
    price: f64,
    side: f64,
    #[serde(with = "ts_milliseconds")]
    trade_time: DateTime<Utc>,
    exchange_id: String,
    #[serde(with = "ts_milliseconds")]
    update_time: DateTime<Utc>,
}

#[derive(Debug)]
pub enum Period {
    // 1m
    Minute1,
    // 5m
    Minute5,
    // 15m
    Minute15,
    // 30m
    Minute30,
    // 1h
    Hour1,
    // 4h
    Hour4,
    // 1d
    Day1,
    // 1w
    Week1,
    // 1M
    Month1,
}

use std::fmt::{self, Display, Formatter};
impl Display for Period {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Period::*;
        match self {
            Minute1 => write!(f, "1m"),
            Minute5 => write!(f, "5m"),
            Minute15 => write!(f, "15m"),
            Minute30 => write!(f, "30m"),
            Hour1 => write!(f, "1h"),
            Hour4 => write!(f, "4h"),
            Day1 => write!(f, "1d"),
            Week1 => write!(f, "1w"),
            Month1 => write!(f, "1M"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KLine {
    currency_pair: String,
    period: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    vol: f64,
    count: i64,
    #[serde(with = "ts_milliseconds")]
    timestamp: DateTime<Utc>,
    exchange: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    symbol: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    vol: f64,
    amount: f64,
    count: i64,
    provider: String,
    #[serde(with = "ts_milliseconds")]
    ticker_time: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    update_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsolidatedOrderBook {
    symbol: String,
    asks: Vec<Order>,
    bids: Vec<Order>,
    #[serde(with = "ts_milliseconds")]
    update_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    price: f64,
    qty: f64,
    source: String,
}
