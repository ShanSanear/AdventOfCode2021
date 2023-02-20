use crate::common;
use std::collections::VecDeque;
use std::fmt::format;
use std::path::Path;
use std::str::FromStr;

enum Command {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Command::Forward),
            "down" => Ok(Command::Down),
            "up" => Ok(Command::Up),
            _ => Err(format!("Invalid value for Command: {}", s)),
        }
    }
}

struct CommandWithValue {
    command: Command,
    value: u32,
}

impl FromStr for CommandWithValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid format of input: {}", s));
        }

        let command: Command = parts[0].parse()?;
        let value: u32 = parts[1]
            .parse()
            .map_err(|e| format!("Invalid argument: {}", e))?;
        Ok(CommandWithValue { command, value })
    }
}

fn load_file(test_data: bool) -> Vec<CommandWithValue> {
    let input_file_path: &Path;
    if test_data {
        input_file_path = Path::new(r"input_data\day2_example.txt");
    } else {
        input_file_path = Path::new(r"input_data\day2.txt");
    }
    let out = common::load_file(input_file_path);
    let actual_out = out
        .iter()
        .map(|x| CommandWithValue::from_str(x).expect(&*format!("Couldn't parse input: {}", x)))
        .collect();
    actual_out
}

pub fn solve_day2_base() -> u32 {
    let out = load_file(false);
    let mut depth = 0;
    let mut horizontal = 0;
    for command_with_value in out {
        match command_with_value.command {
            Command::Forward => horizontal += command_with_value.value,
            Command::Down => depth += command_with_value.value,
            Command::Up => depth -= command_with_value.value,
        }
    }
    depth * horizontal
}

pub fn solve_day2_add(test_data: bool) -> u32 {
    let out = load_file(test_data);
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for command_with_value in out {
        match command_with_value.command {
            Command::Forward => {
                horizontal += command_with_value.value;
                depth += aim * command_with_value.value;
            }
            Command::Down => {
                aim += command_with_value.value;
            }
            Command::Up => {
                aim -= command_with_value.value;
            }
        }
    }
    depth * horizontal
}
