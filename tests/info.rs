use bybit_rust_sdk::Bybit;
use bybit_rust_sdk::Category;

#[tokio::test]
pub async fn test_get_instrument_info() {
    let bybit = Bybit::new(None, None, None).unwrap();

    let map = bybit.get_instrument_info(Category::Linear, Some("ZORAUSDT")).await.unwrap();
    dbg!(map);
}

#[tokio::test]
pub async fn test_get_instrument_all_info() {
    let bybit = Bybit::new(None, None, None).unwrap();

    let map = bybit.get_instrument_info(Category::Linear, None).await.unwrap();
    dbg!(map.len());
    dbg!(map.iter().last().unwrap());
}

#[tokio::test]
pub async fn test_get_funding_info() {
    let bybit = Bybit::new(None, None, None).unwrap();

    let funding_interval =  240;
    let map = bybit.get_futures_tickers(Some("ZORAUSDT")).await.unwrap();
    dbg!(&map);

    let f = map.iter().last().unwrap().1.annualized_funding(funding_interval);
    dbg!(f);
}

#[tokio::test]
pub async fn test_get_all_funding_info() {
    let bybit = Bybit::new(None, None, None).unwrap();

    let funding_interval =  240;
    let map = bybit.get_futures_tickers(None).await.unwrap();
    let json = serde_json::to_string_pretty(&map).unwrap();
    std::fs::write("ftickers.json", &json).unwrap();

    //let f = map.iter().last().unwrap().1.annualized_funding(funding_interval);
    //dbg!(f);
}