extern crate combine;

use combine::{Parser};
use combine::parser::char::{char, digit};
use combine::parser::repeat::*;
use combine::{token};
use std::collections::HashMap;
use std::iter::*;

// [1518-05-30 00:04] Guard #2417 begins shift
// [1518-10-20 00:48] wakes up
// [1518-08-12 00:14] falls asleep
enum EventEntry {
    Begin(i32),
    FallsAsleep,
    WakesUp
}

fn parse_line(line: &str) -> (i32, EventEntry) {

    let minute = take_until::<String,_>(token(':'))
        .with(char(':'))
        .with(many1::<String, _>(digit()))
        .map(|s| { s.parse::<i32>().unwrap() });

    let begin = combine::parser::char::string("Guard #")
        .with(many1::<String, _>(digit())
              .map(|s| {s.parse::<i32>().unwrap()})
        ).map(|id : i32| { EventEntry::Begin(id) });

    let event = combine::choice((
        begin,
        combine::parser::char::string("falls asleep").map(|_| { EventEntry::FallsAsleep }),
        combine::parser::char::string("wakes up").map(|_| { EventEntry::WakesUp })
    ));

    let mut parser = combine::between(combine::token('['), combine::token(']'), minute)
        .skip(char(' '))
        .and(event);

    let (result, _rest) = parser.parse(line)
        .expect(&format!("parse failed: {}", &line));

    result
}

struct GuardSleep {
    id: i32,
    start_min: i32,
    end_min: i32
}

fn part1(es: &Vec<(i32, EventEntry)>) -> (i32, i32) {

    let mut minutes_asleep = HashMap::new();
    let mut current_guard = None;
    let mut start_min = None;
    let mut max_guard = None;

    let mut sleeps : Vec<GuardSleep> = Vec::new();

    for (min, ev) in es {
        match ev {
            | EventEntry::Begin(id) => {
                current_guard = Some(id);
            }
            | EventEntry::FallsAsleep => {
                start_min = Some(min);
            }
            | EventEntry::WakesUp => {
                let current_id = current_guard.expect("No current guard");
                let mut total_mins = minutes_asleep
                    .entry(current_id).or_insert(0);

                *total_mins += min - start_min.expect("No start min");
                sleeps.push(GuardSleep { id: *current_id,
                                         start_min: *start_min.expect("No start min"),
                                         end_min: *min });

                match max_guard {
                    | None => { max_guard = Some((current_id, *total_mins)) }
                    | Some((_other_id, other_mins)) => {
                        if *total_mins > other_mins {
                            max_guard = Some((current_id, *total_mins));
                        }
                    }

                }
            }
        }
    }

    let (sleepiest_guard_id, mins) = max_guard.expect("No max guard");
    let all_zeros = (vec![0]).iter().cycle().take(60);
    let sleep_freqs = sleeps.iter().fold(all_zeros, |acc, sleep| {

    });
}

fn main() {
    let fname = "data/04.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let mut lines : Vec<_> = fdata.lines().collect();
    lines.sort();

    let parsed_lines : Vec<_> =
        lines.iter().map(|line| parse_line(&line)).collect();

    println!("result p1: {}", part1(&parsed_lines));
    // println!("result p2: {}", part2(&fabric, &claims));
}
