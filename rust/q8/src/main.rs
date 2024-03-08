use regex::Regex;
use std::collections::HashMap;

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

    println!(
        "Solution 1: {}",
        steps_from(&nodes, &dirs, "AAA", Box::new(|node| node == "ZZZ"))
    );

    let start_nodes: Vec<&String> = nodes.keys().filter(|k| k.ends_with("A")).collect();

    let step_counts = start_nodes.iter().map(|cursor| {
        steps_from(&nodes, &dirs, cursor, Box::new(|node| node.ends_with("Z"))) as i128
    });

    let step_lcm = lcm(&step_counts.collect::<Vec<i128>>());

    println!("Solution 2: {}", step_lcm);
}

fn steps_from(
    nodes: &HashMap<String, (String, String)>,
    dirs: &Vec<Dir>,
    from: &str,
    to: Box<dyn Fn(&str) -> bool>,
) -> i32 {
    let mut steps = 0;
    let mut node = from;
    'outer: loop {
        for dir in dirs {
            let (left, right) = nodes.get(node).unwrap();
            // println!("Node {} {:?}, Dir: {:?}", node, nodes.get(node), dir);
            match dir {
                Dir::L => node = left,
                Dir::R => node = right,
            }
            steps += 1;

            if to(node) {
                break 'outer;
            }
        }
    }

    steps
}

fn lcm(nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
