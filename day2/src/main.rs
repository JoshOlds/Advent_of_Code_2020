use advent_libs::input_helpers;

struct Password {
    pub required_char: char,
    pub lower_requirement: usize,
    pub upper_requirement: usize,
    pub password: String,
}

fn main() {
    println!("Advent of Code 2020 - Day 2");
    println!("---------------------------");

    // Read in puzzle input
    let input = input_helpers::read_puzzle_input_to_string(2);
    // Parse to vector of strings on newline
    let input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Loop over the lines and create password objects
    let mut password_vec: Vec<Password> = Vec::new();
    for line in input_vec.iter() {
        // Create a vector of strings, split at '-'
        let mut data: Vec<&str> = line.split("-").collect();
        // Create another, splitting the second string by space
        let data2 = data[1].split(" ");
        // Merge the vectors
        data.pop();
        data.extend(data2);
        // Toss the ':' out of the character string
        data[2] = &data[2][..data[2].len() - 1];

        // Create the password object
        let pass = Password {
            required_char: data[2].chars().next().unwrap(),
            lower_requirement: data[0].parse().unwrap(),
            upper_requirement: data[1].parse().unwrap(),
            password: String::from(data[3]),
        };
        password_vec.push(pass);
    }

    // Verify passwords
    let mut valid_count = 0;
    for pass in &password_vec {
        let matches = pass.password.matches(pass.required_char).count();
        if matches <= pass.upper_requirement && matches >= pass.lower_requirement {
            valid_count += 1;
        }
    }
    println!("The amount of valid passwords is: {}", valid_count);

    // Part Two -------------------
    valid_count = 0;
    for pass in &password_vec {
        let mut match_once = false;
        if pass
            .password
            .chars()
            .nth(pass.lower_requirement - 1)
            .unwrap()
            == pass.required_char
        {
            match_once = true;
        }
        if pass
            .password
            .chars()
            .nth(pass.upper_requirement - 1)
            .unwrap()
            == pass.required_char
        {
            if match_once == true {
                match_once = false;
            } else {
                match_once = true;
            }
        }
        if match_once {
            valid_count += 1;
        }
    }

    println!("------- Part 2 -------");
    println!("The amount of valid passwords is: {}", valid_count);
}
