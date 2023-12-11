use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_10.txt";

pub fn day_10(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_10_task_1(),
        Task::TASK2 => day_10_task_2()
    }
}

struct Pipe {
    direction: char,
    x: usize,
    y: usize
}

fn day_10_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut animal = Pipe {direction: 'S', x: 1, y: 1};
    let pipes = read.lines().enumerate()
        .fold(
            Vec::new(),
            |mut pipes, (x, line)|
            {
                for (y, direction) in line.chars().enumerate() {
                    if direction == '.' {
                        continue;
                    }
                    if direction == 'S' {
                        animal = Pipe {direction, x, y};
                    }
        
                    pipes.push(Pipe { direction, x, y });
                }
                return pipes;
            }
        );

    let mut dir = 'x'; // NESW
    let mut current_pipe = pipes.iter()
        .find(
            |pipe| 
            {
                if pipe.y == animal.y {
                    if pipe.x == animal.x - 1 {
                        return match pipe.direction {
                            'F' => {dir = 'E'; true},
                            '7' => {dir = 'W'; true},
                            '|' => {dir = 'N'; true},
                            _ => false
                        }
                    }
                    else if pipe.x == animal.x + 1 {
                        return match pipe.direction {
                            'J' => {dir = 'W'; true},
                            'L' => {dir = 'E'; true},
                            '|' => {dir = 'S'; true},
                            _ => false
                        }
                    }
                }
                else if pipe.x == animal.x {
                    if pipe.y == animal.y - 1 {
                        return match pipe.direction {
                            'F' => {dir = 'S'; true},
                            'L' => {dir = 'N'; true},
                            '-' => {dir = 'W'; true},
                            _ => false
                        }
                    }
                    else if pipe.y == animal.y + 1 {
                        return match pipe.direction {
                            'J' => {dir = 'N'; true},
                            '7' => {dir = 'S'; true},
                            '-' => {dir = 'E'; true},
                            _ => false
                        }
                    }
                }

                return false;
            }
        ).unwrap();
    
    let mut iter = 1;
    loop {
        if current_pipe.direction == 'S' {
            break;
        }
        match dir {
            'N' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x - 1 && pipe.y == current_pipe.y).unwrap();
                match current_pipe.direction {
                    '7' => dir = 'W',
                    'F' => dir = 'E',
                    _ => ()
                }
            },
            'E' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x && pipe.y == current_pipe.y + 1).unwrap();
                match current_pipe.direction {
                    '7' => dir = 'S',
                    'J' => dir = 'N',
                    _ => ()
                }
            },
            'S' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x + 1 && pipe.y == current_pipe.y).unwrap();
                match current_pipe.direction {
                    'J' => dir = 'W',
                    'L' => dir = 'E',
                    _ => ()
                }
            },
            'W' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x && pipe.y == current_pipe.y - 1).unwrap();
                match current_pipe.direction {
                    'L' => dir = 'N',
                    'F' => dir = 'S',
                    _ => ()
                }
            },
            _ => {println!("UNEXPECTED DIRECTION\nEXPECTED: [N, E, S, W]\nRECEIVED: {}\nProcess stopped; outcome is INVALID", dir); break;}
        }

        iter += 1;
    }

    return iter / 2;
}

fn day_10_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut animal = Pipe {direction: 'S', x: 1, y: 1};
    let pipes = read.lines().enumerate()
        .fold(
            Vec::new(),
            |mut pipes, (x, line)|
            {
                for (y, direction) in line.chars().enumerate() {
                    if direction == 'S' {
                        animal = Pipe {direction, x, y};
                    }
        
                    pipes.push(Pipe { direction, x, y });
                }
                return pipes;
            }
        );

    let mut dir = ' '; // NESW
    
    let mut s_top = false;
    let mut s_left = false;
    let mut current_pipe = pipes.iter()
        .find(
            |pipe| 
            {
                if pipe.y == animal.y {
                    if pipe.x == animal.x - 1 {
                        
                        return match pipe.direction {
                            'F' => {s_top = true; dir = 'E'; true},
                            '7' => {s_top = true; dir = 'W'; true},
                            '|' => {s_top = true; dir = 'N'; true},
                            _ => false
                        }
                    }
                    else if pipe.x == animal.x + 1 {
                        return match pipe.direction {
                            'J' => {dir = 'W'; true},
                            'L' => {dir = 'E'; true},
                            '|' => {dir = 'S'; true},
                            _ => false
                        }
                    }
                }
                else if pipe.x == animal.x {
                    if pipe.y == animal.y - 1 {
                        return match pipe.direction {
                            'F' => {s_left = true; dir = 'S'; true},
                            'L' => {s_left = true; dir = 'N'; true},
                            '-' => {s_left = true; dir = 'W'; true},
                            _ => false
                        }
                    }
                    else if pipe.y == animal.y + 1 {
                        return match pipe.direction {
                            'J' => {dir = 'N'; true},
                            '7' => {dir = 'S'; true},
                            '-' => {dir = 'E'; true},
                            _ => false
                        }
                    }
                }

                return false;
            }
        ).unwrap();

    let s_val = match dir {
        'N' => if s_left {'L'} else {'J'},
        'E' => if s_top {'F'} else {'L'},
        'S' => if s_left {'F'} else {'7'},
        'W' => if s_top {'7'} else {'J'},
        _ => ' '
    };

    let mut sequence: Vec<&Pipe> = Vec::new();
    loop {
        sequence.push(current_pipe);
        if current_pipe.direction == 'S' {
            break;
        }
        match dir {
            'N' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x - 1 && pipe.y == current_pipe.y).unwrap();
                match current_pipe.direction {
                    '7' => dir = 'W',
                    'F' => dir = 'E',
                    _ => ()
                }
            },
            'E' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x && pipe.y == current_pipe.y + 1).unwrap();
                match current_pipe.direction {
                    '7' => dir = 'S',
                    'J' => dir = 'N',
                    _ => ()
                }
            },
            'S' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x + 1 && pipe.y == current_pipe.y).unwrap();
                match current_pipe.direction {
                    'J' => dir = 'W',
                    'L' => dir = 'E',
                    _ => ()
                }
            },
            'W' => {
                current_pipe = pipes.iter().find(|pipe| pipe.x == current_pipe.x && pipe.y == current_pipe.y - 1).unwrap();
                match current_pipe.direction {
                    'L' => dir = 'N',
                    'F' => dir = 'S',
                    _ => ()
                }
            },
            _ => {println!("UNEXPECTED DIRECTION\nEXPECTED: [N, E, S, W]\nRECEIVED: {}\nProcess stopped; outcome is INVALID", dir); break;}
        }
    }

    let mut retval = 0;
    let size = pipes.iter().max_by(|x, y| {x.y.cmp(&y.y)}).unwrap().y + 1;
    let mut columns = vec![false; size];
    let mut directions = vec![' '; size];
    for pipe in &pipes {
        let col = columns[pipe.y];

        if sequence.iter().find(|s_pipe| s_pipe.x == pipe.x && s_pipe.y == pipe.y).is_none() {
            if col {
                retval += 1;
            }
        }
        else {
            let direction = if pipe.direction == 'S' {s_val} else {pipe.direction};
            match direction {
                'F' => directions[pipe.y] = 'E',
                '7' => directions[pipe.y] = 'W',
                'L' => if directions[pipe.y] == 'W' {columns[pipe.y] = !col},
                'J' => if directions[pipe.y] == 'E' {columns[pipe.y] = !col},
                '-' => columns[pipe.y] = !col,
                _ => ()
            }
        }
    }

    return retval;
}