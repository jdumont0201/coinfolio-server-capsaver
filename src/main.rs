#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate serde;
extern crate serde_json;
extern crate json;
extern crate clap;
extern crate ansi_term;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate reqwest;
extern crate job_scheduler;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate rand;
extern crate colored;
mod debug;

use std::collections::HashMap;
use std::fs::File;

use std::thread;
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use rand::Rng;

static DB_ADDRESS: &str = "http://0.0.0.0:3000";

fn parsei64(i: &String) -> i64 {
    i.parse::<i64>().unwrap()
}

fn parsef64(i: &String) -> f64 {
    i.parse::<f64>().unwrap()
}

fn concat(a: &str, b: &str) -> String {
    let mut owned_str: String = "".to_owned();
    owned_str.push_str(a);
    owned_str.push_str(b);
    owned_str
}

mod CoinMarketCap {
    use reqwest;
    use chrono;
    use hyper;
    use chrono::prelude::*;
    use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

    #[derive(Serialize, Deserialize)]
    pub struct Data {
        id: String,
        name: String,
        symbol: String,
        rank: String,
        price_usd: String,
        price_btc: String,
        // 24h_volume_usd: String,
        market_cap_usd: String,
        available_supply: String,
        total_supply: String,
        max_supply: Option<String>,
        percent_change_1h: String,
        percent_change_24h: String,
        percent_change_7d: String,
        last_updated: String,
    }

