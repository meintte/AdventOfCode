// use std::cmp;
use std::env;
use std::fs;
use std::vec;

fn apply_mapping(val: u32, m: &str) -> u32 {
    let ranges: Vec<&str> = m.split('\n').skip(1).collect();
    for r in ranges {
        let cur: Vec<u32> = r
            .split_whitespace()
            .map(|c: &str| c.parse::<u32>().unwrap())
            .collect();
        if val >= cur[1] && val - cur[1] < cur[2] {
            return cur[0] + (val - cur[1]);
        }
    }
    return val;
}

fn apply_mapping_pack(val: (u64, u64), m: &str) -> Vec<(u64, u64)> {
    let ranges: Vec<&str> = m.split('\n').skip(1).collect();
    for r in ranges {
        let cur: Vec<u64> = r
            .split_whitespace()
            .map(|c: &str| c.parse::<u64>().unwrap())
            .collect();
        if val.0 >= cur[1] && val.1 < cur[1] + cur[2] {
            return vec![(val.0 + cur[0] - cur[1], val.1 + cur[0] - cur[1])];
        } else if val.0 >= cur[1] + cur[2] || val.1 < cur[1] {
            // completely outside
        } else if val.0 < cur[1] {
            // partially inside, left part is outside
            let mut res: Vec<(u64, u64)> = Vec::new();
            res.extend(apply_mapping_pack((val.0, cur[1] - 1), m));
            res.extend(apply_mapping_pack((cur[1], val.1), m));
            return res;
        } else {
            // partially inside, right part is outside
            let mut res: Vec<(u64, u64)> = Vec::new();
            res.extend(apply_mapping_pack((val.0, cur[1] + cur[2]-1), m));
            res.extend(apply_mapping_pack((cur[1] + cur[2], val.1), m));
            return res;
        }
    }

    return vec![val];
}

fn main() {
    // for debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Read file the file given as first argument
    let mut contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    contents = contents.trim().to_string();

    /*
     * Apply the mappings to the seeds and return the minimum location.
     */
    let mut lines = contents.split("\n\n");
    let mut seeds: Vec<u32> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|c: &str| c.parse::<u32>().unwrap())
        .collect();
    for m in lines {
        for s in &mut seeds {
            *s = apply_mapping(*s, m.trim());
        }
    }
    println!("Minimum location: {}", seeds.iter().min().unwrap());

    /*
     * The seeds are now ranges, still return the minimum location
     */
    let mut lines = contents.split("\n\n");
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let a: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect();
    for i in (0..a.len()).step_by(2) {
        seeds.push((a[i], a[i] + a[i + 1]-1));
    }

    for m in lines {
        let m = m.trim();
        let mut new_seeds: Vec<(u64, u64)> = Vec::new();
        for s in &seeds {
            new_seeds.extend(apply_mapping_pack(*s, m));
        }
        seeds = new_seeds;
    }
    println!("Minimum location: {}", seeds.iter().min_by_key(|s| s.0).unwrap().0);
}
