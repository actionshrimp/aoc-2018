// extern crate reqwest;

use std::env;
use std::fs::File;
use std::io::prelude::*;

// use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1]; let dayi = day.parse::<i32>().expect("invalid day");

    let mut f = File::open("config/sessionid").expect("data/sessionid file not found");

    let mut sessionid = String::new();
    f.read_to_string(&mut sessionid)
        .expect("something went wrong reading the file");

    let uri = format!("https://adventofcode.com/2018/day/{}/input", dayi);

    let client = reqwest::Client::new();
    let mut res = client
        .get(&uri)
        .header("Cookie", format!("session={}", &sessionid))
        .send()
        .expect("http error");

    let body = res.text();

    println!("body = {:?}", body);
    // Ok(())
}