    impl Data {
        fn to_json(&self) -> String {
            let tsi: i64 = self.last_updated.parse::<i64>().unwrap();
            let ts = super::chrono::Utc.timestamp(tsi, 0).format("%Y-%m-%d %H:%M:%S");
            let maxsup;
            let s;
            match self.max_supply {
                Some(ref ma) => {
                    maxsup = ma.to_string();
                    s = format!(r#"{{"ts" :"{}","symbol"  :"{}","marketcap"  :"{}","supply":"{}","maxsupply":"{}"}}"#, ts, self.symbol, self.market_cap_usd, self.total_supply, maxsup);
                }
                None => {
                    s = format!(r#"{{"ts" :"{}","symbol"  :"{}","marketcap"  :"{}","supply":"{}"}}"#, ts, self.symbol, self.market_cap_usd, self.total_supply);
                }
            };

            s
        }
    }


    #[derive(Serialize, Deserialize)]
    pub struct GlobalData {
        total_market_cap_usd: f64,
        total_24h_volume_usd: f64,
        bitcoin_percentage_of_market_cap: f64,
        active_currencies: f64,
        active_assets: f64,
        active_markets: f64,
        last_updated: i64
    }

    impl GlobalData {
        fn to_json(&self) -> String {
            let tsi: i64 = self.last_updated;
            let ts = super::chrono::Utc.timestamp(tsi, 0).format("%Y-%m-%d %H:%M:%S");
            let s;
            s = format!(r#"{{"ts" :"{}","symbol"  :"GLOBAL","marketcap"  :"{}"}}"#, ts, self.total_market_cap_usd);

            s
        }
    }

    fn parse(text: &str) {}

    pub fn save_coinmarketcap(client: &reqwest::Client, data: Data) {
        let tsi = data.last_updated.parse::<i64>().unwrap();
        let ts = chrono::Utc.timestamp(tsi, 0).format("%Y-%m-%d %H:%M:%S");
        let json = data.to_json();
        let uriexists = format!("{}/cmc_cap?symbol=eq.{}&ts=eq.'{}'", super::DB_ADDRESS, data.symbol, ts);
        if let Ok(mut res) = reqwest::get(&uriexists) {
            let getres = match res.text() {
                Ok(val) => {
                    if val.len() > 2 {} else {
                        let uri = format!("{}/cmc_cap", super::DB_ADDRESS);
                        if let Ok(mut res) = client.post(&uri).body(json).send() {
                            let st = res.status();
                            if st == hyper::StatusCode::Conflict {} else if st == hyper::StatusCode::Created {} else {}
                        }
                    }
                }
                Err(err) => {
                    println!("[GET_CMC] !!cmc_cap existing? {} ", err);
                }
            };
        } else {
            println!("[GET] nok uri {}", uriexists);
        }
    }

    pub fn save_coinmarketcap_global(client: &reqwest::Client, data: GlobalData) {
        let tsi = data.last_updated;
        let ts = chrono::Utc.timestamp(tsi, 0).format("%Y-%m-%d %H:%M:%S");
        let json = data.to_json();
        let uriexists = format!("{}/cmc_cap?symbol=eq.{}&ts=eq.'{}'", super::DB_ADDRESS, "GLOBAL".to_string(), ts);
        if let Ok(mut res) = reqwest::get(&uriexists) {
            let getres = match res.text() {
                Ok(val) => {
                    if val.len() > 2 {} else {
                        let uri = format!("{}/cmc_cap", super::DB_ADDRESS);
                        if let Ok(mut res) = client.post(&uri).body(json).send() {
                            let st = res.status();
                            if st == hyper::StatusCode::Conflict {} else if st == hyper::StatusCode::Created {} else {}
                        }
                    }
                }
                Err(err) => {
                    println!("[GET_CMC] !!cmc_cap existing? {} ", err);
                }
            };
        } else {
            println!("[GET] nok uri {}", uriexists);
        }
    }

    pub fn save_global(client: &reqwest::Client, data: GlobalData) {
        let tsi = data.last_updated;
        let ts = chrono::Utc.timestamp(tsi, 0).format("%Y-%m-%d %H:%M:%S");
        let json = data.to_json();
        let uri = format!("{}/cmc_cap_global", super::DB_ADDRESS);
        if let Ok( res) = client.post(&uri).body(json).send() {
            let st = res.status();
            if st == hyper::StatusCode::Conflict {
                //existing
            } else if st == hyper::StatusCode::Created {
                //created
            } else {}
        }
    }
}

fn fetch_cmc() -> Option<String> {
    let client = reqwest::Client::new();
    let uri = "https://api.coinmarketcap.com/v1/ticker/";
    debug::print_fetch(uri.to_string());
    match client.get(uri).send() {
        Ok(mut res) => {
            println!("[GET] {} ", res.status());
            match res.text() {
                Ok(text) => { Some(text) }
                Err(err) => {
                    println!(" [GET_CAP] cap ERR !!!  {}", err);
                    None
                }
            }
        }
        Err(err) => {
            println!(" [GET_CAP] cap ERR !!!  {}", err);
            None
        }
    }
}

fn fetch_cmc_global() -> Option<String> {
    let client = reqwest::Client::new();
    let uri = "https://api.coinmarketcap.com/v1/global/";
    debug::print_fetch(uri.to_string());
    match client.get(uri).send() {
        Ok(mut res) => {
            println!("[GET] {} ", res.status());
            match res.text() {
                Ok(text) => { Some(text) }
                Err(err) => {
                    println!(" [GET_CAP] cap ERR !!!  {}", err);
                    None
                }
            }
        }
        Err(err) => {
            println!(" [GET_CAP] cap ERR !!!  {}", err);
            None
        }
    }
}

fn fetch_and_save_cmc() {
    println!(" -> CMC market cap");
    let client = reqwest::Client::new();
    let fetchRes = fetch_cmc();
    match fetchRes {
        Some(text) => {
            let data: Result<Vec<CoinMarketCap::Data>,serde_json::Error> = serde_json::from_str(&text);
            match data {
                Ok(data_) => {
                    for d in data_ {
                        CoinMarketCap::save_coinmarketcap(&client, d);
                    }
                }
                Err(err) => {
                    debug::err(format!("fetch_and_save_cmc {}",err))
                }
            }
        }
        None => {
            debug::err(format!("fetch_and_save_cmc no fetch"));
            ;
        }
    };
}

fn fetch_and_save_global_cmc() {
    println!(" -> CMC market cap");

    let client = reqwest::Client::new();
    let fetchRes = fetch_cmc_global();
    match fetchRes {
        Some(text) => {
            let data: Result<CoinMarketCap::GlobalData,serde_json::Error> = serde_json::from_str(&text);
            match data {
                Ok(data_) => {
                    CoinMarketCap::save_coinmarketcap_global(&client, data_);
                }
                Err(err) => {
                    debug::err(format!("fetch_and_save_cmc_global {}",err))
                }
            }
        }
        None => {
            debug::err(format!("fetch_and_save_cmc_global no fetch"));
        }
    };
}


fn main() {
    println!("Coinamics Server Cap saver");
    let mut children = vec![];

    //CoinMarketCap Crypto Cap
    children.push(thread::spawn(move || {
        println!("Starting CMC  threads");
        let mut sched = job_scheduler::JobScheduler::new();
        sched.add(job_scheduler::Job::new("30 1,6,11,16,21,26,31,36,41,46,51,56 * * * *".parse().unwrap(), || {
            let delay = rand::thread_rng().gen_range(0, 10);
            thread::sleep(std::time::Duration::new(delay, 0));
            fetch_and_save_cmc();
        }));
        loop {
            sched.tick();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }));



    //CoinMarketCap Global Cap
    children.push(thread::spawn(move || {
        println!("Starting CMC  threads");
        let mut sched = job_scheduler::JobScheduler::new();
        sched.add(job_scheduler::Job::new("30 1,6,11,16,21,26,31,36,41,46,51,56 * * * *".parse().unwrap(), || {
            let delay = rand::thread_rng().gen_range(0, 10);
            thread::sleep(std::time::Duration::new(delay, 0));
            fetch_and_save_global_cmc();
        }));
        loop {
            sched.tick();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }));
    for child in children {
        let _ = child.join();
    }
}