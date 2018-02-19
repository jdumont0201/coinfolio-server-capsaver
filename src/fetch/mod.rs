use reqwest;
use debug;
pub fn fetch_cmc() -> Option<String> {
    let client = reqwest::Client::new();
    let uri = "https://api.coinmarketcap.com/v1/ticker/";
    debug::print_fetch(uri.to_string());
    match client.get(uri).send() {
        Ok(mut res) => {
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

pub fn fetch_cmc_global() -> Option<String> {
    let client = reqwest::Client::new();
    let uri = "https://api.coinmarketcap.com/v1/global/";
    debug::print_fetch(uri.to_string());
    match client.get(uri).send() {
        Ok(mut res) => {
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