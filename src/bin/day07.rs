use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;

type Node = char;

struct Graph {
    available: HashSet<Node>,
    node_requirements: HashMap<Node, HashSet<char>>,
    node_dependents: HashMap<Node, HashSet<char>>
}

impl Graph {
    fn from_pairs (pairs: &Vec<Pair>) -> Graph {
        let mut g = Graph {
            available: HashSet::new(),
            node_requirements: HashMap::new(),
            node_dependents: HashMap::new()
        };

        for p in pairs {
            g.available.insert(p.requirement);
            g.node_requirements.entry(p.node).or_insert(HashSet::new()).insert(p.requirement);
            g.node_dependents.entry(p.requirement).or_insert(HashSet::new()).insert(p.node);
        }

        for (k, _) in &g.node_requirements {
            g.available.remove(&k);
        }

        g
    }

    fn consume_first_available(&mut self) -> Option<Node> {
        let e = self.available.iter().min().map(|v| v.clone());

        match e {
            None => {}
            Some(earliest) => {
                self.available.remove(&earliest);
                match self.node_dependents.remove_entry(&earliest) {
                    | None => {}
                    | Some((_, deps)) => {
                        deps.iter().for_each(|d| {
                            self.node_requirements.entry(*d).and_modify(|v| { v.remove(&earliest); });

                            if self.node_requirements.get(d).expect("reqs for dep").len() == 0 {
                                self.node_requirements.remove(d);
                                self.available.insert(*d);
                            }
                        })
                    }
                }
            }
        }

        e
    }
}

struct Pair { node: Node, requirement: Node }

fn parse_line(l: &str) -> Pair {
    let mut it = l.chars().skip(5);
    Pair {
        requirement: it.next().expect("first char"),
        node: it.skip(30).next().expect("second char")
    }
}

#[test]
fn test_parse() {
    let parsed = parse_line("Step L must be finished before step M can begin.");
    assert_eq!('L', parsed.requirement);
    assert_eq!('M', parsed.node);
}

fn calc_order(mut g : Graph) -> String {
    let mut s = String::new();

    while let Some(c) = g.consume_first_available() {
        s.push(c);
    }

    s
}

#[test]
fn test_example() {
    let example_input =
        vec!["Step C must be finished before step A can begin.",
             "Step C must be finished before step F can begin.",
             "Step A must be finished before step B can begin.",
             "Step A must be finished before step D can begin.",
             "Step B must be finished before step E can begin.",
             "Step D must be finished before step E can begin.",
             "Step F must be finished before step E can begin."];

    let parsed = example_input.iter().map(|p| {parse_line(p)}).collect::<Vec<_>>();
    let g = Graph::from_pairs(&parsed);
    assert_eq!("CABDFE", calc_order(g));
}

fn part1(pairs: &Vec<Pair>) -> String {

    let g = Graph::from_pairs(pairs);
    calc_order(g)

}

fn main() {
    let fname = "data/07.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let lines : Vec<&str> = fdata.lines().collect();

    let parsed_lines : Vec<Pair> =
        lines.iter().map(|line| parse_line(&line)).collect();

    println!("result p1: {}", part1(&parsed_lines));
    // println!("result p2: {}", part2(&parsed_lines));
}
