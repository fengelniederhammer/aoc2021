use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::ops::Deref;

pub fn get_answer_1() -> usize {
    count_ways(read_file("src/day12/input.txt"))
}

pub fn get_answer_2() -> usize {
    count_ways_2(read_file("src/day12/input.txt"))
}

fn read_file(path: &str) -> CaveGraph {
    let file_content = read_to_string(path).expect("hoppla");

    let edges = file_content
        .lines()
        .map(String::from)
        .map(Edge::from)
        .collect();

    CaveGraph { edges }
}

#[derive(Debug)]
struct CaveGraph {
    edges: Vec<Edge>,
}

#[derive(Debug, Eq, PartialEq)]
struct Edge {
    node1: Box<Node>,
    node2: Box<Node>,
}

impl From<String> for Edge {
    fn from(str: String) -> Self {
        let parts: Vec<&str> = str.split("-").collect();
        Edge {
            node1: Box::from(Node::from(parts[0])),
            node2: Box::from(Node::from(parts[1])),
        }
    }
}

impl Edge {
    fn get_target_from(&self, node: &Node) -> Option<&Node> {
        match node {
            Node::End => None,
            node if node == self.node1.deref() => Some(&self.node2),
            node if node == self.node2.deref() => Some(&self.node1),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
enum Node {
    Start,
    End,
    Small(String),
    Large(String),
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Start => write!(f, "start"),
            Node::End => write!(f, "end"),
            Node::Small(str) | Node::Large(str) => write!(f, "{}", str.to_string()),
        }
    }
}

impl From<&str> for Node {
    fn from(str: &str) -> Node {
        match str {
            "start" => Node::Start,
            "end" => Node::End,
            str if str.to_uppercase() == str => Node::Large(str.to_string()),
            _ => Node::Small(str.to_string()),
        }
    }
}

fn count_ways(graph: CaveGraph) -> usize {
    count_ways_with_filter(&graph, Box::from(allow_multiple_large_caves))
}

fn count_ways_2(graph: CaveGraph) -> usize {
    count_ways_with_filter(&graph, Box::from(allow_one_small_cave_twice))
}

fn count_ways_with_filter<F>(graph: &CaveGraph, cave_filter: Box<F>) -> usize
where
    F: Fn(&Way, &Node) -> bool + Clone + 'static,
{
    let mut ways = vec![];
    let mut unfinished_ways = vec![Way {
        caves: vec![Node::Start],
    }];

    while !unfinished_ways.is_empty() {
        let next_ways = unfinished_ways
            .into_iter()
            .flat_map(|way| add_next_possible_caves(&graph, way, cave_filter.clone()))
            .collect::<Vec<Way>>();

        unfinished_ways = vec![];
        for way in next_ways {
            match way.caves.contains(&Node::End) {
                true => ways.push(way),
                false => unfinished_ways.push(way),
            }
        }
    }

    ways.len()
}

fn add_next_possible_caves(
    graph: &CaveGraph,
    way: Way,
    cave_filter: Box<dyn Fn(&Way, &Node) -> bool>,
) -> Vec<Way> {
    let last_cave = way.caves.last().unwrap().clone();
    let way2 = way.clone();

    graph
        .edges
        .iter()
        .filter_map(move |edge| edge.get_target_from(&last_cave))
        .filter(move |next_cave| cave_filter(&way2, *next_cave))
        .map(move |next_cave| way.clone().with(next_cave.clone()))
        .collect::<Vec<Way>>()
}

fn allow_multiple_large_caves(way: &Way, next_cave: &Node) -> bool {
    !way.caves.contains(next_cave) || matches!(next_cave, Node::Large(_))
}

fn allow_one_small_cave_twice(way: &Way, next_cave: &Node) -> bool {
    match next_cave {
        Node::Start => false,
        Node::End => !way.caves.contains(next_cave),
        Node::Large(_) => true,
        Node::Small(_) => {
            let small_caves_count = way
                .caves
                .iter()
                .filter(|cave| matches!(cave, Node::Small(_)))
                .fold(HashMap::new(), |mut map, cave| {
                    map.entry(cave).and_modify(|count| *count += 1).or_insert(1);
                    map
                });

            let small_caves_multiply_visited = small_caves_count
                .values()
                .fold(0, |twos, count| twos + count / 2);
            small_caves_multiply_visited < 1 || !way.caves.contains(next_cave)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Way {
    caves: Vec<Node>,
}

impl Way {
    fn with(mut self, next_cave: Node) -> Self {
        self.caves.push(next_cave);
        self
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::day12::*;

    #[test]
    fn test_read_file() {
        let graph = read_file("src/day12/test_input2.txt");

        let expected_first = Edge {
            node1: Box::new(Node::Small("fs".to_string())),
            node2: Box::new(Node::End),
        };
        let expected_last = Edge {
            node1: Box::new(Node::Start),
            node2: Box::new(Node::Large("RW".to_string())),
        };
        assert_eq!(Some(&expected_first), graph.edges.first());
        assert_eq!(Some(&expected_last), graph.edges.last());
    }

    #[rstest]
    #[case("src/day12/test_input.txt", 10)]
    #[case("src/day12/test_input1.txt", 19)]
    #[case("src/day12/test_input2.txt", 226)]
    fn test_count(#[case] file: &str, #[case] expected: usize) {
        let input = read_file(file);

        let result = count_ways(input);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("src/day12/test_input.txt", 36)]
    #[case("src/day12/test_input1.txt", 103)]
    #[case("src/day12/test_input2.txt", 3509)]
    fn test_count_2(#[case] file: &str, #[case] expected: usize) {
        let input = read_file(file);

        let result = count_ways_2(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn result() {
        println!("{}", get_answer_1());
        assert_eq!(3708, get_answer_1());
        println!("{}", get_answer_2());
        assert_eq!(93858, get_answer_2());
    }
}
