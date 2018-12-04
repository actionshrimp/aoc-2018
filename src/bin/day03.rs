// #[macro_use]
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Sub;

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

fn part1(fabric: &HashMap<(i32, i32), Vec<i32>>) -> i32 {
    fabric.values().fold(0, |acc, v| {
        if v.len() > 1 {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2(fabric: &HashMap<(i32, i32), Vec<i32>>, claims: &Vec<Claim>) -> i32 {

    let mut overlapping_ids : HashSet<i32> = HashSet::new();
    for square_ids in fabric.values() {
        for id in square_ids.iter() {
            if square_ids.len() > 1 {
                overlapping_ids.insert(*id);
            }
        }
    }

    let all_ids : HashSet<i32> = HashSet::from_iter(claims.iter().map(|c| c.id));

    let sub = all_ids.sub(&overlapping_ids);

    if sub.len() == 1 {
        match sub.iter().nth(0) {
            | None => panic!("no elements"),
            | Some(x) => *x,
        }
    } else {
        panic!("more than one element");
    }
}

fn scan(claims: &Vec<Claim>) -> HashMap<(i32, i32), Vec<i32>> {
    let mut fabric : HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for claim in claims {
        let mut coords = to_coords(claim);
        for coord in coords {
            let sq_claims = fabric.entry(coord).or_insert(Vec::new());
            sq_claims.push(claim.id)
        }
    }
    fabric
}

fn main() {
    let re: Regex = Regex::new(r"^#(?P<id>\d+) @ (?P<posx>\d+),(?P<posy>\d+): (?P<sizex>\d+)x(?P<sizey>\d+)$").expect("error compiling regex");
    let fname = "data/03.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let claims = fdata.lines().map(|l| parse_line(&re, l)).collect();
    let fabric = scan(&claims);

    println!("result p1: {}", part1(&fabric));
    println!("result p2: {}", part2(&fabric, &claims));
}
