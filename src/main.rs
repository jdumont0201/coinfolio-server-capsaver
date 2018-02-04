
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


use std::collections::HashMap;
use std::fs::File;

use std::thread;
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use rand::Rng;

static DB_ADDRESS:&str="http://0.0.0.0:3000";

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
    impl Data{
        fn to_json(&self) -> String {
            let tsi:i64=self.last_updated.parse::<i64>().unwrap();
            let ts = super::chrono::Utc.timestamp(tsi , 0).format("%Y-%m-%d %H:%M:%S");
            let maxsup;let s;
            match self.max_supply{
                Some(ref ma)=>{
                    maxsup=ma.to_string();
                    s = format!(r#"{{"ts" :"{}","symbol"  :"{}","marketcap"  :"{}","supply":"{}","maxsupply":"{}"}}"#, ts, self.symbol,self.market_cap_usd,self.total_supply,maxsup);
                },None=>{

                    s = format!(r#"{{"ts" :"{}","symbol"  :"{}","marketcap"  :"{}","supply":"{}"}}"#, ts, self.symbol,self.market_cap_usd,self.total_supply);
                }
            };

            s
        }
    }
    fn parse(text: &str) {}

    pub fn save_coinmarketcap(client: &reqwest::Client,  data: Data) {
        let tsi=data.last_updated.parse::<i64>().unwrap();
        let ts = chrono::Utc.timestamp(tsi , 0).format("%Y-%m-%d %H:%M:%S");
        let json = data.to_json();
        let uriexists = format!("{}/cmc_cap?symbol=eq.{}&ts=eq.'{}'",super::DB_ADDRESS, data.symbol,ts);
        println!("save {}",json);
        if let Ok(mut res) = reqwest::get(&uriexists) {
            //println!("get ok");
            let getres = match res.text() {
                Ok(val) => {
              //      println!("getres {}",val);
                    if val.len() > 2 { //already exists, do nothing

                    } else {
                        let uri = format!("{}/cmc_cap",super::DB_ADDRESS);
                //        println!("post {}",json);

                        if let Ok(mut res) = client.post(&uri).body(json).send() {

                            let st = res.status();
                  //          println!("post st {}",st);
                            //println!("[{}] [POST] {}_ohlc_1m {} {}", pp.to_string(), bb.to_string(), res.status(), res.text().unwrap());
                            if st == hyper::StatusCode::Conflict {//existing
                                //        println!("[{}] [POST] {}_ohlc_1m {} {}", pp.to_string(), bb.to_string(), res.status(), res.text().unwrap());
                            } else if st == hyper::StatusCode::Created {//created
                                //          println!("[{}] [POST] {}_ohlc_1m {} {}", pp.to_string(), bb.to_string(), res.status(), res.text().unwrap());
                            } else {
                                //            println!("[{}] [POST] {}_ohlc_1m {} {}", pp.to_string(), bb.to_string(), res.status(), res.text().unwrap());
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("[GET_CMC] !!cmc_cap existing? {} ", err);
                }
            };
        } else {
            println!("[GET] nok uri {}",uriexists);
        }
    }
}


fn fetch_and_save_cmc() {
    println!(" -> CMC market cap");
    let client = reqwest::Client::new();
    let uri = "https://api.coinmarketcap.com/v1/ticker/";
    if let Ok(mut res) = client.get(uri).send() {
        println!("[GET] {} ", res.status());
        let result = match res.text() {
            Ok(text) => {
                let data: Vec<CoinMarketCap::Data> = serde_json::from_str(&text).unwrap();
                for d in data {
                    CoinMarketCap::save_coinmarketcap(&client,d);
                }
            }
            Err(err) => {
                println!(" [GET_CAP] cap ERR !!!  {}",  err);
            }
        };
    }
}


fn main() {
    println!("Coinamics Server Cap saver");
    let mut children = vec![];
  
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
    for child in children {
        let _ = child.join();
    }
}