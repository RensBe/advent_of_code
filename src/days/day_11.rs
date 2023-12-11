use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_11.txt";

pub fn day_11(task: Task) -> usize {
    match task {
        Task::TASK1 => day_11_task(1),
        Task::TASK2 => day_11_task(999999) // I don't know why I have to do -1, but it works this way
    }
}

fn day_11_task(empty_distance: usize) -> usize {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut x = 0;
    for line in read.lines() {
        let mut has_galaxy = false;
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                has_galaxy = true;
            }
        }
        if !has_galaxy {
            x += empty_distance;
        }
        x += 1;
    }

    let max_galaxy_y = galaxies.iter().max_by(|(_, fy), (_, sy)| fy.cmp(sy)).unwrap().1;
    for iter in (0..max_galaxy_y).rev() {
        if galaxies.iter().find(|(_, y)| y == &iter).is_some() {
            continue;
        }

        for index in 0..galaxies.len() {
            let (x, y) = galaxies[index];
            if y < iter {
                continue;
            }

            galaxies[index] = (x, y + empty_distance);
        }
    }

    let retval: isize = galaxies.iter().enumerate()
        .map(
            |(first_index, (first_x, first_y))|
            {
                let distances: isize = galaxies.iter().enumerate()
                    .map(
                        |(second_index, (second_x, second_y))|
                        {
                            if first_index >= second_index {
                                return 0;
                            }

                            return ((*first_x as isize) - (*second_x as isize)).abs() + ((*first_y as isize) - (*second_y as isize)).abs();
                        }
                    ).sum();

                return distances;
            }
        ).sum();

    return retval as usize;
}