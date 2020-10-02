#![feature(label_break_value)]

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error;
use reqwest::Response;
use sha2::Digest;
use std::collections::HashMap;
use std::env;
use std::future::Future;

const ALPACA_ENDPOINT: &str = "https://paper-api.alpaca.markets";
const ALPACA_API_KEY_ID: &str = "APCA-API-KEY-ID";
const ALPACA_SECRET_KEY: &str = "APCA-API-SECRET-KEY";
const ALPACA_ENDPOINT_ACCOUNT: &str = "https://paper-api.alpaca.markets/v2/account";

fn main() {
    println!(
        "ALPACA_API_KEY_ID: {:?}",
        env::var_os(ALPACA_SECRET_KEY).unwrap()
    );
    println!(
        "ALPACA_SECRET_KEY: {:?}",
        env::var_os(ALPACA_API_KEY_ID).unwrap()
    );

    let res = get_account_info();

   // println!("Response: {:?}", res.await);
    //println!("Response text: {}", res.is_ok().text().unwrap());
}

async fn get_account_info() -> Response {
    let client = reqwest::Client::new();

    let mut headers = 'a: {
        let mut headers = HeaderMap::new();

        let mut header_val_1 =
            HeaderValue::from_str(env::var_os(ALPACA_API_KEY_ID).unwrap().to_str().unwrap());
        let mut header_val_2 =
            HeaderValue::from_str(env::var_os(ALPACA_SECRET_KEY).unwrap().to_str().unwrap());

        headers.insert(ALPACA_API_KEY_ID, header_val_1.unwrap());
        headers.insert(ALPACA_SECRET_KEY, header_val_2.unwrap());

        headers
    };

    let mut res = client
        .get(ALPACA_ENDPOINT_ACCOUNT)
        .headers(headers)
        .send()
        .await;

    res.unwrap()
}
