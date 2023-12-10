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

    // part 1 & 2
    /*
     * Each game consist of a series of turns where in each turn the color and amout of cubes pulled out is given.
     * For part one: Given 12 red, 13 green, and 14 blue. What games are valid and what is their sum (of the respective ids)?
     * For part two: In each game determine the minimum amount such that it is a valid game. The power is defined as
     * the product of the amounts (per color). What is the sum of all powers
     */
    let check = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum: u32 = 0;
    let mut pow: u32 = 0;
    for line in contents.lines() {
        let id: u32 = line
            .split(":")
            .next()
            .unwrap()
            .rsplit(" ")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut min_bag = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for turn in line.rsplit(":").next().unwrap().split(";") {
            for color in turn.split(",") {
                let c_count: u32 = color
                    .trim()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let c = color.rsplit(" ").next().unwrap();
                if !min_bag.contains_key(c) {
                    min_bag.insert(c, c_count);
                } else if min_bag[c] < c_count {
                    min_bag.insert(c, c_count);
                }
            }
        }
        // for part 1
        if min_bag["red"] > check["red"]
            || min_bag["green"] > check["green"]
            || min_bag["blue"] > check["blue"]
        {
            sum += id;
        }
        // for part 2
        pow += min_bag["red"] * min_bag["green"] * min_bag["blue"];
    }
    println!("The valid games ids sum to: {}", sum);
    println!("The min power sum is: {}", pow);
}
