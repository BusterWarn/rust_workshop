# Rust Workshop

This repository contains boilerplate code and solutions for the programming problems from our [Kattis contest](https://open.kattis.com/contests/k44vjq). The boilerplate code provides a starting point for solving each problem, handling input/output and testing setup so you can focus on implementing the core solution.

Here you can also find some simple installation instructions and cheat sheet for Rust.

## Repository Status

When to competition starts, more problems will be uploaded here.

# Rust Installation and Kattis Setup Guide

## Installation Steps

We recommend you install Rust before the workshop starts, but it's also okay to just an online compiler such as [godbolt](https://godbolt.org/).

Official installation documentation: https://www.rust-lang.org/tools/install.

# Installation Steps

<details>
<summary>Windows</summary>

1. Choose your installation method:
   - **Option A - Native Installation:**
     - Download and run [rustup-init.exe](https://www.rust-lang.org/tools/install)
     - Follow the installer prompts

   - **Option B - WSL (Recommended):**
     1. Install WSL following [Microsoft's guide](https://learn.microsoft.com/en-us/windows/wsl/install)
     2. Open WSL terminal
     3. Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
</details>

<details>
<summary>macOS</summary>

```bash
# Install Xcode Command Line Tools first
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details>
<summary>Linux</summary>

```bash
# Install build essentials (Ubuntu/Debian)
sudo apt update
sudo apt install build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

After installation, restart your terminal and verify installation:
```bash
rustc --version
cargo --version
```

## Testing Your Installation

1. Create a new Rust project:
```bash
cargo new tornbygge
cd tornbygge
```

2. Replace the contents of `src/main.rs` with the code from [this gist](https://gist.github.com/BusterWarn/faa15c4cbf8d0003d7bbab7283af90ae)

3. Run tests:
```bash
cargo test
```
You should see two failed tests initially.

## Solving Your First Kattis Problem

1. Visit the [Tornbygge problem](https://open.kattis.com/problems/tornbygge)
2. Find the TODO comment in the code and implement the solution
3. Run `cargo test` to verify your solution
4. Submit your solution to Kattis

## Join the Contest

1. Visit [Kattis contest page](https://open.kattis.com/contests/zs4uwq)
2. Create an account if needed
3. Create a team (recommended: use your name) or join an existing team

## IDE Setup

### VS Code
1. Install [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
2. Open your Rust project folder
3. The extension should automatically activate

### Neovim
1. Install rust-analyzer using Mason:
```lua
require("mason").setup()
require("mason-lspconfig").setup({
    ensure_installed = { "rust_analyzer" }
})
```

2. Add to your LSP configuration:
```lua
setup_lsp("rust_analyzer", {})
```

Reference: [Sample Configuration](https://github.com/BusterWarn/config_files/commit/9b165ffa9d4cecf31a1b7cc13265f3c403ecbee7)

## Troubleshooting

- If cargo commands aren't recognized, ensure your PATH includes `~/.cargo/bin`
- For VS Code, open the project from the root folder for proper LSP functionality
- On Windows, if using WSL, ensure you're working within the WSL filesystem

## Structure

Each problem has its own directory containing:
- A Rust solution file with boilerplate code
- Test cases based on the sample input/output from Kattis

## Running Solutions

To run a specific solution:

1. Navigate to the problem directory:
   ```bash
   cd tornbygge  # or other problem directory
   ```

2. Run the solution:
   ```bash
   cargo run
   ```

3. Paste the test input and press Enter. If the program doesn't complete:
   - Press Ctrl+D (Unix/Mac)
   - Press Ctrl+Z then Enter (Windows)

## Testing

To run the tests for a solution:
```bash
cargo test
```

This will run all test cases including the sample inputs/outputs from Kattis.


## Cheatsheet

Two online cheatsheets:
https://cheats.rs/
https://dev.to/moekatib/the-completesh-rust-cheat-sheet-4fnn

## Input/Output and Debugging

```rust
println!("{:?}", variable);
dbg!(variable); // prints file:line = value

```

## Loops

```rust
// Basic for loop
for i in 0..10 { } // 0 to 9
for i in 0..=10 { } // 0 to 10

// With enumerate
for (index, value) in vec.iter().enumerate() { }

// While loop
while condition { }

// Loop with break
loop {
    if condition { break; }
}
```

## Vectors
```rust
// Creation
let mut vec = Vec::new();
let vec = vec![1, 2, 3];
let vec = (0..10).collect::<Vec<i32>>();

// Operations
vec.push(4);
vec.pop();
vec.len();
vec.is_empty();

// Sorting
vec.sort();
vec.sort_by(|a, b| b.cmp(a)); // descending
vec.sort_unstable(); // faster, doesn't preserve order of equal elements
```

## HashMap
```rust
use std::collections::HashMap;

// Creation
let mut map = HashMap::new();

// Insert and access
map.insert(key, value);
map.entry(key).or_insert(value);
map.get(&key);
map.contains_key(&key);

// Update value
*map.entry(key).or_insert(0) += 1; // counter pattern
```

## Strings
```rust
// Creation
let s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

// Operations
s.push_str(" world");
s.push('!');
s.contains("hello");
s.replace("old", "new");

// Split and join
let parts: Vec<&str> = s.split_whitespace().collect();
let joined = parts.join(" ");

// Convert to/from numbers
let num = "42".parse::<i32>().unwrap();
let str = num.to_string();
```

## Looping through strings

```rust
let text = String::from("Hello");

// Iterate over characters
for c in text.chars() {
    println!("{}", c);
}

// Iterate with index
for (i, c) in text.chars().enumerate() {
    println!("Position {}: {}", i, c);
}

// Iterate over bytes
for b in text.bytes() {
    println!("{}", b);
}

// Split into words
let sentence = String::from("Hello World");
for word in sentence.split_whitespace() {
    println!("{}", word);
}

// Get character at position (note: O(n) operation)
if let Some(c) = text.chars().nth(1) {
    println!("Second character: {}", c);
}
```

## Common Collections
```rust
use std::collections::{HashSet, VecDeque, BinaryHeap};

// HashSet
let mut set = HashSet::new();
set.insert(1);
set.contains(&1);

// VecDeque (double-ended queue)
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);

// BinaryHeap (max-heap)
let mut heap = BinaryHeap::new();
heap.push(1);
heap.pop();
```

## Error Handling Shortcuts

```rust
// Quick unwrap for competitive programming
result.unwrap();
option.unwrap_or(default);
option.unwrap_or_else(|| computation);
```
