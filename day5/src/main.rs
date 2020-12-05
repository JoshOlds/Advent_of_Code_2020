use advent_libs::input_helpers;
#[derive(Default, Debug)]
struct BoardingPass {
    row: i32,
    col: i32,
    id: i32,
}
fn main() {
    println!("Advent of Code 2020 - Day 5");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(5);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of strings on newline
    let input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Parse and calculate Boarding Passes
    let mut passes: Vec<BoardingPass> = Vec::new();
    for line in input_vec.iter() {
        let mut pass: BoardingPass = Default::default();
        let mut upper = 127;
        let mut lower = 0;
        let mut left = 0;
        let mut right = 7;
        for c in line.chars() {
            match c {
                'F' => upper = upper - ((upper - lower) / 2) - 1,
                'B' => lower = lower + ((upper - lower) / 2) + 1,
                'R' => left = left + ((right - left) / 2) + 1,
                'L' => right = right - ((right - left) / 2) - 1,
                _ => panic!("Invalid character"),
            }
        }
        // Populate the Pass object
        pass.col = right;
        pass.row = upper;
        pass.id = (pass.row * 8) + pass.col;
        passes.push(pass);
    }

    // Part 1 -- Find the highest ID
    let mut highest = 0;
    for pass in passes.iter() {
        if pass.id > highest {
            highest = pass.id;
        }
    }
    println!("Part 1 -- Highest: {}", highest);

    // Part 2 -- Find the missing ID
    // Sort the vector to make it easy to find the ID gap
    passes.sort_by_key(|k| k.id);

    // Iterate through and find the missing ID in the list
    let mut last_id = passes[0].id;
    for pass in passes[1..].iter() {
        if pass.id != last_id + 1 {
            println!("Part 2 -- Your seat is: {}", pass.id - 1);
        }
        last_id = pass.id;
    }
}
