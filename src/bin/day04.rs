extern crate combine;

use combine::{Parser, ParseError, Stream, StreamOnce};
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

fn minute<I>() -> impl Parser<Input = I, Output = i32>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    take_until::<Vec<_>,_>(token(':'))
        .with(char(':'))
        .with((digit(), digit()))
        .map(|(x, y) : (char, char)| {
            let x = x.to_digit(10).expect("digit");
            let y = y.to_digit(10).expect("digit");
            (x * 10 + y) as i32
        })
}

fn parse_event<I>() -> impl Parser<Input = I, Output = EventEntry>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let begin = combine::parser::char::string(" Guard #")
        .with(many1::<Vec<_>, _>(digit()).map(|d| {
            let s : String = d.iter().collect();
            s.parse::<i32>().unwrap()
        }))
        .map(|id : i32| {
            EventEntry::Begin(id)
        });

    let wakes_up = combine::parser::char::string(" wakes up").map(|_| { EventEntry::WakesUp });
    let falls_asleep = combine::parser::char::string(" falls asleep").map(|_| { EventEntry::FallsAsleep });

    combine::choice((
        begin,
        wakes_up,
        falls_asleep
    ))
}

fn parse_line(line: &str) -> (i32, EventEntry) {

    let mut parser = combine::between(combine::token('['), combine::token(']'), minute())
        .and(parse_event());

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
