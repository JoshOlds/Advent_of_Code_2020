use advent_libs::input_helpers;
use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2020 - Day 6");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(6);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');

    // Parse into 'groups' and then 'people'
    let mut groups_vec: Vec<Vec<String>> = Vec::new();

    for group in input_helpers::split_string_to_vector::<String>(&input, "\n\n").into_iter() {
        groups_vec.push(input_helpers::split_string_to_vector::<String>(
            &group, "\n",
        ));
    }

    // Create hash map of yes's for each group
    let mut group_hashes: Vec<HashMap<char, i32>> = Vec::new();

    for group in groups_vec.iter() {
        let mut hash: HashMap<char, i32> = HashMap::new();
        for line in group.iter() {
            for c in line.chars() {
                let entry = hash.entry(c).or_insert(0);
                *entry += 1;
            }
        }
        group_hashes.push(hash);
    }

    // Debug if curious
    //println!("{:?}", group_hashes);

    // Part 1 ---------
    let mut yes_count = 0;
    for hash in group_hashes.iter() {
        yes_count += hash.keys().len();
    }
    println!(
        "Part 1 - Total unique YES count of all groups: {}",
        yes_count
    );

    // Part 2 ----------
    yes_count = 0;
    for (index, hash) in group_hashes.iter().enumerate() {
        for (_key, value) in hash.iter() {
            if *value == groups_vec[index].len() as i32 {
                yes_count += 1;
            }
        }
    }
    println!("Part 1 - Total ALL YES count of all groups: {}", yes_count);
}
