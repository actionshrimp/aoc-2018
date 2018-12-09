extern crate combine;

use combine::{Parser, ParseError, Stream};
use combine::parser::char::{char, digit};
use combine::parser::repeat::*;
use combine::{token};

enum EventEntry {
    Begin(i32),
    FallsAsleep,
    WakesUp
}

// [1518-05-30 00:04] Guard #2417 begins shift
// [1518-10-20 00:48] wakes up
// [1518-08-12 00:14] falls asleep

fn parse_line(line: &str) -> (i32, EventEntry) {

    let minute = take_until::<String,_>(token(':'))
        .with(char(':'))
        .with(many1::<String, _>(digit()))
        .map(|s| { s.parse::<i32>().unwrap() });

    let begin = combine::parser::char::string(" Guard #")
        .with(many1::<String, _>(digit())
              .map(|s| {s.parse::<i32>().unwrap()})
        ).map(|id : i32| { EventEntry::Begin(id) });

    let event = combine::choice((
        begin,
        combine::parser::char::string(" wakes up").map(|_| { EventEntry::WakesUp }),
        combine::parser::char::string(" falls asleep").map(|_| { EventEntry::FallsAsleep })
    ));

    let mut parser = combine::between(combine::token('['), combine::token(']'), minute)
        .and(event);

    let (result, _rest) = parser.parse(line).expect(&format!("parse failed: {}", &line));

    result
}


fn main() {
    let fname = "data/04.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let mut lines : Vec<_> = fdata.lines().collect();
    lines.sort();

    let first = lines.first().expect("xxx");
    println!("{}", &first);
    parse_line(&first);
    // let parsed = &lines.iter().map(|line| parse_line(&line));

    // println!("result p1: {}", part1(&fabric));
    // println!("result p2: {}", part2(&fabric, &claims));
}
