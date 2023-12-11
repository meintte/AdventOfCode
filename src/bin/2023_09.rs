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

    // part 1
    /*
     * From a list of numbers. Calculate a new list of numbers by subtracting the adjacent numbers. Repeat until
     * the list is full of zeros. Next append an additional zero and track back up to fund what value has to
     * be added to the original list.
     */
    let mut sum: i32 = 0;
    for hist in contents.split('\n') {
        let mut nums: Vec<i32> = hist
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for j in 1..nums.len() {
            for i in 0..nums.len() - j {
                nums[i] = nums[i + 1] - nums[i];
            }
        }
        let next_value = nums.iter().sum::<i32>();
        sum += next_value;
    }
    println!("sum: {}", sum);

    // part 2
    /*
     * Instead of adding a zero to the right, now add it to the left and find the value before the first value.
     */
    sum = 0;
    for hist in contents.split('\n') {
        let mut nums: Vec<i32> = hist
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for j in 1..nums.len() {
            for i in (j..nums.len()).rev() {
                nums[i] = nums[i] - nums[i - 1];
            }
        }
        let next_value = nums.iter().rfold(0, |acc, x| x - acc);
        sum += next_value;
    }
    println!("sum: {}", sum);
}
