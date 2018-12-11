fn is_inverse (c1 : char, c2 : char) -> bool {
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2
}

fn run_chain(chain : &str) -> Vec<char> {
    let mut stack = Vec::new();

    for c in chain.chars() {
        match stack.pop() {
            | None =>
                stack.push(c),
            | Some(c_last) =>
                if !is_inverse(c, c_last) {
                    stack.push(c_last);
                    stack.push(c);
                }
        }
    }

    stack
}

#[test]
fn test_chain() {
    let res = run_chain("dabAcCaCBAcCcaDA");
    assert_eq!(res.into_iter().collect::<String>(), "dabCBAcaDA");
}

fn part1(chain: &str) -> usize {
    let result = run_chain(chain);
    result.len()
}

fn part2(chain: &str) -> usize {
    let mut shortest = 50000;
    for bad in "abcdefghijklmnopqrstuvwxyz".chars() {
        let without = chain.chars().filter(|c| {
            c.to_ascii_lowercase() != bad
        });

        let chain_result = run_chain(&without.collect::<String>());
        let len = chain_result.len();

        if len < shortest {
            shortest = len
        }
    }

    shortest
}

fn main() {
    let fname = "data/05.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let first_line = fdata.lines().collect::<Vec<_>>().into_iter().nth(0).expect("first line");

    let chain = first_line;
    println!("result p1: {}", part1(&chain));
    println!("result p2: {}", part2(&chain));
}
