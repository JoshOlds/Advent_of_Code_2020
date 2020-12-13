use advent_libs::input_helpers;

/// Ship struct is used to represent both a ship and a waypoint
#[derive(Debug)]
struct Ship {
    /// Heading of 0 represents EAST. Positive rotation is clockwise (E,S,W,N)
    pub heading: i32,
    pub lat: i32,
    pub lon: i32,
}

impl Ship {
    fn new(heading: i32, lat: i32, lon: i32) -> Ship {
        Ship { heading, lat, lon }
    }
}

fn process_command(ship: &mut Ship, cmd: &String) {
    let val: i32 = cmd[1..].parse().unwrap();
    let cmd = &cmd[0..1];

    match cmd {
        "N" => ship.lon += val,
        "S" => ship.lon -= val,
        "E" => ship.lat += val,
        "W" => ship.lat -= val,
        "R" => {
            ship.heading += val;
            ship.heading = ship.heading % 360;
        }
        "L" => {
            ship.heading -= val;
            if ship.heading < 0 {
                ship.heading += 360;
            }
        }
        "F" => match ship.heading {
            0 => ship.lat += val,
            90 => ship.lon -= val,
            180 => ship.lat -= val,
            270 => ship.lon += val,
            _ => panic!("Invalid heading"),
        },
        _ => panic!("Invalid command"),
    }
}

fn process_command_part2(ship: &mut Ship, waypoint: &mut Ship, cmd: &String) {
    // Local rotation functions
    fn rotate_left(waypoint: &mut Ship) {
        let lon = waypoint.lon * -1;
        waypoint.lon = waypoint.lat;
        waypoint.lat = lon;
    }
    fn rotate_right(waypoint: &mut Ship) {
        let lat = waypoint.lat * -1;
        waypoint.lat = waypoint.lon;
        waypoint.lon = lat;
    }

    let val: i32 = cmd[1..].parse().unwrap();
    let cmd = &cmd[0..1];

    match cmd {
        "N" => waypoint.lon += val,
        "S" => waypoint.lon -= val,
        "E" => waypoint.lat += val,
        "W" => waypoint.lat -= val,
        "R" => {
            for _x in 0..(val / 90) {
                rotate_right(waypoint);
            }
        }
        "L" => {
            for _x in 0..(val / 90) {
                rotate_left(waypoint);
            }
        }
        "F" => {
            ship.lat += waypoint.lat * val;
            ship.lon += waypoint.lon * val;
        }
        _ => panic!("Invalid command"),
    }
}

fn main() {
    println!("Advent of Code 2020 - Day 12");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(12);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of usize on newline
    let input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Create ship object
    let mut ship = Ship::new(0, 0, 0);

    // Part 1
    for cmd in input_vec.iter() {
        process_command(&mut ship, cmd);
    }
    let manhattan_dist = ship.lat.abs() + ship.lon.abs();
    println!("Part 1 -");
    println!("{:?}", &ship);
    println!("Manhattan Distance: {}", manhattan_dist);

    // Part 2
    ship = Ship::new(0, 0, 0); // Reset ship
    let mut waypoint = Ship::new(0, 10, 1); // Waypoint starts E:10 and N:1 of the ship

    for cmd in input_vec.iter() {
        process_command_part2(&mut ship, &mut waypoint, cmd);
    }

    let manhattan_dist = ship.lat.abs() + ship.lon.abs();
    println!("Part 2 -");
    println!("{:?}", &ship);
    println!("Manhattan Distance: {}", manhattan_dist);
}
