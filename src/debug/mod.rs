
use colored::*;


pub fn print_fetch( url: &String) {
    println!("{}", format!("fetch {}", url).yellow());
}
pub fn err(msg: String) {
    println!("{}", msg.yellow());
}

pub fn warn(msg: String) {
    println!("{}", msg.red());
}

pub fn verb(msg: String) {
    println!("{}", msg);
}
