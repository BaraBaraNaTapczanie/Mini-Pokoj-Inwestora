use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::{Stream, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const API_KEY: &str = "pg1XbIBTiRC5WZaWrDuMQzSbphnchVOM";

const TICKERS: [&str; 14] = [
    "I:NDX",
    "TSLA",
    "AAPL",
    "MSFT",
    "X:BTCUSD",
    "GOOGL",
    "NVDA",
    "X:ETHUSD",
    "X:DOGEUSD",
    "AMZN",
    "UBER",
    "NFLX",
    "DELL",
    "F",
];

#[derive(Serialize, Deserialize, Debug)]
struct TickerEntry {
    pub symbol: String,
    pub open: f32,
    pub close: f32,
}

#[get("/")]
async fn hello() -> impl Responder {
    let url = format!(
        "{}{}",
        "https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/day/2025-01-09/2025-01-10?apiKey=",
        API_KEY
    );
    let res = reqwest::get(url).await.unwrap();
    let ss = res.json::<serde_json::Value>().await.unwrap();
    HttpResponse::Ok().json(ss)
}

#[get("/tabledata")]
async fn tabledata() -> impl Responder {
    let yesterday = chrono::Local::now().checked_sub_days(chrono::Days::new(1)).unwrap();
    let data: Vec<_> = futures_util::stream::iter(TICKERS.iter().take(1))
        .then(|symbol| async move {
            let url = format!(
                "https://api.polygon.io/v1/open-close/{}/{}?apiKey={}",
                symbol,
                yesterday.format("%Y-%m-%d"),
                API_KEY
            );
            let res = reqwest::get(url).await.unwrap();
            if res.status() == StatusCode::OK {
                let entry: TickerEntry = res.json().await.unwrap();
                Ok(entry)
            } else {
                Err(res.text().await.unwrap())
            }

            //todo
            // funkcja w tokio co robi co pewien czas (24h) Interval spawni taski do pobieranie tych jebanych danych do tabelki
            
            }).filter(|res| {let ret = res.is_ok(); async move {ret}})
        .try_collect()
        .await.unwrap();
    // let file = File::open("DATA.json").expect("msg");
    // let data: serde_json::Value = serde_json::from_reader(file).expect("cos");
    println!("{:#?}", &data);
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let url = format!(
    //     "{}{}",
    //     "https://api.polygon.io/v3/reference/tickers?&active=true&limit=1000&apiKey=", API_KEY
    // );
    // let res = reqwest::get(url).await.unwrap();
    // let tickers = res
    //     .json::<TickersResponse>()
    //     .await
    //     .map(|result| {
    //         result
    //             .results
    //             .into_iter()
    //             .map(|ticker| ticker.ticker)
    //             .collect::<Vec<_>>()
    //     })
    //     .unwrap();

    // println!("{:?}",tickers);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(hello).service(tabledata)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
