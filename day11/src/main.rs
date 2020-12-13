use advent_libs::input_helpers;

// PART TWO WAS A JERK

#[derive(Debug, PartialEq, Copy, Clone)]
enum Seating {
    Floor,
    OpenSeat,
    OccupiedSeat,
}

fn count_seating(seating_vec: &Vec<Vec<Seating>>) -> usize {
    let mut count = 0;
    for row in seating_vec.iter() {
        for seat in row.iter() {
            match seat {
                Seating::OccupiedSeat => count += 1,
                _ => {}
            }
        }
    }
    count
}

fn run_once(seating_vec: &mut Vec<Vec<Seating>>) -> Vec<Vec<Seating>> {
    let mut temp_seating_vec = seating_vec.clone();
    let width = seating_vec[0].len();
    let height = seating_vec.len();

    for (y, row) in seating_vec.iter().enumerate() {
        for (x, seat) in row.iter().enumerate() {
            // Short circuit if this is open floor
            if *seat == Seating::Floor {
                continue;
            }

            let mut occupied_adjacent = 0;

            // Check row above
            if y > 0 {
                if x > 0 {
                    if seating_vec[y - 1][x - 1] == Seating::OccupiedSeat {
                        occupied_adjacent += 1;
                    }
                }
                if seating_vec[y - 1][x] == Seating::OccupiedSeat {
                    occupied_adjacent += 1;
                }
                if x < width - 1 {
                    if seating_vec[y - 1][x + 1] == Seating::OccupiedSeat {
                        occupied_adjacent += 1;
                    }
                }
            }

            // Check the same row
            if x > 0 {
                if seating_vec[y][x - 1] == Seating::OccupiedSeat {
                    occupied_adjacent += 1;
                }
            }
            if x < width - 1 {
                if seating_vec[y][x + 1] == Seating::OccupiedSeat {
                    occupied_adjacent += 1;
                }
            }

            // Check the row below
            if y < height - 1 {
                if x > 0 {
                    if seating_vec[y + 1][x - 1] == Seating::OccupiedSeat {
                        occupied_adjacent += 1;
                    }
                }
                if seating_vec[y + 1][x] == Seating::OccupiedSeat {
                    occupied_adjacent += 1;
                }
                if x < width - 1 {
                    if seating_vec[y + 1][x + 1] == Seating::OccupiedSeat {
                        occupied_adjacent += 1;
                    }
                }
            }

            if *seat == Seating::OpenSeat {
                if occupied_adjacent == 0 {
                    temp_seating_vec[y][x] = Seating::OccupiedSeat;
                }
            } else {
                if occupied_adjacent >= 4 {
                    temp_seating_vec[y][x] = Seating::OpenSeat;
                }
            }
        }
    }
    temp_seating_vec
}

// Part 2 funcs
fn search_left(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut x = x;
    while x > 0 {
        x -= 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_right(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut x = x;
    while x < vec[0].len() - 1 {
        x += 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_up(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut y = y;
    while y > 0 {
        y -= 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_down(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut y = y;
    while y < vec.len() - 1 {
        y += 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_up_left(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut y = y;
    let mut x = x;
    while y > 0 && x > 0 {
        y -= 1;
        x -= 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_up_right(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut y = y;
    let mut x = x;
    while y > 0 && x < vec[0].len() - 1 {
        y -= 1;
        x += 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_down_left(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut y = y;
    let mut x = x;
    while y < vec.len() - 1 && x > 0 {
        y += 1;
        x -= 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn search_down_right(vec: &Vec<Vec<Seating>>, y: usize, x: usize) -> bool {
    let mut y = y;
    let mut x = x;
    while y < vec.len() - 1 && x < vec[0].len() - 1 {
        y += 1;
        x += 1;
        if vec[y][x] == Seating::OccupiedSeat {
            return true;
        }
        if vec[y][x] == Seating::OpenSeat {
            return false;
        }
    }
    return false;
}
fn run_part2_once(seating_vec: &mut Vec<Vec<Seating>>) -> Vec<Vec<Seating>> {
    let mut temp_seating_vec = seating_vec.clone();

    for (y, row) in seating_vec.iter().enumerate() {
        for (x, seat) in row.iter().enumerate() {
            if *seat == Seating::Floor {
                continue;
            }

            let mut occupied_count = 0;
            if search_down(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_left(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_right(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_up(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_down_right(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_down_left(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_up_left(seating_vec, y, x) {
                occupied_count += 1;
            }
            if search_up_right(seating_vec, y, x) {
                occupied_count += 1;
            }

            if occupied_count >= 5 {
                temp_seating_vec[y][x] = Seating::OpenSeat;
            } else if occupied_count == 0 {
                temp_seating_vec[y][x] = Seating::OccupiedSeat;
            }
        }
    }
    temp_seating_vec
}

fn main() {
    println!("Advent of Code 2020 - Day 11");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(11);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of usize on newline
    let input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Parse the seating into a 2D vector of booleans
    let mut vec: Vec<Vec<Seating>> = Vec::new();

    for (row, line) in input_vec.iter().enumerate() {
        vec.push(Vec::new());
        for char in line.chars() {
            match char {
                '.' => vec[row].push(Seating::Floor),
                'L' => vec[row].push(Seating::OpenSeat),
                _ => panic!("Invalid input character found"),
            }
        }
    }

    // Copy for part 2
    let mut vec_2 = vec.clone();

    // Part 1 -----------
    let mut same = false;
    let mut run_count = 0;
    while same == false {
        let next_vec = run_once(&mut vec);
        same = next_vec == vec;
        vec = next_vec;
        run_count += 1;
    }
    println!("Part 1 - Cycles until stabilization: {}", run_count);
    println!("Stabilized occupied count: {}", count_seating(&vec));

    // Part 2 -----------
    let mut same = false;
    let mut run_count = 0;
    while same == false {
        let next_vec = run_part2_once(&mut vec_2);
        same = next_vec == vec_2;
        vec_2 = next_vec;
        run_count += 1;
    }
    println!("Part 2 - Cycles until stabilization: {}", run_count);
    println!("Stabilized occupied count: {}", count_seating(&vec_2));
}
