use std::io::{self, Read};

fn main() {
    let input = read_input();
    let solution = solve(&input);
    println!("{}", solution);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn solve(input: &str) -> char {
    // TODO: Remember to remove this print
    println!("{}", input);

    'A'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let input = "A2B1A2B2A1A2A2A2";
        let expected = 'A';
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn test_sample_2() {
        let input = "A2B2A1B2A2B1A2B2A1B2A1A1B1A1A2";
        let expected = 'A';
        assert_eq!(solve(input), expected);
    }
}
