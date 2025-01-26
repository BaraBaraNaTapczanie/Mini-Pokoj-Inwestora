use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::{lock::Mutex, task, Stream, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::time;
use std::{collections::HashMap, sync::Arc, time::Duration};

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

#[derive(Serialize, Deserialize, Debug)]
struct PrevResult{
    pub T: String,
    pub o: f32,
    pub c: f32
}

impl From <PrevResult> for TickerEntry {
    fn from(value: PrevResult) -> Self {
        TickerEntry{symbol: value.T, open: value.o, close: value.c}
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct PrevResponse{
    pub results: Vec<PrevResult>
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
async fn tabledata(data: web::Data<Mutex<HashMap<String, TickerEntry>>>) -> impl Responder {
    let lock = data.lock().await;
    let values = lock.iter().map(|(k, v)| v.to_owned()).collect::<Vec<_>>();
    HttpResponse::Ok().json(values)
}

async fn update(data: &mut Arc<Mutex<HashMap<String, TickerEntry>>>) {  
    for x in TICKERS.into_iter() {
        let mut lock = data.lock().await;  

        let url = format!(
            "https://api.polygon.io/v2/aggs/ticker/{}/prev?apiKey={}",
            x,
            API_KEY
        );

        let res: reqwest::Response = reqwest::get(url).await.unwrap();
        let mut entry: PrevResponse = res.json().await.unwrap();
        let ticker_entry: TickerEntry = entry.results.pop().unwrap().into();
        lock.insert(x.to_owned(), ticker_entry);
        drop(lock);
        tokio::time::sleep(Duration::from_secs(13)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data: Arc<Mutex<HashMap<String, TickerEntry>>> = Arc::new(Mutex::new(HashMap::new()));

    let data_clone = data.clone();

    let update_tabledata = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(70));
        let mut data = data_clone;

        loop {
            interval.tick().await;
            update(&mut data).await;
        }
    });

    let data = web::Data::from(data);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new().app_data(data.clone()).wrap(cors).service(hello).service(tabledata)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
