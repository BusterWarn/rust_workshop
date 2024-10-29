use std::io::{self, BufRead};

#[derive(Debug)]
struct Problem {
    year: i32,
    cycle1: i32,
    cycle2: i32,
}

#[derive(Debug)]
struct TestCase {
    problems: Vec<Problem>,
}

fn read_input() -> TestCase {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of pairs
    let k: i32 = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut problems = Vec::new();

    // Read k lines of problem data
    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        problems.push(Problem {
            year: nums[0],
            cycle1: nums[1],
            cycle2: nums[2],
        });
    }

    TestCase { problems }
}

fn solve(problem: &Problem) -> i32 {
    // TODO: Remember to remove this print
    println!("Solving for: {:?}", problem);
    0
}

fn main() {
    let test_case = read_input();

    // Find the minimum year among all problems
    let min_year = test_case.problems.iter()
        .map(|p| solve(p))
        .min()
        .unwrap();

    println!("{}", min_year);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_1() {
        let problems = vec![
            Problem { year: 1992, cycle1: 13, cycle2: 17 },
            Problem { year: 1992, cycle1: 14, cycle2: 18 },
            Problem { year: 2001, cycle1: 5, cycle2: 7 },
        ];

        let min_year = problems.iter()
            .map(|p| solve(p))
            .min()
            .unwrap();

        assert_eq!(min_year, 2036);
    }

    #[test]
    fn test_sample_input_2() {
        let problems = vec![
            Problem { year: 2020, cycle1: 2, cycle2: 3 },
            Problem { year: 2019, cycle1: 3, cycle2: 4 },
        ];

        let min_year = problems.iter()
            .map(|p| solve(p))
            .min()
            .unwrap();

        assert_eq!(min_year, 2026);
    }
}
