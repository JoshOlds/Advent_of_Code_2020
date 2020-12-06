use advent_libs::input_helpers;

fn main() {
    println!("Advent of Code 2020 - Day 3");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(3);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of strings on newline
    let input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Represent map as 2D vector of bools. True is tree, false is open
    let map_width = input_vec[0].len();
    let map_height = input_vec.len();
    let mut map = vec![vec![false; map_width]; map_height];

    // Populate map
    for (row, line) in input_vec.iter().enumerate() {
        for (col, character) in line.chars().enumerate() {
            match character {
                '.' => map[row][col] = false,
                '#' => map[row][col] = true,
                _ => panic!("Character encountered that is neither # nor ."),
            }
        }
    }

    // Uncomment for debug view of data sets
    //println!("{:?}", input_vec);
    //println!("{:?}", map);

    // Anonymous function (closure) to use for calculating trajectory
    fn calculate_trajectory(
        map: &Vec<Vec<bool>>,
        map_height: usize,
        map_width: usize,
        horz_delta: usize,
        vert_delta: usize,
    ) -> usize {
        let mut tree_count: usize = 0;
        let mut row: usize = 0;
        let mut col: usize = 0;
        while row < map_height {
            if map[row][col] == true {
                tree_count += 1;
            }
            row += vert_delta;
            col += horz_delta;
            // Rollover
            if col >= map_width {
                col -= map_width;
            }
        }
        tree_count
    };

    // ------------------------------------
    // Calculate part 1 - Trees encountered with right:3 down:1 trajectory
    println!(
        "Trees encountered on initial trajectory: {}",
        calculate_trajectory(&map, map_height, map_width, 3, 1)
    );

    // Calculate part 2
    let multiplied_trees = calculate_trajectory(&map, map_height, map_width, 1, 1)
        * calculate_trajectory(&map, map_height, map_width, 3, 1)
        * calculate_trajectory(&map, map_height, map_width, 5, 1)
        * calculate_trajectory(&map, map_height, map_width, 7, 1)
        * calculate_trajectory(&map, map_height, map_width, 1, 2);
    println!(
        "Trees encountered on multiplied trajectory: {}",
        multiplied_trees
    );
}
