use std::collections::HashMap;
use std::env;
use std::fs;

fn count_symbols(code: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in code.chars() {
        if map.contains_key(&c) {
            *map.get_mut(&c).unwrap() += 1;
        } else {
            map.insert(c, 1);
        }
    }
    return map;
}

fn get_hand(min: u32, max: u32, code: u64, bid: u32) -> (u32, u64, u32) {
    match (max, min) {
        (5, _) => return (6, code, bid),
        (4, _) => return (5, code, bid),
        (3, 2) => return (4, code, bid),
        (3, _) => return (3, code, bid),
        (2, 2) => return (2, code, bid),
        (2, _) => return (1, code, bid),
        _ => return (0, code, bid),
    }
}

fn encode_hand(hand: impl Iterator<Item = char>, j_value: u64) -> u64 {
    let mut code: u64 = 0;
    for (i, c) in hand.enumerate() {
        let exp = 2 * (4 - i) as u32;
        if c.is_digit(10) {
            code += c.to_digit(10).unwrap() as u64 * 10u64.pow(exp);
        } else {
            match c {
                'T' => code += 10 * 10u64.pow(exp),
                'J' => code += j_value * 10u64.pow(exp),
                'Q' => code += 12 * 10u64.pow(exp),
                'K' => code += 13 * 10u64.pow(exp),
                'A' => code += 14 * 10u64.pow(exp),
                _ => panic!("Unknown symbol!"),
            }
        }
    }
    return code;
}

fn process_hands(mut res: Vec<(u32, u64, u32)>) -> u32 {
    res.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let mut sum = 0;
    for (i, a) in res.iter().enumerate() {
        sum += (i + 1) as u32 * a.2;
    }
    return sum;
}
fn main() {
    // for debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Read file the file given as first argument
    let mut contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    contents = contents.trim().to_string();

    // Part 1
    /*
     * Order the hands and multiply the bid with the position in the list
     */
    let mut res: Vec<(u32, u64, u32)> = Vec::new();
    for l in contents.lines() {
        let l_split: Vec<&str> = l.split_whitespace().collect();
        let bid = l_split[1].parse::<u32>().unwrap();
        // to find out what type of hand it is
        let mut max = 0;
        let mut min = 0;
        for (_, v) in count_symbols(&l_split[0]).iter() {
            if *v > max {
                min = max;
                max = *v;
            } else if *v > min {
                min = *v;
            }
        }

        res.push(get_hand(min, max, encode_hand(l_split[0].chars(), 11), bid));
    }

    println!("Sum: {}", process_hands(res));

    // Part 2
    /*
     * Now 'J' is a joker which can be used to make the strongest hand possible
     */
    let mut res: Vec<(u32, u64, u32)> = Vec::new();
    for l in contents.lines() {
        let l_split: Vec<&str> = l.split_whitespace().collect();
        let bid = l_split[1].parse::<u32>().unwrap();

        // to find out what type of hand it is
        let map = count_symbols(&l_split[0]);
        let mut max = 0;
        let mut min = 0;
        for (k, v) in map.iter() {
            if *k == 'J' {
                continue;
            }
            if *v > max {
                min = max;
                max = *v;
            } else if *v > min {
                min = *v;
            }
        }
        if let Some(j_value) = map.get(&'J') {
            if max + j_value > 5 {
                min += max + j_value - 5;
                max = 5;
            } else {
                max += j_value;
            }
        }

        res.push(get_hand(min, max, encode_hand(l_split[0].chars(), 1), bid));
    }
    println!("Sum: {}", process_hands(res));
}
