use std::collections::HashMap;
use std::env;
use std::fs;

fn extract_first_digit(line: impl Iterator<Item = char>) -> Option<u32> {
    for c in line {
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap());
        }
    }
    return None;
}

fn main() {
    // for debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Read file the file given as first argument
    let mut contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    contents = contents.trim().to_string();

    // part 1
    /*
     * Extract the first and last digit per line. The calibration value is the two digit number formed by the first and last digit (that order)
     */
    let mut sum: u32 = 0;
    for line in contents.split('\n') {
        // first digit
        match extract_first_digit(line.chars()) {
            None => println!("The given line has no digits!"),
            Some(digit) => sum += 10 * digit,
        }

        // last digit
        match extract_first_digit(line.chars().rev()) {
            None => println!("The given line has no digits!"),
            Some(digit) => sum += digit,
        }
    }
    println!("The sum of the calibration values is {}", sum);

    // part 2
    /*
     * Extract the first and last number per line. A number is either a digit or spelled out. The calibration value is the two digit number formed by the first and last number (that order)
     */
    let forwards: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    sum = 0;
    for line in contents.split('\n') {
        // first number
        sum += 10
            * forwards
                .keys()
                .map(|k| match line.find(k) {
                    None => (usize::MAX, 0),
                    Some(p) => (p, forwards[k]),
                })
                .min()
                .unwrap()
                .1;
        // second number
        sum += forwards
            .keys()
            .map(|k| match line.rfind(k) {
                None => (usize::MIN, 0),
                Some(p) => (p, forwards[k]),
            })
            .max()
            .unwrap()
            .1;
    }
    println!("The sum of the calibration values is {}", sum);
}
