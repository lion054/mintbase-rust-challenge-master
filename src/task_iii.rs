pub async fn get_stock_prices(tickers: Vec<String>) -> Vec<(String, f64)> {
    println!("start");
    let mut result = Vec::with_capacity(500);
    let (tx, mut rx) = tokio::sync::mpsc::channel(500);
    for ticker in tickers {
        println!("ticker");
        let tx_clone = tx.clone();
        let yahoo = yahoo_finance_api::YahooConnector::new();
        tokio::spawn(async move {
            println!("call");
            if let Ok(response) = yahoo.get_latest_quotes(&ticker, "1m").await {
                let res = (ticker, response.last_quote().unwrap().close);
                println!("tx");
                tx_clone.send(res).await;
            }
        });
    }
    while let res = rx.recv().await {
        println!("rx");
        result.push(res.unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn make_it_fly() {
        let tickers = std::fs::read_to_string("./sp500.csv")
            .unwrap()
            .lines()
            .skip(1)
            .take(500)
            .map(|line| line.split_once(',').unwrap().0.to_string())
            .collect();

        let before = chrono::Local::now();
        assert!(
            get_stock_prices(tickers).await.len() > 485,
            "Did you lower the number of fetched prices?"
        );
        let elapsed = chrono::Local::now() - before;
        assert!(
            elapsed < chrono::Duration::seconds(30),
            "Your target is to get below 30 seconds, it took: {} s",
            elapsed.num_seconds()
        );
    }
}
