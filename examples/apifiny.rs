use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conf = apifiny::ApiFiny::parse();
    let client = apifiny::RestClient::new(conf);

    let venue = apifiny::BINANCE;
    // client.list_symbol_info(venue.name).await?;

    let _s = client.current_time_millis(venue.name).await?;
    // let s = client.ticker("BTCUSD", venue.name).await?;
    let s = client.consolidated_order_book("BTCUSD").await?;
    let json_str: String = serde_json::to_string(&s).unwrap();
    println!("{}", json_str);

    Ok(())
}
