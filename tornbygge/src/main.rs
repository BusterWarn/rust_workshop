use std::io::{self, BufRead};

fn read_input() -> Vec<u32> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N (number of bricks)
    let _n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the brick widths
    lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve(bricks: Vec<u32>) -> u32 {
    let mut towers = 0;
    let mut current_top = 0;

    for &brick in &bricks {
        if brick > current_top {
            // TODO: Uncomment these lines
            // towers += 1;
            // current_top = 0;
        }
        current_top = brick;
    }

    towers
}

fn main() {
    let bricks = read_input();
    let solution = solve(bricks);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_1() {
        let input = vec![4, 3, 3, 2, 1, 2, 2, 1, 1, 3];
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_sample_input_2() {
        let input = vec![2, 2, 2, 2, 2];
        assert_eq!(solve(input), 1);
    }
}
