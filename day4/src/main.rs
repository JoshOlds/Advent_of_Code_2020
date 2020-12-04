use advent_libs::input_helpers;

#[derive(Clone, PartialEq, Debug)]
enum HeightUnit {
    Invalid,
    Centimeters,
    Inches,
}
impl Default for HeightUnit {
    fn default() -> Self {
        HeightUnit::Invalid
    }
}

#[derive(Clone, Default, Debug)]
struct Passport {
    byr: i32,
    has_byr: bool,
    iyr: i32,
    has_iyr: bool,
    eyr: i32,
    has_eyr: bool,
    hgt: i32,
    has_hgt: bool,
    hgt_unit: HeightUnit,
    hcl: String,
    has_hcl: bool,
    ecl: String,
    has_ecl: bool,
    pid: String,
    has_pid: bool,
    cid: i32,
    has_cid: bool,
}
fn main() {
    println!("Advent of Code 2020 - Day 4");
    println!("---------------------------");

    // Read in puzzle input
    let input = input_helpers::read_puzzle_input_to_string(4);
    // Parse to vector of strings on newline
    let mut input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Create a vector of passport objects
    let mut passport_vec: Vec<Passport> = Vec::new();
    // Construct a default passport object
    let mut passport: Passport = Default::default();

    // Convert data into passport objects
    for line in input_vec.iter_mut() {
        // Save and reset the passport when we hit a newline
        // Input on windows finds a carriage return, linux has blank data! Eww!
        if line == "\r" || line == "" {
            passport_vec.push(passport.clone());
            passport = Default::default();
            continue;
        }

        // remove trailing carriage return
        if line.chars().last().unwrap() == '\r' {
            line.pop();
        }
        // Split out all of the fields of a particular line
        let line_vec: Vec<String> = input_helpers::split_string_to_vector(&line, " ");

        // Iterate over fields and populate Passport
        for field in line_vec.iter() {
            // Match on the first three characters
            match &field[..3] {
                "byr" => {
                    passport.byr = field[4..].parse().unwrap();
                    passport.has_byr = true;
                }
                "iyr" => {
                    passport.iyr = field[4..].parse().unwrap();
                    passport.has_iyr = true;
                }
                "eyr" => {
                    passport.eyr = field[4..].parse().unwrap();
                    passport.has_eyr = true;
                }
                "hgt" => {
                    match field[4..].parse::<i32>() {
                        // Expect error where there is a height unit, handle by truncating the 2-letter unit
                        Ok(hgt) => passport.hgt = hgt,
                        // If we encounter units, parse them appropriately
                        Err(_e) => {
                            passport.hgt = field[4..field.len() - 2].parse().unwrap();
                            match &field[field.len() - 2..] {
                                "in" => passport.hgt_unit = HeightUnit::Inches,
                                "cm" => passport.hgt_unit = HeightUnit::Centimeters,
                                _ => panic!("Unknown height unit encountered"),
                            }
                        }
                    }
                    passport.has_hgt = true;
                }
                "hcl" => {
                    passport.hcl = field[4..].to_string();
                    passport.has_hcl = true;
                }
                "ecl" => {
                    passport.ecl = field[4..].to_string();
                    passport.has_ecl = true;
                }
                "pid" => {
                    passport.pid = field[4..].to_string();
                    passport.has_pid = true;
                }
                "cid" => {
                    passport.cid = field[4..].parse().unwrap();
                    passport.has_cid = true;
                }
                _ => panic!("Unrecognized value! {}", &field[..2]),
            }
        }
    }
    // Push the final passport
    passport_vec.push(passport.clone());

    // Part 1 -----------------
    // Count 'good' passports
    let mut good_count = 0;
    for passport in passport_vec.iter() {
        if passport.has_byr
            && passport.has_iyr
            && passport.has_eyr
            && passport.has_hgt
            && passport.has_hcl
            && passport.has_ecl
            && passport.has_pid
        {
            good_count += 1;
        }
    }
    println!("Good passport count: {}", good_count);

    // Part 2 ---------------------
    // Extra validation
    good_count = 0;
    for passport in passport_vec.iter() {
        if passport.byr < 1920 || passport.byr > 2002 {
            continue;
        }
        if passport.iyr < 2010 || passport.iyr > 2020 {
            continue;
        }
        if passport.eyr < 2020 || passport.eyr > 2030 {
            continue;
        }
        if passport.hgt_unit == HeightUnit::Invalid {
            continue;
        }
        if passport.hgt_unit == HeightUnit::Centimeters {
            if passport.hgt < 150 || passport.hgt > 193 {
                continue;
            }
        }
        if passport.hgt_unit == HeightUnit::Inches {
            if passport.hgt < 59 || passport.hgt > 76 {
                continue;
            }
        }
        if passport.hcl.len() != 7 {
            continue;
        }
        for (index, letter) in passport.hcl.chars().enumerate() {
            if index == 0 {
                if letter != '#' {
                    continue;
                }
            }
            // TYay standard library
            else if letter.is_ascii_hexdigit() == false {
                continue;
            }
        }
        if passport.ecl != "amb"
            && passport.ecl != "blu"
            && passport.ecl != "brn"
            && passport.ecl != "gry"
            && passport.ecl != "grn"
            && passport.ecl != "hzl"
            && passport.ecl != "oth"
        {
            continue;
        }

        if passport.pid.len() != 9 {
            continue;
        }
        // If it doesn't parse to an int, its not a number
        match passport.pid.parse::<i32>() {
            Ok(_v) => {}
            Err(_e) => continue,
        }

        good_count += 1;
    }

    println!(
        "Good count with additional verification rules: {}",
        good_count
    );
}
