use std::time::SystemTime;

use days::day_11::day_11;

pub mod days;

pub enum Task {
    TASK1,
    TASK2
}

fn main() {
    let now = SystemTime::now();

    println!("Result: {}, Duration: {}", day_11(Task::TASK2), now.elapsed().unwrap().as_micros());
}