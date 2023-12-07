use std::{fs, thread::{self, JoinHandle}};

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_6.txt";

pub fn day_6(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_6_task_1(),
        Task::TASK2 => day_6_task_2()
    }
}

fn day_6_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = read.lines();
    let times: Vec<usize> = lines.next().unwrap().split(' ').filter(|text| text.parse::<usize>().is_ok()).map(|text| text.parse::<usize>().unwrap()).collect();
    let distances: Vec<usize> = lines.next().unwrap().split(' ').filter(|text| text.parse::<usize>().is_ok()).map(|text| text.parse::<usize>().unwrap()).collect();

    let races: Vec<(usize, usize)> = times.iter().enumerate().into_iter().map(|time| (distances[time.0], *time.1)).collect();

    let mut retval = 1;
    for race in races {
        let mut wins = 0;
        for iter in 0..race.1 {
            let speed = iter;
            let time = race.1 - iter;

            if speed * time > race.0 {
                wins += 1;
            }
        }
        retval *= wins;
    }

    return retval;
}

fn day_6_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = read.lines();
    let time: String = lines.next().unwrap().chars().filter(|c| c.is_numeric()).collect();
    let time = time.parse::<usize>().unwrap();
    
    let distance: String = lines.next().unwrap().chars().filter(|c| c.is_numeric()).collect();
    let distance = distance.parse::<usize>().unwrap();

    let mut wins = 0;
    for iter in 0..time {
        let speed = iter;
        let time = time - iter;

        if speed * time > distance {
            wins += 1;
        }
    }

    return wins;
}