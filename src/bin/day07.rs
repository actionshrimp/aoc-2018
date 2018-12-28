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

    fn remove (self : &mut Graph, n: Node) -> () {
        self.available.remove(&n);
        match self.node_dependents.remove_entry(&n) {
            | None => {}
            | Some((_, deps)) => {
                deps.iter().for_each(|d| {
                    self.node_requirements.entry(*d).and_modify(|v| { v.remove(&n); });

                    if self.node_requirements.get(d).expect("reqs for dep").len() == 0 {
                        self.node_requirements.remove(d);
                        self.available.insert(*d);
                    }
                })
            }
        }

        ()
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

fn remove_first_available(g: &mut Graph) -> Option<Node> {
    let e = g.available.iter().min().map(|v| v.clone());

    match e {
        None => {}
        Some(earliest) => {
            g.remove(earliest);
        }
    }

    e
}

fn calc_order(mut g : Graph) -> String {
    let mut s = String::new();

    while let Some(c) = remove_first_available(&mut g) {
        s.push(c);
    }

    s
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_LINES : [&str; 7] =
        ["Step C must be finished before step A can begin.",
         "Step C must be finished before step F can begin.",
         "Step A must be finished before step B can begin.",
         "Step A must be finished before step D can begin.",
         "Step B must be finished before step E can begin.",
         "Step D must be finished before step E can begin.",
         "Step F must be finished before step E can begin."];

    fn example_pairs() -> Vec<Pair> {
        EXAMPLE_LINES.iter().map(|p| {parse_line(p)}).collect::<Vec<_>>()
    }

    #[test]
    fn test_example() {
        let g = Graph::from_pairs(&example_pairs());
        assert_eq!("CABDFE", calc_order(g));
    }

    #[test]
    fn test_example_part_2() {
        let g = Graph::from_pairs(&example_pairs());
        assert_eq!(15, calc_duration(g, 2, 0));
    }
}


fn part1(pairs: &Vec<Pair>) -> String {
    let g = Graph::from_pairs(pairs);
    calc_order(g)
}

fn node_duration(base_delay: usize, c: char) -> usize {
    base_delay + (c as usize - 64)
}

fn calc_duration(mut g: Graph, workers: usize, base_delay: usize) -> usize {
    let mut t : usize = 0;
    let mut work_items : Vec<(Node, usize)> = Vec::new();

    while g.available.len() > 0 {
        let mut available_not_assigned : Vec<_> =
            g.available.iter().filter(|a| { work_items.iter().find(|(w, _)| { &w == a }).is_none() }).collect();

        available_not_assigned.sort();
        for node in available_not_assigned.iter().take(workers - work_items.len()) {
            work_items.push((*node.clone(), t as usize));
        }

        work_items.sort_unstable_by(|(a, s_a), (b, s_b)| {
            let t_a = node_duration(base_delay, *a) - (t - s_a);
            let t_b = node_duration(base_delay, *b) - (t - s_b);
            t_b.cmp(&t_a)
        });

        let (completed_node, started_at) = work_items.pop().expect("empty queue");
        let d = node_duration(base_delay, completed_node);
        t += d - (t - started_at);
        g.remove(completed_node);
    }

    t as usize
}


fn part2(pairs: &Vec<Pair>) -> usize {
    let g = Graph::from_pairs(pairs);
    calc_duration(g, 5, 60)
}

fn main() {
    let fname = "data/07.txt";
    let fdata = std::fs::read_to_string(fname)
        .expect(&format!("couldn't read {}", fname));

    let lines : Vec<&str> = fdata.lines().collect();

    let parsed_lines : Vec<Pair> =
        lines.iter().map(|line| parse_line(&line)).collect();

    println!("result p1: {}", part1(&parsed_lines));
    println!("result p2: {}", part2(&parsed_lines));
}
