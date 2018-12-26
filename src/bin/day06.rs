use std::collections::HashMap;
use std::collections::HashSet;

type Coord = (i32, i32);

fn main() {
    let fname = "data/06.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let mut lines : Vec<&str> = fdata.lines().collect();
    lines.sort();

    let parsed_lines : Vec<Coord> =
        lines.iter().map(|line| parse_line(&line)).collect();

    println!("result p1: {}", part1(&parsed_lines));
    // println!("result p2: {}", part2(&sleeps));
}

fn parse_line(l : &str) -> Coord {
    let mut s = l.split(", ");
    ( s.next().expect("x").parse::<i32>().expect("parse x")
      , s.next().expect("y").parse::<i32>().expect("parse y")
    )
}

#[test]
fn test_parse() {
    let parsed = parse_line("3, 23");
    assert_eq!(3, parsed.0);
    assert_eq!(23, parsed.1);
}

fn part1(coords: &Vec<Coord>) -> i32 {
    let (x0, x1, y0, y1) = grid_corner_coords(coords);

    let cap : usize = ((x1 - x0) * (y1 - y0)) as usize;
    let mut counts : HashMap<&(i32, i32), i32> = HashMap::with_capacity(cap);
    let mut hitting_edge : HashSet<&(i32, i32)> = HashSet::new();

    for x in (x0 - 1)..(x1 + 1) {
        for y in (y0 - 1)..(y1 + 1) {
            match closest(coords, (x, y)) {
                None => {}
                Some(c) => {
                    *counts.entry(c).or_insert(0) += 1;
                    if x == x0 || x == x1 || y == y0 || y == y1 {
                        hitting_edge.insert(c);
                    }
                }
            }
        }
    }

    let mut by_largest = counts.iter().collect::<Vec<_>>();
    by_largest.sort_by(|(_,v1), (_,v2)| {v2.cmp(v1)});

    *by_largest.iter()
        .filter(|(c, _)| { ! hitting_edge.contains(*c) }).nth(0)
        .expect("largest not hitting edge").1
}

fn grid_corner_coords(coords: &Vec<Coord>) -> (i32, i32, i32, i32) {
    let mut x0 = None;
    let mut x1 = None;
    let mut y0 = None;
    let mut y1 = None;

    for (x, y) in coords {
        match x0 { None => { x0 = Some(x); }
                   | Some(xx0) => { if x < xx0 { x0 = Some(x); }} };
        match x1 { None => { x1 = Some(x); }
                   | Some(xx1) => { if x > xx1 { x1 = Some(x); }} };
        match y0 { None => { y0 = Some(y); }
                   | Some(yy0) => { if y < yy0 { y0 = Some(y); }} };
        match y1 { None => { y1 = Some(y); }
                   | Some(yy1) => { if y > yy1 { y1 = Some(y); }} };
    }

    (*x0.expect("x0"), *x1.expect("x1"), *y0.expect("y0"), *y1.expect("y1"))
}

fn closest(coords: &Vec<Coord>, c0: (i32, i32)) -> Option<&Coord> {
    let mut closest_d = None;
    let mut closest_coords = Vec::new();

    for c1 in coords {
        let new_d = distance(*c1, c0);
        match closest_d {
            | None => {
                closest_d = Some(new_d);
                closest_coords.push(c1);
            }
            | Some(old_d) =>  {
                if new_d < old_d {
                    closest_d = Some(new_d);
                    closest_coords.clear();
                    closest_coords.push(c1);
                } else if new_d == old_d {
                    closest_coords.push(c1);
                }
            }
        }
    }

    if closest_coords.len() == 1 {
        Some(closest_coords[0])
    } else {
        None
    }
}

fn distance((x1, y1) : Coord, (x2, y2): Coord) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}
