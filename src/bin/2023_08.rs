use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // for debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Read file the file given as first argument
    let mut contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    contents = contents.trim().to_string();

    // parse instructions list and network
    let split: Vec<&str> = contents.split("\n\n").collect();
    let instructions: Vec<usize> = split[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();
    let mut network: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in split[1].trim().split("\n") {
        let key = l.split(" = (").nth(0).unwrap().trim();
        let mut dest: Vec<&str> = l.split(" = (").nth(1).unwrap().split(",").collect();
        dest[1] = dest[1].trim().strip_suffix(")").unwrap();
        network.insert(key, dest);
    }

    // part 1
    /*
     * The first line gives which direction at an intersection to go. The instructions are infinitely repeated
     *  until the target is found. How many steps are needed to go from 'AAA' to 'ZZZ'?
     */
    let mut current_node = "AAA";
    let mut steps = 0;
    loop {
        let mut found_end = false;
        for i in instructions.iter() {
            steps += 1;
            current_node = network.get(current_node).unwrap()[*i];
            if current_node == "ZZZ" {
                found_end = true;
                break;
            }
        }
        if found_end {
            break;
        }
    }
    println!("It took {} steps", steps);

    // part 2
    /*
     * Instead of starting at 'AAA' this time we start at any node which ends with 'A'. We follow the
     * instructions in parallel for each starting point. If in all cases we are at a node which ends with
     * 'Z' we found the end. How many steps are needed to achieve this?
     */
    let mut current_nodes = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect::<Vec<&str>>();
    // Only do the first node for now
    let mut steps: Vec<Vec<(&str, i32)>> = Vec::new();
    for current_node in current_nodes.iter_mut() {
        let mut step = 0;
        let mut tmp: Vec<(&str, i32)> = Vec::new();
        loop {
            let mut found_end = false;
            for i in instructions.iter() {
                step += 1;
                *current_node = network.get(current_node).unwrap()[*i];
                if current_node.ends_with('Z') {
                    // check if we already found this node
                    found_end = tmp.iter().any(|(n, _)| *n == *current_node);
                    tmp.push((current_node, step));
                    break;
                }
            }
            if found_end {
                break;
            }
        }
        steps.push(tmp);
    }
    // investigating the output of the steps we see the following:
    // 1. Loops start at the first end node
    // 2. The loops have the same number of steps as to reach the first end node
    // --> just find the smallest common multiple of the loop sizes
    let loop_size: Vec<u64> = steps.iter().map(|s| (s[1].1 - s[0].1) as u64).collect();
    let res = loop_size.iter().fold(loop_size[0], |acc, f| {
        // lcm(a,b) = a*b/gcd(a,b)
        // euclid's algorithm
        let mut a = acc;
        let mut b = *f;
        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        return (acc * *f) / a;
    });
    println!("It took {} steps", res);
}
