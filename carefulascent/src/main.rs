use std::io::{self, BufRead};

// Struct to hold shield information
#[derive(Debug)]
struct Shield {
    lower_bound: i32,
    upper_bound: i32,
    factor: f64,
}

// Struct to hold all input data
#[derive(Debug)]
struct Problem {
    firefly_x: i32,
    firefly_y: i32,
    shields: Vec<Shield>,
}

fn read_input() -> Problem {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read Firefly coordinates
    let coords: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read number of shields
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    // Read shield information
    let mut shields = Vec::new();
    for _ in 0..n {
        let shield_info: Vec<f64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        shields.push(Shield {
            lower_bound: shield_info[0] as i32,
            upper_bound: shield_info[1] as i32,
            factor: shield_info[2],
        });
    }

    Problem {
        firefly_x: coords[0],
        firefly_y: coords[1],
        shields,
    }
}

fn solve(problem: &Problem) -> f64 {
    // TODO: Remember to remove this print
    println!("{:?}", problem);

    0.0
}

fn main() {
    let problem = read_input();
    let solution = solve(&problem);
    println!("{:.7}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_1() {
        let problem = Problem {
            firefly_x: 100,
            firefly_y: 140,
            shields: vec![
                Shield {
                    lower_bound: 40,
                    upper_bound: 90,
                    factor: 0.2,
                },
            ],
        };

        let expected = 1.0;
        assert!((solve(&problem) - expected).abs() < 1e-6);
    }

    #[test]
    fn test_sample_input_2() {
        let problem = Problem {
            firefly_x: 100,
            firefly_y: 100,
            shields: vec![
                Shield {
                    lower_bound: 0,
                    upper_bound: 20,
                    factor: 2.0,
                },
                Shield {
                    lower_bound: 50,
                    upper_bound: 100,
                    factor: 0.1,
                },
                Shield {
                    lower_bound: 20,
                    upper_bound: 50,
                    factor: 0.2,
                },
            ],
        };

        let expected = 1.9607843137;
        assert!((solve(&problem) - expected).abs() < 1e-6);
    }
}
