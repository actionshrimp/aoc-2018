fn parse_lines(lines: &Vec<&str>) {
}

fn main() {
    let fname = "data/04.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let mut lines : Vec<_> = fdata.lines().collect();
    lines.sort();

    let parsed = parse_lines(&lines);

    // println!("result p1: {}", part1(&fabric));
    // println!("result p2: {}", part2(&fabric, &claims));
}
