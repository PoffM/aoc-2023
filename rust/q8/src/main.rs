use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use regex::Regex;

#[derive(Debug)]
enum Dir {
    L,
    R,
}

fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let header = lines[0];
    let dirs: Vec<Dir> = header
        .chars()
        .map(|c| match c {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("Invalid direction: {}", c),
        })
        .collect();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    let node_lines = &lines[2..];
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    for line in node_lines {
        // let mut results = vec![];
        for (_, [id, left, right]) in re.captures_iter(line).map(|c| c.extract()) {
            // insert first 3 chars
            nodes.insert(id.to_string(), (left.to_string(), right.to_string()));
        }
    }

    // get first 3 chars
    let mut steps = 0;
    let mut node = "AAA";
    'outer: loop {
        for dir in &dirs {
            let (left, right) = nodes.get(node).unwrap();
            // println!("Node {} {:?}, Dir: {:?}", node, nodes.get(node), dir);
            match dir {
                Dir::L => node = left,
                Dir::R => node = right,
            }
            steps += 1;

            if node == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("Steps: {}", steps);
}
