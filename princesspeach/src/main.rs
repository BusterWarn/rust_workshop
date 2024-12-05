use std::io::{self, BufRead};

// Structure to hold the problem input
#[derive(Debug)]
struct Problem {
    n: i32,           // total number of obstacles
    y: i32,           // number of obstacles Mario found
    obstacles: Vec<i32>, // obstacles Mario reported
}

// Structure to hold the solution
#[derive(Debug, PartialEq)]
struct Solution {
    missed_obstacles: Vec<i32>,
    found_count: i32,
}

fn read_input() -> Problem {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line containing N and Y
    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = nums[0];
    let y = nums[1];

    // Read Y lines containing obstacles
    let mut obstacles = Vec::new();
    for _ in 0..y {
        let line = lines.next().unwrap().unwrap();
        obstacles.push(line.parse::<i32>().unwrap());
    }

    Problem {
        n,
        y,
        obstacles,
    }
}

fn solve(input: &Problem) -> Solution {
    // TODO: Remember to remove this print
    println!("{:?}", input);

    for i in 0..=input.n {
        println!("{}", i);
    }

    Solution {
        missed_obstacles: vec![],
        found_count: 0,
    }
}

fn print_solution(solution: &Solution) {
    for obstacle in &solution.missed_obstacles {
        println!("{}", obstacle);
    }

    // Print the summary line
    println!("Mario got {} of the dangerous obstacles.", solution.found_count);
}

fn main() {
    let problem = read_input();
    let solution = solve(&problem);
    print_solution(&solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let input = Problem {
            n: 20,
            y: 4,
            obstacles: vec![5, 10, 12, 16],
        };

        let expected = Solution {
            missed_obstacles: vec![0, 1, 2, 3, 4, 6, 7, 8, 9, 11, 13, 14, 15, 17, 18, 19],
            found_count: 4,
        };

        let result = solve(&input);
        assert_eq!(result, expected);
    }
}
