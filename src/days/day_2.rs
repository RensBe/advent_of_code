use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_2.txt";

pub fn day_2(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_2_task_1(),
        Task::TASK2 => day_2_task_2()
    }
}

fn day_2_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    return read.lines().into_iter()
        .map(
            |line|
            {
                let pos_semicolon = line.find(':').unwrap();
                let iter = &line[5 .. pos_semicolon].parse::<u32>().unwrap();

                let sets = &line[pos_semicolon + 2 ..].split("; ");

                for set in sets.clone().into_iter() {
                    let colors = &set.split(", ");

                    for color in colors.clone().into_iter() {
                        let mut full_color = color.split(' ');
                        let color_count = full_color.next().unwrap().parse::<u32>().unwrap();
                        let color_text = full_color.next().unwrap();

                        let is_valid = match color_text {
                            "red" => color_count <= max_red,
                            "green" => color_count <= max_green,
                            "blue" => color_count <= max_blue,
                            _ => false
                        };

                        if !is_valid {
                            return 0;
                        }
                    }
                }

                return iter.clone();
            }
        ).sum();
}

fn day_2_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    return read.lines().into_iter()
        .map(
            |line|
            {
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;
                
                let sets = &line[line.find(':').unwrap() + 2 ..].split("; ");

                for set in sets.clone().into_iter() {
                    let colors = &set.split(", ");

                    for color in colors.clone().into_iter() {
                        let mut full_color = color.split(' ');
                        let color_count = full_color.next().unwrap().parse::<u32>().unwrap();
                        let color_text = full_color.next().unwrap();

                        match color_text {
                            "red" => if color_count > min_red {min_red = color_count},
                            "green" => if color_count > min_green {min_green = color_count},
                            "blue" => if color_count > min_blue {min_blue = color_count},
                            _ => ()
                        };
                    }
                }

                return min_red * min_green * min_blue;
            }
        ).sum();
}