// #[macro_use]
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

struct Claim {
    id: i32,
    pos: (i32, i32),
    size: (i32, i32)
}

fn cap_int(cap: &regex::Captures, name: &str) -> i32 {
    cap.name(name)
        .expect(&format!("{} cap", name))
        .as_str()
        .parse::<i32>()
        .expect(&format!("{} parse", name))
}

fn parse_line(re: &Regex, s: &str) -> Claim {
    re.captures(s).map(|caps| {
        Claim {
            id: cap_int(&caps, "id"),
            pos: (cap_int(&caps, "posx"), cap_int(&caps, "posy")),
            size: (cap_int(&caps, "sizex"), cap_int(&caps, "sizey"))
        }
    }).expect(&format!("error parsing line: {}", s))
}

fn to_coords(c: &Claim) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for x in c.pos.0 .. (c.pos.0 + c.size.0) {
        for y in c.pos.1 .. (c.pos.1 + c.size.1) {
            res.push((x, y));
        }
    }
    res
}

fn part1(claims: &Vec<Claim>) -> i32 {
    let mut fabric : HashMap<(i32, i32), i32> = HashMap::new();
    for claim in claims {
        let mut coords = to_coords(claim);
        for coord in coords {
            let count = fabric.entry(coord).or_insert(0);
            *count += 1;
        }
    }

    fabric.values().fold(0, |acc, v| {
        if *v > 1 {
            acc + 1
        } else {
            acc
        }
    })

}

fn main() {
    let re: Regex = Regex::new(r"^#(?P<id>\d+) @ (?P<posx>\d+),(?P<posy>\d+): (?P<sizex>\d+)x(?P<sizey>\d+)$").expect("error compiling regex");
    let fname = "data/03.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let claims = fdata.lines().map(|l| parse_line(&re, l)).collect();

    println!("result p1: {}", part1(&claims));
}
