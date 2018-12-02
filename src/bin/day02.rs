use std::collections::HashMap;
use std::iter::Iterator;

fn counts(s: &str) -> HashMap<char, i8> {
    let mut counts = HashMap::new();
    for c in s.chars() {

        let count = counts.entry(c).or_insert(0);
        *count += 1;
    };

    counts
}

fn has(c : &HashMap<char, i8>, j : i8) -> bool {
    match c.iter().find(|(_, &i)| j == i) {
        | None => false,
        | Some(_) => true
    }
}

fn part1(s: &str) -> i32 {
    let (twos, threes) = s.lines().fold((0, 0), |(twos, threes), line| {
        let c = counts(line);

        match (has(&c, 2), has(&c, 3)) {
            | (true, true) => (twos+1, threes+1),
            | (true, false) => (twos+1, threes),
            | (false, true) => (twos, threes+1),
            | (false, false) => (twos, threes),
        }
    });

    twos * threes
}

fn compare_ids(s: &str, s2: &str) -> i8 {
    s.chars().zip(s2.chars()).fold(0, |acc, (c1, c2)| {
        if c1 != c2 {
            acc + 1
        } else {
            acc
        }
    })
}

fn common(s: &str, s2: &str) -> String {
    let mut result = String::new();

    s.chars().zip(s2.chars()).for_each(|(c1, c2)| {
        if c1 == c2 {
            result.push(c1)
        }
    });

    result
}

fn part2(s: &str) -> String {
    let lines : Vec<&str> = s.lines().collect();
    let mut i = 0;

    let (item1, item2) = loop {
        let item = lines[i];
        let rest = &lines[(&i+1)..];

        let search = rest.iter().find(|item2| {
            compare_ids(item, item2) == 1
        });

        match search {
            | None => i += 1,
            | Some (item2) => break (item, item2)
        }
    };

    common(item1, item2)
}

fn main() {
    let fname = "data/02.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    println!("result p1: {}", part1(&fdata));
    println!("result p2: {}", part2(&fdata));
}
