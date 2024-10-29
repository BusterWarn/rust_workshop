use std::io::{self, BufRead};

// Structure to hold the problem data
#[derive(Debug)]
struct Problem {
    length: i32,
}

fn read_input() -> Vec<Problem> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let num_cases = lines.next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut problems = Vec::new();

    // Read each test case
    for _ in 0..num_cases {
        let n = lines.next()
            .unwrap()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        problems.push(Problem { length: n });
    }

    problems
}

fn solve(problem: &Problem) -> i64 {
    // TODO: Remember to remove this print
    println!("{:?}", problem);
    0
}

fn main() {
    let problems = read_input();

    for problem in problems {
        let solution = solve(&problem);
        println!("{}", solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let problem = Problem { length: 2 };
        assert_eq!(solve(&problem), 3);
    }

    #[test]
    fn test_sample_2() {
        let problem = Problem { length: 10 };
        assert_eq!(solve(&problem), 144);
    }
}
