// This solution turned into a bear. Use of recursive functions to dig through Bags... However bag storage was initialized outside of class. Ended up passing in outside storage to member functions, eck!
// Used a hash map to speed up searching. Solution is much faster than digging through a Vector, and uses a small amount of memory, since full bag copies are not stored within other bags.

use advent_libs::input_helpers;
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
struct BagContent(String, i32);

#[derive(Clone, Default, Debug)]
struct Bag {
    name: String,
    contents: Vec<BagContent>,
}

impl Bag {
    fn contains(&self, bags: &HashMap<String, Bag>, name: &String) -> bool {
        // Check child names first
        for content in self.contents.iter() {
            if content.0 == *name {
                return true;
            }
        }
        // recursive search
        for content in self.contents.iter() {
            let child = bag_lookup(bags, &content.0).unwrap();
            if child.contains(bags, name) {
                return true;
            }
        }
        return false;
    }

    // This function returns 1 count to many due to starting the intial count at 1... Didn't fix, out of time
    fn must_contain(&self, bags: &HashMap<String, Bag>) -> i32 {
        let mut count = 1;
        for content in self.contents.iter() {
            let child = bag_lookup(bags, &content.0).unwrap();
            count += content.1 * child.must_contain(bags);
        }
        count
    }
}

fn bag_lookup<'a>(bags: &'a HashMap<String, Bag>, name: &String) -> Option<&'a Bag> {
    bags.get(name)
}

fn main() {
    println!("Advent of Code 2020 - Day 7");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(7);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');

    // Parse into lines
    let lines: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // HashTable of bags for faster lookup
    let mut bag_hash: HashMap<String, Bag> = HashMap::new();

    // Iterate through lines and do some nasty string parsing
    for line in lines.iter() {
        let words: Vec<String> = input_helpers::split_string_to_vector(line, " ");
        let name = format!("{} {}", words[0], words[1]);
        let mut bag = Bag {
            name: name,
            contents: Vec::new(),
        };

        //Case where no other bags are contained
        if words.contains(&"other".to_string()) {
        } else {
            let mut name = String::from("");
            let mut val = 0;
            for (index, word) in words.iter().enumerate() {
                if index <= 3 {
                    continue;
                }
                // Number of bags
                if (index - 4) % 4 == 0 {
                    val = word.parse().unwrap();
                    continue;
                }
                // first part of name
                if (index - 4) % 4 == 1 {
                    name = String::from(word) + " ";
                    continue;
                }
                // Second part of name
                if (index - 4) % 4 == 2 {
                    name += word;
                    let content = BagContent(String::from(&name), val);
                    bag.contents.push(content);
                    continue;
                }
            }
        }
        // Add Bag to HashMap
        bag_hash.insert(String::from(&bag.name), bag);
    }

    // Debug print if interested
    // println!("{:?}", bag_hash);

    // Part 1 -------------
    // Search the tree structure for 'shiny gold' bag
    let mut count = 0;
    for (_key, bag) in bag_hash.iter() {
        if bag.contains(&bag_hash, &"shiny gold".to_string()) {
            count += 1;
        }
    }
    println!(
        "Part 1 - Bags that may eventually contain 'shiny gold' bag: {}",
        count
    );

    // Part 2 ------------
    println!(
        " Part 2 - 'shiny gold' bag must contain: {} other bags.",
        bag_hash.get("shiny gold").unwrap().must_contain(&bag_hash) - 1
    );
}
