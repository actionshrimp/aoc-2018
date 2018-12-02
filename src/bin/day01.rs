use std::ops::Neg;

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

fn main() {
    let fname = "data/01.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    println!("result p1: {}", part1(&fdata));
}
