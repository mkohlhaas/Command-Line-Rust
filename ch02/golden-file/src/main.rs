#![allow(unused)]

use goldenfile::Mint;
use std::io::Write;

// 1. Your code or function to test
fn generate_report(user: &str) -> String {
    format!(
        "--- REPORT ---\nUser: {}\nStatus: Active\nDate: 2026\n",
        user
    )
}

fn main() {
    println!("Nothing to see here!");
}

#[test]
fn test_report_generation() {
    // 2. Initialize the Mint manager pointing to your test data folder
    let mut mint = Mint::new("tests/testdata");

    // 3. Request a golden file from the mint
    let mut file = mint.new_goldenfile("user_report.txt").unwrap();

    // 4. Generate the test output
    let actual_output = generate_report("Alice");

    // 5. Write the output to the golden file wrapper
    write!(file, "{}", actual_output).unwrap();

    // The comparison happens automatically when `mint` goes out of scope!
}
