mod utils;
use std::collections::HashMap;
use num::integer::lcm;

#[derive(Debug)]
struct Node {
    id: u64,
    left: u64,
    right: u64,
    line: String,
}

impl Node {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(['=', ',']).collect();

        let id = node_id_from_( &parts[0].trim() );
        let left = node_id_from_( &parts[1].trim()[1..] );
        let right = node_id_from_( &parts[2].trim()[..3] );

        Node {
            id,
            left,
            right,
            line: String::from(line),
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<u64, Node>,
    current_node: u64,
}

impl Graph {
    fn new(nodes: HashMap<u64, Node>, current_node: u64) -> Self {
        Graph {
            nodes,
            current_node,
        }
    }

    fn step(&mut self, direction: &char) {
        match direction {
            'L' => self.current_node = self.nodes[&self.current_node].left,
            _ => self.current_node = self.nodes[&self.current_node].right,
        }
    }

    fn step_from(&mut self, node: &u64, direction: &char) -> u64 {
        match direction {
            'L' => self.nodes[node].left,
            _ => self.nodes[node].right,
        }
    }

    fn is_at(&self, node: &u64) -> bool {
        self.current_node == *node
    }
}

fn part1() {
    const FILE: &str = "./inputs/day8.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut steps = 0;
    let directions: Vec<char> = lines[0].chars().collect();
    let nodes: HashMap<u64, Node> = lines[2..].iter()
                                    .map(|line| {
                                        let node = Node::from_line(line);
                                        ( node.id, node )
                                    })
                                    .collect();
    let start = node_id_from_("AAA");
    let target = node_id_from_("ZZZ");
    let mut graph = Graph::new(nodes, start);

    'search: loop {
        for direction in &directions {
            graph.step(direction);
            steps += 1;

            if graph.is_at(&target) {
                break 'search;
            }
        }
    }

    println!("Part 1: It takes {steps} steps to reach ZZZ");
}

fn part2() {
    const FILE: &str = "./inputs/day8.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut steps = 0;
    let directions: Vec<char> = lines[0].chars().collect();
    let nodes: HashMap<u64, Node> = lines[2..].iter()
                                    .map(|line| {
                                        let node = Node::from_line(line);
                                        ( node.id, node )
                                    })
                                    .collect();

    let mut start_nodes: Vec<_> = nodes.iter()
                    .filter_map(|(id, node)| if id % 26 == 0 { Some(id) } else { None })
                    .copied()
                    .collect();
    let mut steps_per_node: HashMap<u64, u64> = HashMap::new();
    let mut graph = Graph::new(nodes, 0);

    for node in &start_nodes {
        steps = 0;
        let mut current_node: u64 = *node;

        'search: loop {
            for direction in &directions {
                steps += 1;
                current_node = graph.step_from(&current_node, direction);
                if current_node % 26 == 25 {
                    steps_per_node.insert(*node, steps);
                    break 'search;
                }
            }
        }
    }

    steps = steps_per_node.iter()
                        .fold(1, |a, (id, steps)| lcm(a, *steps));

    println!("Part 2: It takes {steps} steps for all paths to reach **Z");
}

fn node_id_from_(label: &str) -> u64 {
    label.chars().rev().enumerate().map(|(i, c)| 26u64.pow(i as u32) * (((c as u8) - 65) as u64)).sum::<u64>()
}

fn main() {
    part1();
    part2();
}
