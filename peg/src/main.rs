use std::io::{self, BufRead};

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: vec![vec![' '; 7]; 7],
        }
    }

    fn get_above(&self, x: usize, y: usize, offset: usize) -> char {
        if y >= offset {
            self.grid[y - offset][x]
        } else {
            ' '
        }
    }

    fn get_below(&self, x: usize, y: usize, offset: usize) -> char {
        if y + offset < self.grid.len() {
            self.grid[y + offset][x]
        } else {
            ' '
        }
    }

    fn get_left(&self, x: usize, y: usize, offset: usize) -> char {
        if x >= offset {
            self.grid[y][x - offset]
        } else {
            ' '
        }
    }

    fn get_right(&self, x: usize, y: usize, offset: usize) -> char {
        if x + offset < self.grid[0].len() {
            self.grid[y][x + offset]
        } else {
            ' '
        }
    }
}

fn main() {
    let board = read_input();
    let solution = solve(&board);
    println!("{}", solution);
}

fn read_input() -> Board {
    let stdin = io::stdin();
    let mut board = Board::new();

    for (y, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        for (x, ch) in line.chars().enumerate() {
            board.grid[y][x] = ch;
        }
    }

    board
}

fn solve(board: &Board) -> i32 {
    // TODO: Remember to remove this debug print before submission
    println!("Debug - Input received:");
    for row in &board.grid {
        println!("{}", row.iter().collect::<String>());
    }

    // TODO: Implement actual solution here

    0  // Dummy return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let mut board = Board::new();
        let sample = vec![
            "   ooo   ",
            "   ooo   ",
            " ooooooo ",
            " ooo.ooo ",
            " ooooooo ",
            "   ooo   ",
            "   ooo   ",
        ];

        for (y, line) in sample.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                board.grid[y][x] = ch;
            }
        }

        assert_eq!(solve(&board), 4);
    }

    #[test]
    fn test_sample_2() {
        let mut board = Board::new();
        let sample = vec![
            "   ooo   ",
            "   ooo   ",
            "...ooo...",
            "oo...oo  ",
            "...ooo...",
            "   ooo   ",
            "   ooo   ",
        ];

        for (y, line) in sample.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                board.grid[y][x] = ch;
            }
        }

        assert_eq!(solve(&board), 12);
    }
}
