use std::io::{self, BufRead};

// Struct to hold the rule information
#[derive(Debug)]
struct Rule {
    from: char,
    to: String,
}

// Struct to hold all the problem input
#[derive(Debug)]
struct Problem {
    n: usize,            // number of rules
    m: usize,            // number of iterations
    rules: Vec<Rule>,    // transformation rules
    start: String,       // starting sequence S0
}

fn read_input() -> Problem {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line containing n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace()
        .map(|num| num.parse().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    // Read n lines containing rules
    let mut rules = Vec::new();
    for _ in 0..n {
        let rule_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = rule_line.split(" -> ").collect();
        rules.push(Rule {
            from: parts[0].chars().next().unwrap(),
            to: parts[1].to_string(),
        });
    }

    // Read starting sequence
    let start = lines.next().unwrap().unwrap();

    Problem {
        n,
        m,
        rules,
        start,
    }
}

fn solve(problem: &Problem) -> String {
    // TODO: Remember to remove this print
    println!("{:?}", problem);

    problem.start.clone()
}

fn main() {
    let problem = read_input();
    let solution = solve(&problem);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let problem = Problem {
            n: 2,
            m: 4,
            rules: vec![
                Rule { from: 'A', to: String::from("AB") },
                Rule { from: 'B', to: String::from("A") },
            ],
            start: String::from("A"),
        };

        assert_eq!(solve(&problem), "ABAABABA");
    }
}
