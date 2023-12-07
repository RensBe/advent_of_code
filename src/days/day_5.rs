use std::{fs, thread::{self, JoinHandle}};

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_5.txt";

pub fn day_5(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_5_task_1(),
        Task::TASK2 => day_5_task_2()
    }
}

struct MapItem {
    dest_range: usize,
    source_range: usize,
    range_length: usize
}

fn day_5_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = read.lines();

    let seeds: Vec<usize> = lines.next().unwrap()[7..].split(' ').into_iter().map(|seed_id| seed_id.parse::<usize>().unwrap()).collect();
    lines.next();

    let mut maps: Vec<Vec<MapItem>> = Vec::new();
    let mut cur_map: Vec<MapItem> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        else if !line.chars().next().unwrap().is_numeric() {
            maps.push(cur_map);
            cur_map = Vec::new();
            continue;
        }
        
        let mut cur_numbers = line.split(' ');

        cur_map.push(MapItem 
            { 
                dest_range: cur_numbers.next().unwrap().parse::<usize>().unwrap(), 
                source_range: cur_numbers.next().unwrap().parse::<usize>().unwrap(), 
                range_length: cur_numbers.next().unwrap().parse::<usize>().unwrap()
            }
        )
    }

    if !cur_map.is_empty() {
        maps.push(cur_map);
    }

    return seeds.iter().map(
        |seed|
        {
            let mut x: usize = *seed;
        
            for map in &maps {
                let map_item = map.iter()
                    .find(
                        |map_item| 
                        map_item.source_range <= x &&
                        map_item.source_range + map_item.range_length > x
                    );
                
                match map_item {
                    Some(map_item) => {x = map_item.dest_range + x - map_item.source_range},
                    None => ()
                }
            }

            return x;
        }
    ).min().unwrap() as u32;
}

fn day_5_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = read.lines();

    let raw_seeds: Vec<usize> = lines.next().unwrap()[7..].split(' ').into_iter().map(|seed_id| seed_id.parse::<usize>().unwrap()).collect();
    lines.next();

    let mut seeds: Vec<(usize, usize)> = Vec::new();
    let mut cur_seed = (0, 0);
    for (i, raw_seed) in raw_seeds.iter().enumerate() {
        if i % 2 == 0 {
            cur_seed.0 = *raw_seed;
        }
        else {
            cur_seed.1 = *raw_seed;
            seeds.push(cur_seed);
        }
    }

    let mut maps: Vec<Vec<MapItem>> = Vec::new();
    let mut cur_map: Vec<MapItem> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        else if !line.chars().next().unwrap().is_numeric() {
            maps.push(cur_map);
            cur_map = Vec::new();
            continue;
        }
        
        let mut cur_numbers = line.split(' ');

        cur_map.push(MapItem 
            { 
                dest_range: cur_numbers.next().unwrap().parse::<usize>().unwrap(), 
                source_range: cur_numbers.next().unwrap().parse::<usize>().unwrap(), 
                range_length: cur_numbers.next().unwrap().parse::<usize>().unwrap()
            }
        )
    }

    if !cur_map.is_empty() {
        maps.push(cur_map);
    }

    let mut threads: Vec<JoinHandle<u32>> = Vec::new();
    for seed in seeds {
        let cloned_maps = clone_maps(&maps);
        let t = thread::spawn(move || {
            let mut ext: Vec<usize> = Vec::new();

            for iter in 0..seed.1 {
                ext.push(seed.0 + iter);
            }

            return alpha(ext, &cloned_maps);
        });
        
        threads.push(t);
    }

    let mut minval = u32::MAX;
    for t in threads {
        let y = t.join().unwrap();
        if y < minval {
            minval = y;
        }
    }

    return minval;
}

fn alpha(seeds: Vec<usize>, maps: &Vec<Vec<MapItem>>) -> u32 {
    let retval = seeds.iter().map(
        |seed|
        {
            let mut x: usize = *seed;
        
            for map in maps {
                let map_item = map.iter()
                    .find(
                        |map_item| 
                        map_item.source_range <= x &&
                        map_item.source_range + map_item.range_length > x
                    );
                
                match map_item {
                    Some(map_item) => {x = map_item.dest_range + x - map_item.source_range},
                    None => ()
                }
            }

            return x;
        }
    ).min().unwrap() as u32;

    println!("found retval");
    return retval;
}

fn clone_maps(maps: &Vec<Vec<MapItem>>) -> Vec<Vec<MapItem>> {
    let mut new_maps: Vec<Vec<MapItem>> = Vec::new();

    for map in maps {
        let mut new_map: Vec<MapItem> = Vec::new();
        for map_item in map {
            new_map.push(MapItem { ..*map_item });
        }
        new_maps.push(new_map);
    }

    return new_maps;
}