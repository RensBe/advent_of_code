use std::time::SystemTime;

use crate::days::{day_1::day_1, day_2::day_2, day_3::day_3, day_4::day_4, day_5::day_5, day_6::day_6, day_7::day_7};

pub mod days;

pub enum Task {
    TASK1,
    TASK2
}

fn main() {
    let now = SystemTime::now();

    // println!("Result: {}, Duration: {}", day_1(Task::TASK1), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_1(Task::TASK2), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_2(Task::TASK1), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_2(Task::TASK2), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_3(Task::TASK1), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_3(Task::TASK2), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_4(Task::TASK1), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_4(Task::TASK2), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_5(Task::TASK1), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_5(Task::TASK2), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_6(Task::TASK1), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_6(Task::TASK2), now.elapsed().unwrap().as_micros());
    // println!("Result: {}, Duration: {}", day_7(Task::TASK1), now.elapsed().unwrap().as_micros());
    println!("Result: {}, Duration: {}", day_7(Task::TASK2), now.elapsed().unwrap().as_micros());

    // let mut xs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // for x in xs {
    //     print!("{x}; ");
    // }
}