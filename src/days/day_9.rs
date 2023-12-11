use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_9.txt";

pub fn day_9(task: Task) -> i64 {
    match task {
        Task::TASK1 => day_9_task_1(),
        Task::TASK2 => day_9_task_2()
    }
}

fn day_9_task_1() -> i64 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    return read.lines().into_iter()
        .map(
            |line|
            {
                let mut items = line.split(' ').map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>();

                let mut retval: Vec<i64> = Vec::new();
                loop {
                    retval.push(*items.last().unwrap());

                    let mut diff: Vec<i64> = Vec::new();
                    for iter in 0..items.len()-1 {
                        diff.push(items[iter+1] - items[iter]);
                    }

                    if diff.iter().min() == diff.iter().max() {
                        retval.push(diff[0]);
                        break;
                    }

                    items = diff;
                };

                let x: i64 = retval.iter().sum();
                
                return x;
            }
        )
        .sum();
}

fn day_9_task_2() -> i64 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    return read.lines().into_iter()
        .map(
            |line|
            {
                let mut items = line.split(' ').map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>();

                let mut retval: Vec<i64> = Vec::new();
                loop {
                    retval.push(*items.first().unwrap());

                    let mut diff: Vec<i64> = Vec::new();
                    for iter in 0..items.len()-1 {
                        diff.push(items[iter+1] - items[iter]);
                    }

                    if diff.iter().min() == diff.iter().max() {
                        retval.push(diff[0]);
                        break;
                    }

                    items = diff;
                };

                retval.reverse();

                let mut current = 0;
                for item in retval {
                    // println!("Current: {}; ", current);
                    current = item - current;
                }
                // println!("Results: {}", current);
                return current;
            }
        )
        .sum();
}