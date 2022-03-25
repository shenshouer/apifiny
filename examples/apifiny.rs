use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conf = apifiny::ApiFiny::parse();
    // let client = apifiny::RestClient::new(conf, None);

    // let venue = apifiny::BINANCE;
    // client.list_symbol_info(venue.name).await?;

    // let _s = client.current_time_millis(venue.name).await?;
    // let s = client.ticker("BTCUSD", venue.name).await?;
    // let s = client.consolidated_order_book("BTCUSD").await?;

    let client = apifiny::RestClient::new(conf, Some(apifiny::BINANCEUS));
    // let s = client.list_balance().await?;
    // let s = client.query_address("USDT.ETH").await?;
    let s = client.create_withdraw_ticket().await?;

    // let create_withdraw_params = apifiny::rest_client::CreateWithdrawParams {
    //     coin: "BTC".to_string(),
    //     amount: 0.0,
    //     address: "3Nxwena****************fp8v".to_string(),
    //     memo: "".to_string(),
    //     ticket: "e17a560959454d6a92336af8a7d612cd".to_string(),
    // };
    // let s = client.create_withdraw(create_withdraw_params).await?;

    let json_str: String = serde_json::to_string(&s).unwrap();
    println!("{}", json_str);

    Ok(())
}
