use crate::common;
use std::collections::VecDeque;
use std::path::Path;
use std::str::FromStr;

fn load_file() -> Vec<u32> {
    let input_file_path = Path::new(r"input_data\day1.txt");
    let out = common::load_file(input_file_path);
    let actual_out = out
        .iter()
        .map(|x| u32::from_str(x).expect(&*format!("Couldn't parse input: {}", x)))
        .collect();
    actual_out
}

pub fn solve_day1_base() -> u32 {
    let out = load_file();
    let mut previous = u32::MAX;
    let mut increases = 0;
    for number in out {
        if number > previous {
            increases += 1;
        }
        previous = number;
    }
    increases
}

struct MeasurementWindow {
    window_queue: VecDeque<u32>,
}

impl MeasurementWindow {
    fn sum(&self) -> u32 {
        self.window_queue.iter().sum()
    }

    fn clear(&mut self) {
        println!("Cleaning Measurement window");
        self.window_queue.clear()
    }

    fn add(&mut self, val: u32) {
        if self.window_queue.len() == 3 {
           self.window_queue.clear()
        }
        self.window_queue.push_back(val);
    }

    fn new() -> MeasurementWindow {
        let deque = VecDeque::with_capacity(3);
        MeasurementWindow {
            window_queue: deque,
        }
    }
}

pub fn solve_day1_add() -> u32 {
    let out = load_file();
    let mut measurement_window = MeasurementWindow::new();
    let out_max_size = out.len();
    let mut previous_measurement_window_sum = u32::MAX;
    let mut increases = 0;
    for i in 0..out_max_size - 2 {
        //println!("New measurement window");
        for j in 0..3 {
            // println!("Adding {} to measurement window", out[i+j]);
            measurement_window.add(out[i+j]);
        }
        if measurement_window.sum() > previous_measurement_window_sum {
            increases += 1;
        }
        previous_measurement_window_sum = measurement_window.sum()
    }
    increases
}
