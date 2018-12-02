use std::collections::HashMap;

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
    let (twos, threes) = s.lines().fold((0, 0), |(twos,threes), line| {
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

fn main() {
    let fname = "data/02.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    println!("result p1: {}", part1(&fdata));
}
