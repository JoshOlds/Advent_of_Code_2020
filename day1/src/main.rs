use advent_libs::input_helpers;

fn main() {
    println!("Advent of Code 2020 - Day 1");
    println!("---------------------------");

    // Read puzzle input
    let mut input_str = input_helpers::read_puzzle_input_to_string(1);
    // Strip out the carriage returns (on Windows)
    input_str.retain(|c| c != '\r');
    // Convert to vector of i32
    let input_vec: Vec<i32> = input_helpers::split_string_to_vector(&input_str, "\n");

    // Search for first match
    for (index, val1) in input_vec.iter().enumerate() {
        // Don't iterate over pairs we have already evaluated
        // Rust lets us iterate over a 'slice' of a vector
        for val2 in input_vec[index + 1..].iter() {
            if val1 + val2 == 2020 {
                println!("Found value match: {} + {} == 2020", val1, val2);
                println!("{} * {} == {}", val1, val2, val1 * val2);
            }
        }
    }

    // Search for second match
    for (index1, val1) in input_vec.iter().enumerate() {
        for (index2, val2) in input_vec[index1 + 1..].iter().enumerate() {
            for val3 in input_vec[index2 + index1 + 1..].iter() {
                if val1 + val2 + val3 == 2020 {
                    println!("Found value match: {} + {} + {} == 2020", val1, val2, val3);
                    println!("{} * {} * {} == {}", val1, val2, val3, val1 * val2 * val3);
                }
            }
        }
    }
}
