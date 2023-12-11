use std::env;
use std::fs;

fn solve_task(contents: &str, expansion: usize) -> usize {
    let mut hx: Vec<usize> = vec![expansion; contents.lines().next().unwrap().len()];
    let mut hy: Vec<usize> = vec![1; contents.lines().count()];
    let mut pos: Vec<(usize, usize)> = Vec::new();
    for (j, line) in contents.lines().enumerate() {
        // find all '#' and
        let mut is_empty = true;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                pos.push((i, j));
                is_empty = false;
                if hx[i] == expansion {
                    hx[i] = 1;
                }
            }
        }
        if is_empty {
            hy[j] = expansion;
        }
    }
    for i in 1..hx.len() {
        hx[i] += hx[i - 1];
    }
    for j in 1..hy.len() {
        hy[j] += hy[j - 1];
    }

    let mut sum = 0;
    for (i, p) in pos.iter().enumerate() {
        for o in pos[i + 1..].iter() {
            let dx = usize::abs_diff(hx[p.0], hx[o.0]);
            let dy = usize::abs_diff(hy[p.1], hy[o.1]);
            sum += dx + dy;
        }
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

    // part 1
    /*
     * There are galaxies '#'. Find the sum of all pairwise distances. The distance is the manhattan metric where
     * rows and columns which contain no galaxies are twice as long as the other rows and columns.
     */
    println!("Sum of all distances: {}", solve_task(contents.as_str(), 2));

    // part 2
    /*
     * Same as part 1, but now the rows and columns which contain no galaxies are 1000000 times as long as
     * the other rows and columns.
     */
    println!(
        "Sum of all distances: {}",
        solve_task(contents.as_str(), 1000000)
    );
}
