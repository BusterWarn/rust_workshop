use std::io::{self, BufRead};

fn main() {
    let (n, a, enemy_ships) = read_input();
    let solution = solve(n, a, &enemy_ships);
    println!("{}", solution);
}

fn read_input() -> (u32, u32, Vec<u32>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line with n and a
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let a: u32 = iter.next().unwrap().parse().unwrap();

    // Read second line with enemy ships
    let second_line = lines.next().unwrap().unwrap();
    let enemy_ships: Vec<u32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (n, a, enemy_ships)
}

fn solve(n: u32, a: u32, enemy_ships: &Vec<u32>) -> i32 {
    // TODO: Remember to remove this print before submission
    println!("Input received: {} {} {:?}", n, a, enemy_ships);

    // TODO: Implement actual solution here

    // TODO: This is a dummy return value - replace with actual solution
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let n: u32 = 3;
        let a: u32 = 6;
        let enemy_ships: Vec<u32> = vec![1, 2, 3];
        assert_eq!(solve(n, a, &enemy_ships), 2);
    }

    #[test]
    fn test_sample_2() {
        let n: u32 = 5;
        let a: u32 = 8;
        let enemy_ships: Vec<u32> = vec![7, 0, 3, 5, 2];
        assert_eq!(solve(n, a, &enemy_ships), 3);
    }
}
