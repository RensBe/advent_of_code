use std::fs;

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_4.txt";

// I tried speedrunning today, managed to finish in 1 hour
// Code looks horible though
pub fn day_4(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_4_task_1(),
        Task::TASK2 => day_4_task_2()
    }
}

fn day_4_task_1() -> u32 {
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

struct Card {
    number: usize,
    matches: usize
}

fn day_4_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let mut cards: Vec<Card> = Vec::new();
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

        cards.push(Card {number: card_number, matches: result});
    }

    let mut mito = 0;
    for card in &cards {
        mito += repeat(card, &cards)
    }
    mito += cards.len() as u32;

    return mito;
}

fn repeat(card: &Card, cards: &Vec<Card>) -> u32 {
    let mut card_count = card.matches as u32;
    if card.matches == 0 {
        return card_count;
    }

    for iter in 1..card.matches + 1 {
        let fort = cards.iter().find(|crd| crd.number == card.number + iter);
        match fort {
            Some(elem) => card_count += repeat(elem, cards),
            None => {}
        }
    }

    return card_count;
}
