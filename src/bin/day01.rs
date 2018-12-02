use std::ops::Neg;
use std::collections::HashSet;

fn part1(data : &str) -> i32 {
    data.lines().fold(0, |acc, line| {
        acc + parse_line(line)
    })
}

fn parse_line(s : &str) -> i32 {
    let out = s.get(0..1).and_then(|sign| {
        s.get(1..).and_then(|rest| {
            rest.parse::<i32>().ok().map(|i| {
                if sign == "+" {
                    i
                } else {
                    i.neg()
                }
            })
        })
    });

    match out {
        | Some(x) => x,
        | None => panic!("couldn't parse {}", s)
    }
}

fn part2(data : &str) -> i32 {

    let mut acc = 0;
    let mut seen : HashSet<i32> = HashSet::new();
    seen.insert(acc);

    let mut lines = data.lines().map(parse_line).cycle();

    loop {
        acc += lines.next().expect("no lines left!");

        if seen.contains(&acc) {
            break acc
        }

        seen.insert(acc);
    }
}

fn main() {
    let fname = "data/01.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    println!("result p1: {}", part1(&fdata));
    println!("result p2: {}", part2(&fdata));
}
