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

    /*
     * Per line there are two sets of numbers, the first set are the winning numbers, the second are the candiate numbers.
     * For part 1 the number of matching numbers to the power of 2  is the score of this scratch card. Return the 
     * sum of all the scores. For part 2 the number of matching numbers is how many extra cards you get. If one gets 
     * n extra cards, this means that you get one additional card for each of the next n cards.
     */
    let mut cards: Vec<u32> = vec![1; contents.lines().count()];
    let mut sum = 0;
    for (id, line) in contents.lines().enumerate() {
        let all_numbers = line.split_once(":").unwrap().1;
        let winning_numbers: Vec<u32> = all_numbers
            .split_once("|")
            .unwrap()
            .0
            .split_whitespace()
            .map(|c: &str| c.parse::<u32>().unwrap())
            .collect();
        let candidate_numbers: Vec<u32> = all_numbers
            .split_once("|")
            .unwrap()
            .1
            .split_whitespace()
            .map(|c: &str| c.parse::<u32>().unwrap())
            .collect();

        let mut tmp = 0;
        for c in &candidate_numbers {
            for w in &winning_numbers {
                if c == w {
                    tmp += 1;
                }
            }
        }

        // for part 1
        if tmp > 0 {
            sum += u32::pow(2, tmp - 1);
        }

        // for part 2
        for i in 1..(tmp + 1) {
            cards[id + i as usize] += cards[id];
        }
    }
    println!("Sum of all points: {}", sum);
    println!("Total number of cards: {}", cards.iter().sum::<u32>());
}
