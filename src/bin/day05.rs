use std::str::Chars;

fn is_inverse (c1 : char, c2 : char) -> bool {
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2
}

fn run_chain(chars : Chars) -> Vec<char> {
    let mut stack = Vec::new();

    for c in chars {
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
    let res = run_chain("dabAcCaCBAcCcaDA".chars());
    assert_eq!(res.into_iter().collect::<String>(), "dabCBAcaDA");
}

fn main() {
    let fname = "data/05.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let first_line = fdata.lines().collect::<Vec<_>>().into_iter().nth(0).expect("first line");

    let chars = first_line.chars();

    let p1_chain = run_chain(chars);
    println!("{}", p1_chain.iter().collect::<String>());

    println!("result p1: {}", p1_chain.len());
}
