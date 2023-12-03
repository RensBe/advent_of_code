use std::time::SystemTime;

use crate::days::day_3::day_3;

pub mod days;

pub enum Task {
    TASK1,
    TASK2
}

fn main() {
    let now = SystemTime::now();

    //other();
    println!("result: {}", day_3(Task::TASK2));
    
    println!("duration: {}", now.elapsed().unwrap().as_micros())
}