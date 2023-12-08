use std::{fs, collections::HashMap};

use num_integer::lcm;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_8.txt";

pub fn day_8(task: Task) -> u64 {
    match task {
        Task::TASK1 => day_8_task_1(),
        Task::TASK2 => day_8_task_2()
    }
}

fn day_8_task_1() -> u64 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = read.lines();

    let directions = lines.next().unwrap();
    lines.next();

    let nodes = lines.into_iter()
        .fold(
            HashMap::new(),
            |mut map, line|
            {
                let mut values = line.split(" = (");
                let point = values.next().unwrap();
                let options = values.next().unwrap();
                let options: &Vec<&str> = &options[..options.len()-1].split(", ").collect();

                map.insert(point, (options[0], options[1]));

                return map;
            }
        );
    
    let mut current_node = nodes.get_key_value("AAA").unwrap();

    let mut iter = 1;
    loop {
        for c in directions.chars() {
            let r = match c {
                'L' => current_node.1.0,
                'R' => current_node.1.1,
                _ => ""
            };

            current_node = nodes.get_key_value(r).unwrap();

            if *current_node.0 == "ZZZ" {
                return iter as u64;
            }
            iter += 1;
        }
    }
}

fn day_8_task_2() -> u64 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut lines = read.lines();

    let directions = lines.next().unwrap();
    lines.next();

    let nodes = lines.into_iter()
        .fold(
            HashMap::new(),
            |mut map, line|
            {
                let mut values = line.split(" = (");
                let point = values.next().unwrap();
                let options = values.next().unwrap();
                let options: &Vec<&str> = &options[..options.len()-1].split(", ").collect();

                map.insert(point, (options[0], options[1]));

                return map;
            }
        );

    let start_nodes: Vec<(&&str, &(&str, &str))> = nodes.iter()
        .filter(
            |(node, _)| 
            // returns all nodes that end with an A
            node.chars().next_back().unwrap() == 'A'
        )
        .collect();

    let node_endpoints: Vec<HashMap<((&&str, &(&str, &str)), usize), u64>> = start_nodes.iter()
        .map(|x| *x)
        .map(
            |mut current_node| 
            {
                let mut map = HashMap::new();
                let mut iter: u64 = 1;
                'node_loop: loop {
                    for (i, c) in directions.chars().enumerate() {
                        let r = match c {
                            'L' => current_node.1.0,
                            'R' => current_node.1.1,
                            _ => ""
                        };
        
                        current_node = nodes.get_key_value(r).unwrap();

                        if !map.get(&(current_node, i)).is_none() {
                            break 'node_loop;
                        }
        
                        if current_node.0.chars().next_back().unwrap() == 'Z' {
                            map.insert((current_node, i), iter.clone());
                        }
                        iter += 1;
                    }
                }
                return map;
            }
        )
        .collect();

    // The code above takes into account that a start node can have more than one end node.
    // However in the data this appeared nowhere.
    // The function below works if an startpoint only has one endpoint. Further implementation would take more time.
    return node_endpoints.iter().map(|x| *x.values().next().unwrap()).fold(1, lcm);
}