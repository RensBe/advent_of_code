use std::fs;

const FILE_PATH: &str = "src/inputs/input_day_1.txt";

pub fn day_1(task: u8) -> u32 {
    // define the numbers that can be replaced
    let numbers: Vec<(&str, &str)>;

    match task {
        1 => numbers = vec![
            ("0", "0"), 
            ("1", "1"), 
            ("2", "2"), 
            ("3", "3"),
            ("4", "4"), 
            ("5", "5"), 
            ("6", "6"), 
            ("7", "7"), 
            ("8", "8"), 
            ("9", "9")
        ],
        _ => numbers = vec![
            ("0", "0"), 
            ("1", "1"), 
            ("2", "2"), 
            ("3", "3"),
            ("4", "4"), 
            ("5", "5"), 
            ("6", "6"), 
            ("7", "7"), 
            ("8", "8"), 
            ("9", "9"), 
            ("zero",  "0"), 
            ("one",   "1"), 
            ("two",   "2"), 
            ("three", "3"),
            ("four",  "4"), 
            ("five",  "5"), 
            ("six",   "6"), 
            ("seven", "7"), 
            ("eight", "8"), 
            ("nine",  "9")
        ]
    }

    day_1_task(numbers)
}

fn day_1_task(numbers: Vec<(&str, &str)>) -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let contents: Vec<u32> = read.lines().into_iter()
        .map(
            |line| 
            {
                // get the lowest text and the highest text in the line
                let mut low_number = ("", None);
                let mut high_number = ("", None);

                for number in &numbers {
                    let low = line.find(number.0);
                    let high = line.rfind(number.0);

                    if low_number.1.is_none() || (!low.is_none() && low.unwrap() < low_number.1.unwrap()) {
                        low_number = (number.1, low);
                    }
                    if high_number.1.is_none() || (!high.is_none() && high.unwrap() > high_number.1.unwrap()) {
                        high_number = (number.1, high);
                    }
                }

                let mut x: String = String::from("");
                x += low_number.0;
                x += high_number.0;

                return x.parse::<u32>().unwrap();
            }
        ).collect();

    return contents.iter().sum();
}