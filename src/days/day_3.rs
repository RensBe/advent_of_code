use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_3.txt";

pub fn day_3(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_3_task_1(),
        Task::TASK2 => day_3_task_2()
    }
}

struct Number {
    value: u32,
    x: isize,
    y_range: Vec<isize>
}

struct Symbol {
    x: isize,
    y: isize
}

fn day_3_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();
    
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    // Keep track of the x (Row)
    let mut x = 0;

    // Go through all the lines in the document
    for line in read.lines() {
        // These values are necesairy for building the numbers in the file
        let mut last_numeric = false;
        let mut numeric_value = String::from("");
        let mut y_range: Vec<isize> = Vec::new();

        // Keep track of the y (Column)
        let mut y = 0;

        // go through all the characters
        for char in line.chars() {
            if char.is_numeric() {
                // If the value is numeric add it to the numeric value
                numeric_value.push(char);
                y_range.push(y);
            }
            else if char != '.' {
                symbols.push(Symbol { x, y });
            }

            if !char.is_numeric() && last_numeric {
                numbers.push(
                    Number { 
                        value: numeric_value.parse::<u32>().unwrap(), 
                        x, 
                        y_range: y_range.clone() 
                    });
                numeric_value = String::from("");
                y_range = Vec::new();
            }

            last_numeric = char.is_numeric();

            y += 1;
        }
        
        if last_numeric {
            numbers.push(
                Number { 
                    value: numeric_value.parse::<u32>().unwrap(), 
                    x, 
                    y_range: y_range 
                });
        }

        x += 1;
    }

    // Go through all the numbers
    return numbers.into_iter()
        .map(
            |number|
            {
                // Check if the number is valid
                for symbol in &symbols {
                    if 
                    ( // Check x
                        symbol.x >= number.x - 1 &&
                        symbol.x <= number.x + 1
                    ) &&
                    ( // Check y
                        symbol.y >= number.y_range.iter().min().unwrap() - 1 &&
                        symbol.y <= number.y_range.iter().max().unwrap() + 1
                    ) 
                    {
                        // If it is valid return the value
                        return number.value;
                    }
                }
                
                // If it is not valid return 0
                return 0;
            }
        ).sum()
}

fn day_3_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();
    
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    // Keep track of the x (Row)
    let mut x = 0;

    // Go through all the lines in the document
    for line in read.lines() {
        // These values are necesairy for building the numbers in the file
        let mut last_numeric = false;
        let mut numeric_value = String::from("");
        let mut y_range: Vec<isize> = Vec::new();

        // Keep track of the y (Column)
        let mut y = 0;

        // go through all the characters
        for char in line.chars() {
            if char.is_numeric() {
                // If the value is numeric add it to the numeric value
                numeric_value.push(char);
                y_range.push(y);
            }
            else if char == '*' {
                symbols.push(Symbol { x, y });
            }

            if !char.is_numeric() && last_numeric {
                numbers.push(
                    Number { 
                        value: numeric_value.parse::<u32>().unwrap(), 
                        x, 
                        y_range: y_range.clone() 
                    });
                numeric_value = String::from("");
                y_range = Vec::new();
            }

            last_numeric = char.is_numeric();

            y += 1;
        }
        
        if last_numeric {
            numbers.push(
                Number { 
                    value: numeric_value.parse::<u32>().unwrap(), 
                    x, 
                    y_range: y_range
                });
        }

        x += 1;
    }

    // Go through all the numbers
    return symbols.into_iter()
        .map(
            |symbol|
            {
                let var: Vec<u32> = numbers.iter()
                    .filter(
                        |number|
                        ( // Check x
                            symbol.x >= number.x - 1 &&
                            symbol.x <= number.x + 1
                        ) &&
                        ( // Check y
                            symbol.y >= number.y_range.iter().min().unwrap() - 1 &&
                            symbol.y <= number.y_range.iter().max().unwrap() + 1
                        )
                    )
                    .map(
                        |number|
                        number.value
                    ).collect();

                if var.len() == 2 {
                    return var[0] * var[1];
                }
                return 0;
            }
        ).sum();
}