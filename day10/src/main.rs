use advent_libs::input_helpers;

// PART TWO WAS A JERK

fn main() {
    println!("Advent of Code 2020 - Day 10");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(10);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of usize on newline
    let mut input_vec: Vec<usize> = input_helpers::split_string_to_vector(&input, "\n");
    // Sort the vector to speed up searching
    input_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Add the implied first and last value to the vector (based on wall input of 0 and device of max adapter + 3)
    input_vec.insert(0, 0);
    input_vec.push(input_vec.last().unwrap() + 3);

    // Copy to second vector, since part 1 manipulates vector in place
    let input_vec2 = input_vec.clone();

    // ------------------
    let adapter_count = input_vec.len();
    let mut current_joltage: usize = 0;

    // let mut jolt_delta_0 = 0;
    let mut jolt_delta_1 = 0;
    let mut jolt_delta_2 = 0;
    let mut jolt_delta_3 = 0;

    // Should have solved this same way as part 2... leaving to show original thought process
    for _i in 0..adapter_count {
        for (delta, desired_output) in (current_joltage + 1..current_joltage + 4).enumerate() {
            let mut found = false;
            for (index, output) in input_vec.iter().enumerate() {
                if *output == desired_output {
                    current_joltage = *output;
                    input_vec.remove(index);
                    match delta {
                        0 => jolt_delta_1 += 1,
                        1 => jolt_delta_2 += 1,
                        2 => jolt_delta_3 += 1,
                        _ => panic!("Illegal jolt delta!"),
                    }
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    // Part 1 ------------
    println!("Part 1 - ");
    println!("Jolt Delta 1 count: {}", jolt_delta_1);
    println!("Jolt Delta 2 count: {}", jolt_delta_2);
    println!("Jolt Delta 3 count: {}", jolt_delta_3);
    println!("Delta 1 * Delta 3 = {}", jolt_delta_1 * jolt_delta_3);

    // Part 2 ------------

    // convert to list of deltas
    // WOW, should have done this to solve part 1!
    let deltas: Vec<usize> = input_vec2.windows(2).map(|w| w[1] - w[0]).collect();

    // Solution is based on contiguous delta's of one
    // For a single 1 delta, there is only one permutation (no numbers can be removed)
    // For two contiguous single deltas, there are two permutation
    // For three contiguous single deltas, there are four permutation
    // For four contiguous single deltas, there are seven permutation
    // This seems to actually be the tribonacci sequence... My input has no more than 4 contiguous one deltas
    // Single one deltas have no permutations
    let mut x: i32 = 0; // Double 1 delta
    let mut y: i32 = 0; // Triple 1 delta
    let mut z: i32 = 0; // Quad 1 delta

    let mut count = 0;
    for delta in deltas.iter() {
        if *delta == 1 {
            count += 1;
        } else {
            match count {
                2 => x += 1,
                3 => y += 1,
                4 => z += 1,
                _ => {}
            }
            count = 0;
        }
    }

    // Answer is the product of all permutations
    let permutations = 2_i64.pow(x as u32) * 4_i64.pow(y as u32) * 7_i64.pow(z as u32);

    println!(
        "Part 2 - The number of legal permutations is: {}",
        permutations
    );
}
