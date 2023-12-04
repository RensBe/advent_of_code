use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_4.txt";

// I tried speedrunning today, managed to finish in 1 hour
// Code looks horible though

// Update: Optimized both parts
pub fn day_4(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_4_task_1_v2(),
        Task::TASK2 => day_4_task_2_v2()
    }
}

fn _day_4_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut count = 0;
    for line in read.lines() {
        let mut game = line.split(": ");
        let _game_number = game.next().unwrap();
        
        let mut sets = game.next().unwrap().split(" | ");
        let wins = sets.next().unwrap().split(' ');

        let mut x: Vec<usize> = Vec::new();
        for win in wins {
            if win == "" {
                continue;
            }
            x.push(win.parse::<usize>().unwrap())
        }

        let checks = sets.next().unwrap().split(' ');

        let mut y: Vec<usize> = Vec::new();
        for check in checks {
            if check == "" {
                continue;
            }
            y.push(check.parse::<usize>().unwrap());
        }

        let mut result = 1;
        for next in x {
            if y.iter().any(|other| next == other.clone()) {
                result *= 2;
            }
        }

        if result != 1 {
            result /= 2;
        }
        else {
            result = 0;
        }

        count += result;
    }

    return count;
}

// about 1.1× faster
fn day_4_task_1_v2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    read.lines().into_iter()
        .map(
            |line|
            {
                let mut sets = line[line.find(':').unwrap()+1..].split(" | ");
                let winners = sets.next().unwrap().split(' ');
                let valids = sets.next().unwrap().split(' ');

                let valids: Vec<u32> = valids.into_iter()
                    .filter(|check| !check.is_empty())
                    .map(|check| check.parse::<u32>().unwrap())
                    .collect();

                let win_count: u32 = winners.into_iter()
                    .filter(|win| !win.is_empty())
                    .map(
                        |win| 
                        {
                            let win = win.parse::<u32>().unwrap();
                            if valids.iter().any(|valid| valid == &win) {return 1} return 0;
                        })
                    .sum();
                
                if win_count == 0 {
                    return 0;
                }

                return u32::pow(2, win_count - 1);
            }
        )
        .sum()
}

struct _Card {
    number: usize,
    matches: usize
}

fn _day_4_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut cards: Vec<_Card> = Vec::new();
    for line in read.lines() {
        
        let mut game = line.split(": ");
        let card_number = game.next().unwrap()[4..].to_string().trim().parse::<usize>().unwrap();
        
        let mut sets = game.next().unwrap().split(" | ");
        let wins = sets.next().unwrap().split(' ');

        let mut x: Vec<usize> = Vec::new();
        for win in wins {
            if win == "" {
                continue;
            }
            x.push(win.parse::<usize>().unwrap())
        }

        let checks = sets.next().unwrap().split(' ');

        let mut y: Vec<usize> = Vec::new();
        for check in checks {
            if check == "" {
                continue;
            }
            y.push(check.parse::<usize>().unwrap());
        }

        let mut result = 0;
        for next in x {
            if y.iter().any(|other| next == other.clone()) {
                result += 1;
            }
        }

        cards.push(_Card {number: card_number, matches: result});
    }

    let mut mito = 0;
    for card in &cards {
        mito += _repeat(card, &cards)
    }
    mito += cards.len() as u32;

    return mito;
}

fn _repeat(card: &_Card, cards: &Vec<_Card>) -> u32 {
    let mut card_count = card.matches as u32;
    if card.matches == 0 {
        return card_count;
    }

    for iter in 1..card.matches + 1 {
        let fort = cards.iter().find(|crd| crd.number == card.number + iter);
        match fort {
            Some(elem) => card_count += _repeat(elem, cards),
            None => {}
        }
    }

    return card_count;
}

// about 3500× faster
fn day_4_task_2_v2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut multipliers: Vec<u32> = vec![1; read.lines().count()];

    read.lines().into_iter()
        .map(
            |line|
            {
                let mut sets = line[line.find(':').unwrap()+1..].split(" | ");
                let winners = sets.next().unwrap().split(' ');
                let valids = sets.next().unwrap().split(' ');

                let valids: Vec<u32> = valids.into_iter()
                    .filter(|check| !check.is_empty())
                    .map(|check| check.parse::<u32>().unwrap())
                    .collect();

                let win_count: usize = winners.into_iter()
                    .filter(|win| !win.is_empty())
                    .map(
                        |win| 
                        {
                            let win = win.parse::<u32>().unwrap();
                            if valids.iter().any(|valid| valid == &win) {return 1} return 0;
                        })
                    .sum();

                let multiplier = multipliers.pop().unwrap();
                let multiplier_len = multipliers.len();

                if win_count != 0 {
                    for iter in 1..win_count+1 {
                        multipliers[multiplier_len - iter] += multiplier;
                    }
                }

                return multiplier;
            }
        )
        .sum()
}