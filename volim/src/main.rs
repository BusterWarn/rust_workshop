use std::io::{self, BufRead};

#[derive(Debug)]
struct Problem {
    initial_player: i32,
    questions: Vec<(i32, char)>, // (time, answer_type)
}

fn read_input() -> Vec<Problem> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read initial player number
    let initial_player: i32 = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Read number of questions
    let num_questions: i32 = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Read question outcomes
    let mut questions = Vec::new();
    for _ in 0..num_questions {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let time = parts[0].parse().unwrap();
        let answer_type = parts[1].chars().next().unwrap();
        questions.push((time, answer_type));
    }

    vec![Problem {
        initial_player,
        questions,
    }]
}

fn solve(problem: &Problem) -> i32 {
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
        let problem = Problem {
            initial_player: 1,
            questions: vec![
                (20, 'T'),
                (50, 'T'),
                (80, 'T'),
                (50, 'T'),
                (30, 'T'),
            ],
        };
        assert_eq!(solve(&problem), 5);
    }

    #[test]
    fn test_sample_2() {
        let problem = Problem {
            initial_player: 3,
            questions: vec![
                (100, 'T'),
                (100, 'N'),
                (100, 'T'),
                (100, 'T'),
                (100, 'N'),
            ],
        };
        assert_eq!(solve(&problem), 4);
    }

    #[test]
    fn test_sample_3() {
        let problem = Problem {
            initial_player: 5,
            questions: vec![
                (70, 'T'),
                (50, 'P'),
                (30, 'N'),
                (50, 'T'),
                (30, 'P'),
                (80, 'T'),
            ],
        };
        assert_eq!(solve(&problem), 7);
    }
}
