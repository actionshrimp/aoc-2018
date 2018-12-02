// extern crate reqwest;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args
        .get(1)
        .expect("no day arg passed")
        .parse::<i32>()
        .expect("invalid day (expected integer)");

    let data_path = format!("data/{:02}.txt", day);
    let fexists = Path::new(&data_path).exists();

    if fexists {
        println!("{} already exists", &data_path);

    } else {
        let mut f = File::open("config/session_id").expect("data/session_id file not found");

        let mut session_id = String::new();
        f.read_to_string(&mut session_id)
            .expect("something went wrong reading the file");

        let uri = format!("https://adventofcode.com/2018/day/{}/input", day);

        let client = reqwest::Client::new();
        let mut res = client
            .get(&uri)
            .header("Cookie", format!("session={}", &session_id))
            .send()
            .expect("http error");

        let body = res.text().expect("something went wrong reading the body");

        let mut f = File::create(&data_path)
            .expect(&format!("couldnt create file: {}", data_path));

        f.write_all(&body.as_bytes()).expect("error writing contents");

        println!("{} written", &data_path);
    }
}
