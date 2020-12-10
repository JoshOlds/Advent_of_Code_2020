// Pretty happy with this one. Use of a hash table makes for quick calculation and minimum (maybe?) amount of summing needed.

use advent_libs::input_helpers;
use std::collections::HashMap;

fn find_invalid(input: &Vec<usize>, preamble_length: usize) -> (&[usize], usize) {
    // Preamble represented of hash of value and count
    let mut preamble: HashMap<usize, usize> = HashMap::new();

    // Populate preamble
    for (index, val) in input[0..preamble_length].iter().enumerate() {
        for next_val in input[index + 1..preamble_length].iter() {
            let entry = preamble.entry(val + next_val).or_insert(0);
            *entry += 1;
        }
    }

    // Loop through the rest of the data
    for (index, val) in input[preamble_length..].iter().enumerate() {
        // Check if value is in hash
        let entry = preamble.entry(*val).or_insert(0);
        // If 0, we found a match that is not a sum of preamble values
        if *entry == 0 {
            return (&input[index - preamble_length..index], *val);
        }
        // Now that we have checked, remove the sum results of the first value in the preamble from the hash
        // While we are here, may as well add the new preamble value
        for preamble_val in input[index + 1..index + preamble_length].iter() {
            // Old preamble value to be removed
            let sum = preamble_val + input[index];
            let entry = preamble.entry(sum).or_default();
            *entry -= 1;

            // New value
            let sum = preamble_val + input[index + preamble_length];
            let entry = preamble.entry(sum).or_default();
            *entry += 1;
        }
    }

    return (&[0], 0);
}

fn find_contiguous_set(input: &Vec<usize>, match_val: usize) -> (&[usize], usize) {
    // Iterate through the whole input and looks for sets that sum to the match value
    for (index, base_val) in input.iter().enumerate() {
        let mut sum = *base_val;
        // Don't iterate over previous values
        for (set_index, val) in input[index + 1..].iter().enumerate() {
            sum += val;
            // If we found a set, find the smallest and largest in the set
            if sum == match_val {
                let mut smallest = match_val;
                let mut largest = 0;
                for set_val in input[index..index + set_index + 2].iter() {
                    if *set_val < smallest {
                        smallest = *set_val;
                    }
                    if *set_val > largest {
                        largest = *set_val;
                    }
                }
                // Return the set (as a reference to a slice of the vector) as well as the sum of smallest and largest
                return (&input[index..index + set_index + 2], smallest + largest);
            }
            // If we exceeded the match_val, break
            if sum >= match_val {
                break;
            }
        }
    }
    return (&[0], 0);
}

fn main() {
    println!("Advent of Code 2020 - Day 9");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(9);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of integers on newline
    let input_vec: Vec<usize> = input_helpers::split_string_to_vector(&input, "\n");

    // Part 1 -----------------------
    let (preamble, invalid) = find_invalid(&input_vec, 25);
    println!(
        "Part 1 - First invalid number: {}, with preamble: {:?}",
        invalid, preamble
    );

    // Part 2 -----------------------
    let (set, sum) = find_contiguous_set(&input_vec, invalid);
    println!(
        "Part 2 - The contiguous set that sums to {} is: {:?}",
        invalid, set
    );
    println!("The sum of the smallest and largest set value is: {}", sum);
}
