// This approach to day8 was MASSIVELY overkill. I wanted to delve into Rust's Trait system and learn more about dynamic types.
// I worked myself into a corner trying to use Traits like class inheritance. This mistake forced me to rethink data structures and
// approach today's challenge in a way that was different than how I would in another language (like C++).
// Overall, learned a lot, but took WAY too long.

use advent_libs::input_helpers;
use byte_codes::ByteCodeName;

type ByteMemory = Vec<Box<dyn byte_codes::ByteCode>>;

// Originally had the memory inside the CPU object. This meant I could not iterate through memory and mutate the CPU, since that would
// be a looped dependency. Borrow checker put a stop to that. Had to break memory out separately, which seemed strange for the design I was going for.
#[derive(Debug)]
pub struct CPU {
    // Accumulator
    pub acc: i32,
    // Instruction Pointer
    pub ip: i32,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { acc: 0, ip: 0 }
    }
    /// Runs the ByteCode at Instruction Pointer. Returns false if execution should halt
    pub fn run_one(&mut self, memory: &mut ByteMemory) -> Result<i32, String> {
        if self.ip >= memory.len() as i32 {
            return Err("Reached end of program".to_string());
        }

        let instruction = &mut memory[self.ip as usize];
        instruction.execute(self)?; // '?' operator propagates errors up
        Ok(self.ip)
    }
}

mod byte_codes {
    use crate::CPU;

    pub trait ByteCode {
        fn execute(&mut self, cpu: &mut CPU) -> Result<i32, String>;
        fn get_name(&self) -> ByteCodeName;
        fn get_val(&self) -> i32;
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum ByteCodeName {
        NoOp,
        Acc,
        Jmp,
    }
    /// No Operation - Does nothing, but input data seems to have a value attached anyway
    pub struct NoOp {
        pub name: ByteCodeName,
        pub val: i32,
        execution_count: usize,
    }
    impl NoOp {
        pub fn new(val: i32) -> NoOp {
            NoOp {
                name: ByteCodeName::NoOp,
                val,
                execution_count: 0,
            }
        }
    }
    impl ByteCode for NoOp {
        fn execute(&mut self, cpu: &mut CPU) -> Result<i32, String> {
            if self.execution_count > 0 {
                return Err("Instruction has already executed.".to_string());
            }
            self.execution_count += 1;
            cpu.ip += 1;
            Ok(self.val)
        }
        fn get_name(&self) -> ByteCodeName {
            self.name
        }
        fn get_val(&self) -> i32 {
            self.val
        }
    }

    /// Accumulate - Adds value embedded in instruction to accumulator
    pub struct Acc {
        pub name: ByteCodeName,
        pub val: i32,
        execution_count: usize,
    }
    impl Acc {
        pub fn new(val: i32) -> Acc {
            Acc {
                name: ByteCodeName::Acc,
                val,
                execution_count: 0,
            }
        }
    }
    impl ByteCode for Acc {
        fn execute(&mut self, cpu: &mut CPU) -> Result<i32, String> {
            if self.execution_count > 0 {
                return Err("Instruction has already executed.".to_string());
            }
            self.execution_count += 1;
            cpu.acc += self.val;
            cpu.ip += 1;
            Ok(self.val)
        }
        fn get_name(&self) -> ByteCodeName {
            self.name
        }
        fn get_val(&self) -> i32 {
            self.val
        }
    }

    /// Jump - Move instruction pointer by the value embedded in instruction
    pub struct Jmp {
        pub name: ByteCodeName,
        pub val: i32,
        execution_count: usize,
    }
    impl Jmp {
        pub fn new(val: i32) -> Jmp {
            Jmp {
                name: ByteCodeName::Jmp,
                val,
                execution_count: 0,
            }
        }
    }
    impl ByteCode for Jmp {
        fn execute(&mut self, cpu: &mut CPU) -> Result<i32, String> {
            if self.execution_count > 0 {
                return Err("Instruction has already executed.".to_string());
            }
            cpu.ip += self.val;
            Ok(self.val)
        }
        fn get_name(&self) -> ByteCodeName {
            self.name
        }
        fn get_val(&self) -> i32 {
            self.val
        }
    }
}

fn get_memory_from_input_vec(vec: &Vec<String>) -> ByteMemory {
    let mut mem: ByteMemory = Vec::new();
    // Populate memory from input
    for line in vec.iter() {
        let mut sign = 1;
        match &line[4..5] {
            "-" => sign = -1,
            "+" => {}
            _ => panic!("Invalid sign value: {}", &line[4..5]),
        }

        let value: i32 = line[5..].parse::<i32>().unwrap() * sign;

        let op = &line[0..3];
        match op {
            "acc" => mem.push(Box::new(byte_codes::Acc::new(value))),
            "nop" => mem.push(Box::new(byte_codes::NoOp::new(value))),
            "jmp" => mem.push(Box::new(byte_codes::Jmp::new(value))),
            _ => panic!("Unknown operation value: {}", op),
        }
    }
    mem
}

fn main() {
    println!("Advent of Code 2020 - Day 4");
    println!("---------------------------");

    // Read in puzzle input
    let mut input = input_helpers::read_puzzle_input_to_string(8);
    // Strip out the carriage returns (on Windows)
    input.retain(|c| c != '\r');
    // Parse to vector of strings on newline
    let input_vec: Vec<String> = input_helpers::split_string_to_vector(&input, "\n");

    // Create CPU and Memory
    let mut cpu = CPU::new();
    let mut mem = get_memory_from_input_vec(&input_vec);

    // Part 1 ---------------------
    let mut ok = true;
    while ok {
        match cpu.run_one(&mut mem) {
            Ok(_x) => {}
            Err(message) => {
                println!("{}", message);
                ok = false;
            }
        }
    }
    println!("{:?}", cpu);

    // Part 2 -----------------------
    // Replace nop and jmp one by one to find working solution
    for (index, inst) in mem.iter().enumerate() {
        let name = inst.get_name();
        if name == byte_codes::ByteCodeName::Jmp || name == ByteCodeName::NoOp {
            // Copy memory
            let mut new_cpu = CPU::new();
            let mut new_mem = get_memory_from_input_vec(&input_vec);
            // Swap out the instruction
            if name == byte_codes::ByteCodeName::Jmp {
                new_mem[index] = Box::new(byte_codes::NoOp::new(inst.get_val()))
            } else {
                new_mem[index] = Box::new(byte_codes::Jmp::new(inst.get_val()))
            }
            // Run the program and see if we get to the end
            let mut ok = true;
            while ok {
                match new_cpu.run_one(&mut new_mem) {
                    Ok(_x) => {}
                    Err(message) => {
                        if message.contains("Reached") {
                            println!("Reached end of program. Changed index: {}", index);
                            println!("{:?}", new_cpu);
                        }
                        ok = false;
                    }
                }
            }
        }
    }
}
