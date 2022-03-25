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

    // https://doc.apifiny.com/connect/#query-list-venues
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

    // https://doc.apifiny.com/connect/#query-list-currencies
    pub async fn list_currency(&self, venue: &str) -> Result<ApiFinyResponse<Vec<CurrencyInfo>>> {
        let req_url = format!("{}/ac/v2/{}/utils/listCurrency", BASE_URL_REST_OTHER, venue);
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    // https://doc.apifiny.com/connect/#query-list-symbols
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

    // https://doc.apifiny.com/connect/#query-current-timestamp-of-server
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

    // https://doc.apifiny.com/connect/#order-book-rest-api
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

    // https://doc.apifiny.com/connect/#trades-rest-api
    pub async fn trade(&self, symbol: &str, venue: &str) -> Result<Vec<TradeOrder>> {
        let req_url = format!("{}/md/trade/v1/{}/{}", BASE_URL_REST_OTHER, symbol, venue);
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    // https://doc.apifiny.com/connect/#ohlcv-rest-api
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

    // https://doc.apifiny.com/connect/#ticker-rest-api
    pub async fn ticker(&self, symbol: &str, venue: &str) -> Result<Ticker> {
        let req_url = format!("{}/md/ticker/v1/{}/{}", BASE_URL_REST_OTHER, symbol, venue);
        Ok(self
            .do_http(reqwest::Method::GET, req_url, None, None)
            .await?
            .json()
            .await?)
    }

    // https://doc.apifiny.com/connect/#consolidated-order-book-rest-api
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

    // https://doc.apifiny.com/connect/#query-account-info
    pub async fn query_account_info(&self) -> Result<ApiFinyResponse<Account>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/account/queryAccountInfo", venue.rest);

            let query = Some(json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
            }));

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, query, None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Query Account Asset
    // https://doc.apifiny.com/connect/#query-account-asset
    pub async fn list_balance(&self) -> Result<ApiFinyResponse<Vec<Balance>>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/listBalance", venue.rest);

            let query = Some(json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
            }));

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, query, None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Get Deposit Address
    // https://doc.apifiny.com/connect/#get-deposit-address
    pub async fn query_address(&self, coin: &str) -> Result<ApiFinyResponse<Vec<DepositAddress>>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/queryAddress", venue.rest);

            let query = Some(json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
                "coin": coin,
            }));

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, query, None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Create a Withdraw Ticket
    // https://doc.apifiny.com/connect/#create-a-withdraw-ticket
    pub async fn create_withdraw_ticket(&self) -> Result<ApiFinyResponse<WithdrawTicket>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/createWithdrawTicket", venue.rest);

            let query = Some(json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
            }));

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, query, None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Create a Withdraw Request
    // https://doc.apifiny.com/connect/#create-a-withdraw-request
    pub async fn create_withdraw(
        &self,
        params: CreateWithdrawParams,
    ) -> Result<ApiFinyResponse<Vec<Withdraw>>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/withdraw", venue.rest);

            let mut body = json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
            });
            let params = serde_json::to_value(params)?;

            super::utils::merge(&mut body, &params);
            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::POST, req_url, None, Some(body))
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Transfer Between Venues
    // https://doc.apifiny.com/connect/#transfer-between-venues
    pub async fn transfer_to_venue(
        &self,
        params: TransferBetweenVenuesParams,
    ) -> Result<ApiFinyResponse<Vec<TransferBetweenVenuesResponse>>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/transferToVenue", venue.rest);

            let mut query = json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
            });
            let params = serde_json::to_value(params)?;

            super::utils::merge(&mut query, &params);
            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, Some(query), None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Query Account History
    // https://doc.apifiny.com/connect/#query-account-history
    pub async fn query_asset_activity_list(
        &self,
        params: QueryAccountHistoryParams,
    ) -> Result<ApiFinyResponse<PagationResponse<AccountHistory>>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/queryAssetActivityList", venue.rest);

            let mut query = json!({
                "accountId": self.conf.apifiny_account_id,
            });
            let params = serde_json::to_value(params)?;

            super::utils::merge(&mut query, &params);
            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, Some(query), None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Query Current Fee Rate
    // https://doc.apifiny.com/connect/#query-current-fee-rate
    pub async fn get_commission_rate(
        &self,
        symbol: String,
    ) -> Result<ApiFinyResponse<Vec<FeeRate>>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/getCommissionRate", venue.rest);

            let query = json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
                "symbol": symbol,
            });

            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, Some(query), None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Query Instant Transfer Quota
    // https://doc.apifiny.com/connect/#query-instant-transfer-quota
    pub async fn query_max_instant_amount(&self, currency: String) -> Result<ApiFinyResponse<f64>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/query-max-instant-amount", venue.rest);

            let query = json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name,
                "currency": currency,
            });

            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::GET, req_url, Some(query), None)
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Create Conversion
    // support COINBASEPRO only
    // https://doc.apifiny.com/connect/#create-conversion
    pub async fn create_conversion(
        &self,
        params: CreateConversionParams,
    ) -> Result<ApiFinyResponse<CreateConversionResponse>> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/asset/currencyConversion", venue.rest);

            let mut body = json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name, // venue name, support COINBASEPRO only
            });

            let params = serde_json::to_value(params)?;
            super::utils::merge(&mut body, &params);

            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::POST, req_url, None, Some(body))
                .await?
                .json()
                .await?);
        }
        Err(super::Error::VenueNotSet())
    }

    // Trading

    // Create New Order
    // https://doc.apifiny.com/connect/#create-new-order
    pub async fn create_order(&self, params: CreateOrderParams) -> Result<CreateOrderResponse> {
        if let Some(ref venue) = self.venue {
            let req_url = format!("{}/order/newOrder", venue.rest);

            let mut body = json!({
                "accountId": self.conf.apifiny_account_id,
                "venue": venue.name, // venue name, support COINBASEPRO only
            });

            let params = serde_json::to_value(params)?;
            super::utils::merge(&mut body, &params);

            // println!("==>{}", body);
            // let s = self
            //     .do_http(reqwest::Method::GET, req_url, None, Some(body))
            //     .await?
            //     .text()
            //     .await?;
            // println!("==>{}", s);
            // return Ok(());

            return Ok(self
                .do_http(reqwest::Method::POST, req_url, None, Some(body))
                .await?
                .json()
                .await?);
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

#[derive(Debug, Serialize)]
pub struct CreateWithdrawParams {
    // currency code in system, different from currency name if multiple chains are supported
    pub coin: String,
    // the amount to withdraw
    pub amount: f64,
    // a crypto address of the recipient
    pub address: String,
    // memo(Optional), It is usually necessary for EOS
    pub memo: String,
    // 	withdraw ticket
    pub ticket: String,
}

#[derive(Debug, Serialize)]
pub struct TransferBetweenVenuesParams {
    // currency name
    pub currency: String,
    // the amount to withdraw
    pub amount: f64,
    // 	name of target venue
    #[serde(rename(serialize = "targetVenue"))]
    pub target_venue: String,
}

#[derive(Debug, Serialize)]
pub struct QueryAccountHistoryParams {
    #[serde(rename(serialize = "startTimeDate"))]
    pub start_time_date: DateTime<Utc>,
    #[serde(rename(serialize = "endTimeDate"))]
    pub end_time_date: DateTime<Utc>,
    pub limit: i32,
    pub page: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateConversionParams {
    // From currency, support USD or USDC only
    currency: String,
    // To currency, support USD or USDC only
    #[serde(rename(serialize = "targetCurrency"))]
    target_currency: String,
    // the amount to convert
    amount: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderParams {
    // 	order ID(Optional)
    order_id: Option<String>,
    order_info: OrderInfo,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    symbol: String,
    // order type, LIMIT or MARKET(only some venues support) or STOP or SOR(smart order router)
    order_type: String,
    // specifies how long the order remains in effect, 1=GTC, 3=IOC=, 7=PostOnly
    time_in_force: i32,
    // order side, BUY or SELL
    order_side: String,
    // limit price
    limit_price: String,
    // quantity, order size
    quantity: String,
    // Amount of quote asset to spend, required when orderSide is BUY, orderType = MARKET
    total: Option<String>,
    // Trigger price at which when the last trade price changes to this value, the stop order will be triggered; Required if orderType = STOP
    trigger_price: Option<String>,
    // ENTRY or LOSS; entry is for when price rises above triggerPrice, loss is for when price drops below triggerPrice; Required if orderType = STOP
    stop_type: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    account_id: String,
    ex55_pin: String,
    account_status: String,
    #[serde(with = "ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    updated_at: DateTime<Utc>,
    sub_account_info: Vec<SubAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccount {
    account_id: String,
    account_status: String,
    #[serde(with = "ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    updated_at: DateTime<Utc>,
    venue: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    account_id: String,
    venue: String,
    currency: String,
    amount: f64,
    available: f64,
    frozen: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    venue: String,
    currency: String,
    coin: String,
    address: String,
    #[serde(with = "ts_milliseconds")]
    expired_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawTicket {
    ticket: String,
    #[serde(with = "ts_milliseconds")]
    expired_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    account_id: String,
    venue: String,
    currency: String,
    coin: String,
    amount: f64,
    fee: f64,
    from_address: Value,
    target_address: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    action_type: String,
    action_note: String,
    log_id: String,
    tx_id: String,
    // SUBMITTED, COMPLETED, CANCELLED
    status: String,
    #[serde(with = "ts_milliseconds")]
    log_created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    log_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferBetweenVenuesResponse {
    account_id: String,
    venue: String,
    // currency name
    currency: String,
    // the amount to withdraw
    amount: f64,
    // fee of withdrawal
    fee: f64,
    // target venue name
    target_venue: String,
    // internal transfer ID
    log_id: String,
    // SUBMITTED, COMPLETED, CANCELLED
    status: String,
    #[serde(with = "ts_milliseconds")]
    log_created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    log_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagationResponse<T> {
    records: Vec<T>,
    total: i32,
    size: i32,
    current: i32,
    hit_count: bool,
    search_count: bool,
    pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistory {
    account_id: String,
    currency: String,
    // SUBMITTED, COMPLETED, CANCELLED
    status: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    amount: f64,
    fee: f64,
    target_address: String,
    coin: String,
    log_id: String,
    #[serde(with = "ts_milliseconds")]
    log_created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    log_updated_at: DateTime<Utc>,
    action_type: String,
    action_note: String,
    from_account_id: String,
    to_account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    account_id: String,
    trading_volume: f64,
    take_fee: f64,
    make_fee: f64,
    special_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversionResponse {
    // Account ID
    account_id: String,
    // From currency
    currency: String,
    // status of this account activity, SUBMITTED, COMPLETED, CANCELLED
    status: String,
    // OUTGOING for convert
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    // amount	amount of currency converted
    amount: f64,
    // fee	fee occurs during this activity
    fee: f64,
    // coin	internal coin name for this currency
    coin: String,
    // logId	internal ID
    log_id: String,
    #[serde(with = "ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    updated_at: DateTime<Utc>,
    action_type: String,
    action_note: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    // 	sub account ID
    account_id: String,
    // 	venue name
    venue: String,
    // 	order ID
    order_id: String,
    // symbol
    symbol: String,
    // order type, LIMIT or MARKET or STOP or SOR
    order_type: String,
    // order side, BUY or SELL
    order_side: String,
    // Trigger price
    trigger_price: Option<f64>,
    // ENTRY or LOSS
    stop_type: Option<String>,
    // stop order activated timestamp
    trigger_time: Option<DateTime<Utc>>,
    // limit price, not available for market order
    limit_price: Option<f64>,
    // quantity
    quantity: f64,
    // average fill price
    filled_average_price: f64,
    // accumulated fill quantity
    filled_cumulative_quantity: f64,
    // open quantity to be filled
    open_quantity: f64,
    // order status
    order_status: String,
    // order creation timestamp
    created_at: DateTime<Utc>,
    // order update timestamp
    updated_at: Option<DateTime<Utc>>,
    // if it is cancelled, cancellation timestamp
    cancelled_updated_at: Option<DateTime<Utc>>,
    // last filled timestamp
    filled_updated_at: Option<DateTime<Utc>>,
    total: Option<f64>,
}
