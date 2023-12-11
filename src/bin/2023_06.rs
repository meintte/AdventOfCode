use std::env;
use std::fs;
use std::iter::zip;

fn num_ints_between(t: f64, d: f64) -> usize {
    // use quadratic formula to find how many integers j fulfill: (t-j)*j > d
    let disc: f64 = ((t * t) - 4.0 * d).sqrt();
    let mut tmin: f64 = 0.5 * (t - disc);
    let mut tmax: f64 = 0.5 * (t + disc);
    // to actually beat the distance
    if tmin.ceil() - tmin < 0.1 {
        tmin += 0.000001;
    }
    if tmax - tmax.floor() < 0.1 {
        tmax -= 0.000001;
    }
    return (tmax.floor() - tmin.ceil() + 1.0) as usize;
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
     * Description
     */
    // fill vectors with the times and distances
    let time: Vec<usize> = contents
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let distance: Vec<usize> = contents
        .lines()
        .nth(1)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    /*
     * Given a time t, how long do I have to press the button such that the boat beats the given distance.
     * The distance the boat achieves is given by s = (t - j) * j where j is how long I press the button.
     */
    // Time is the amount I have
    // Distance is what I have to beat
    let mut prod = 1;
    for (t, d) in zip(&time, &distance) {
        let t: f64 = *t as f64;
        let d: f64 = *d as f64;
        let wins = num_ints_between(t, d);
        prod *= wins;
    }
    println!("Product of all win opportunities: {}", prod);

    // part 2
    /*
     * There is no list of times and distances, just combine append the integers together to get one set
     */
    let t = time
        .iter()
        .fold(String::from(""), |mut acc: String, digit: &usize| {
            acc.push_str(&digit.to_string());
            return acc;
        })
        .parse::<f64>()
        .unwrap();
    let d = distance
        .iter()
        .fold(String::from(""), |mut acc: String, digit: &usize| {
            acc.push_str(&digit.to_string());
            return acc;
        })
        .parse::<f64>()
        .unwrap();
    println!("There are {} win opportunities", num_ints_between(t, d));
}
