use std::env;
use std::fs;

fn get_number(i: usize, j: usize, grid: &Vec<char>, width: usize) -> usize {
    // find the first char
    let mut k_min = j;
    while grid[i * (width + 2) + k_min].is_digit(10) {
        k_min -= 1;
    }
    k_min += 1;
    let mut k_max = j;
    while grid[i * (width + 2) + k_max].is_digit(10) {
        k_max += 1;
    }
    return grid[(i * (width + 2) + k_min)..(i * (width + 2) + k_max)]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
}
fn main() {
    // for debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Read file the file given as first argument
    let mut contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    contents = contents.trim().to_string();

    // load the data into a padded grid
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap().len();
    let mut grid: Vec<char> = vec!['.'; (height + 2) * (width + 2)];
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[(i + 1) * (width + 2) + (j + 1)] = c;
        }
    }

    /*
     * Numbers which are adjecent to a symbol (horizontal, vertical or diagonal) are part numbers, return the sum.
     * For part 2, we need to find all gears '*' with exactly two numbers adjacent to it and return the sum of
     * their gear ratios (product of the two adjacent numbers)
     */
    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;
    for i in 1..height + 1 {
        for j in 1..width + 1 {
            let c = grid[i * (width + 2) + j];
            if !c.is_digit(10) && c != '.' {
                let mut part_numbers: Vec<u32> = Vec::new();
                for k in 0..3 {
                    let triplet = grid
                        [((i + k - 1) * (width + 2) + j - 1)..((i + k - 1) * (width + 2) + j + 2)]
                        .iter()
                        .collect::<String>();
                    if triplet.chars().filter(|c| c.is_digit(10)).count() == 1
                        || triplet.chars().filter(|c| c.is_digit(10)).count() == 3
                    {
                        let jj = j - 1 + triplet.find(|c: char| c.is_digit(10)).unwrap();
                        part_numbers.push(get_number(i + k - 1, jj, &grid, width) as u32);
                    } else if triplet.chars().filter(|c| c.is_digit(10)).count() == 2
                        && grid[(i + k - 1) * (width + 2) + j].is_digit(10)
                    {
                        let jj = j - 1 + triplet.find(|c: char| c.is_digit(10)).unwrap();
                        part_numbers.push(get_number(i + k - 1, jj, &grid, width) as u32);
                    } else if triplet.chars().filter(|c| c.is_digit(10)).count() == 2 {
                        part_numbers.push(get_number(i + k - 1, j - 1, &grid, width) as u32);
                        part_numbers.push(get_number(i + k - 1, j + 1, &grid, width) as u32);
                    }
                }
                sum += part_numbers.iter().sum::<u32>();
                if c == '*' && part_numbers.len() == 2 {
                    sum2 += part_numbers.iter().product::<u32>();
                }

            }
        }
    }
    println!("Sum of all part numbers: {}", sum);
    println!("Sum of all gear ratios: {}", sum2);
    // part 2
    /*
     * Description
     */
}
