use std::{fs, collections::HashMap, cmp::Ordering};

use crate::Task;

const FILE_PATH: &str = "src/inputs/input_day_7.txt";

pub fn day_7(task: Task) -> u32 {
    match task {
        Task::TASK1 => day_7_task_1(),
        Task::TASK2 => day_7_task_2()
    }
}

#[derive(Copy, Clone)]
enum Type {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7
}

fn day_7_task_1() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let lines = read.lines();

    let mut hands = lines.into_iter()
        .map(
            |line|
            {
                let mut parts = line.split(' ');

                let cards = parts.next().unwrap();
                let bid = parts.next().unwrap().parse::<usize>().unwrap();

                let hand = cards.chars()
                    .fold(
                        HashMap::new(), 
                        |mut map, c| 
                        {
                            *map.entry(c).or_insert(0) += 1;
                            map
                        }
                    );

                let mut has_two = false;
                let mut has_three = false;
                let mut hand_type: Option<Type> = None;
                for card in hand {
                    match card.1 {
                        1 => (),
                        2 => if has_two { hand_type = Some(Type::TwoPair); break; } else if has_three { hand_type = Some(Type::FullHouse); break; } else { has_two = true; },
                        3 => if has_two { hand_type = Some(Type::FullHouse); break; } else { has_three = true; },
                        4 => hand_type = Some(Type::FourOfKind),
                        5 => hand_type = Some(Type::FiveOfKind),
                        _ => ()
                    }
                }

                if hand_type.is_none() {
                    if has_two {
                        hand_type = Some(Type::OnePair);
                    }
                    else if has_three {
                        hand_type = Some(Type::ThreeOfKind);
                    }
                    else {
                        hand_type = Some(Type::HighCard);
                    }
                }

                return (cards, hand_type.unwrap(), bid);
            }
        )
        .collect::<Vec<(&str, Type, usize)>>();


    hands.sort_by(
        |x, y| 
        {
            let a = x.1 as u8;
            let b = y.1 as u8;
            let cur_order = a.cmp(&b);

            return match cur_order {
                Ordering::Equal => {
                    for iter in 0..5 {
                        let x_c = match x.0.chars().nth(iter).unwrap() {
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            '9' => 9,
                            'T' => 10,
                            'J' => 11,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => 0
                        };
                        let y_c = match y.0.chars().nth(iter).unwrap() {
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            '9' => 9,
                            'T' => 10,
                            'J' => 11,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => 0
                        };

                        if x_c != y_c {
                            return x_c.cmp(&y_c);
                        }
                    }

                    return Ordering::Greater;
                },
                _ => cur_order
            };
        }
    );

    return hands.into_iter().enumerate().into_iter()
        .map(
            |x|
            {
                return ((x.0 + 1) * x.1.2) as u32;
            }
        ).sum();
}

fn day_7_task_2() -> u32 {
    let read = fs::read_to_string(FILE_PATH).unwrap();

    let lines = read.lines();

    let mut hands = lines.into_iter()
        .map(
            |line|
            {
                let mut parts = line.split(' ');

                let cards = parts.next().unwrap();
                let bid = parts.next().unwrap().parse::<usize>().unwrap();

                let hand = cards.chars()
                    .fold(
                        HashMap::new(), 
                        |mut map, c| 
                        {
                            *map.entry(c).or_insert(0) += 1;
                            map
                        }
                    );
                
                let joker = hand.get_key_value(&'J');
                let mut highest: Option<(&char, &i32)> = None;
                if !joker.is_none() {
                    highest = hand.iter().filter(|card| *card.0 != 'J').max_by(|x, y| x.1.cmp(y.1));
                }

                let mut has_two = false;
                let mut has_three = false;
                let mut hand_type: Option<Type> = None;
                for card in &hand {
                    if card.0 == &'J' && card.1 != &5 {
                        continue;
                    }

                    let mut comp_val = *card.1;
                    if !highest.is_none() && card.0 == highest.unwrap().0 {
                        comp_val += joker.unwrap().1;
                    }

                    match comp_val {
                        1 => (),
                        2 => if has_two { hand_type = Some(Type::TwoPair); break; } else if has_three { hand_type = Some(Type::FullHouse); break; } else { has_two = true; },
                        3 => if has_two { hand_type = Some(Type::FullHouse); break; } else { has_three = true; },
                        4 => hand_type = Some(Type::FourOfKind),
                        5 => hand_type = Some(Type::FiveOfKind),
                        _ => ()
                    }
                }

                if hand_type.is_none() {
                    if has_two {
                        hand_type = Some(Type::OnePair);
                    }
                    else if has_three {
                        hand_type = Some(Type::ThreeOfKind);
                    }
                    else {
                        hand_type = Some(Type::HighCard);
                    }
                }

                return (cards, hand_type.unwrap(), bid);
            }
        )
        .collect::<Vec<(&str, Type, usize)>>();


    hands.sort_by(
        |x, y| 
        {
            let a = x.1 as u8;
            let b = y.1 as u8;
            let cur_order = a.cmp(&b);

            return match cur_order {
                Ordering::Equal => {
                    for iter in 0..5 {
                        let x_c = match x.0.chars().nth(iter).unwrap() {
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            '9' => 9,
                            'T' => 10,
                            'J' => 1,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => 0
                        };
                        let y_c = match y.0.chars().nth(iter).unwrap() {
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            '9' => 9,
                            'T' => 10,
                            'J' => 1,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => 0
                        };

                        if x_c != y_c {
                            return x_c.cmp(&y_c);
                        }
                    }

                    return Ordering::Greater;
                },
                _ => cur_order
            };
        }
    );

    return hands.into_iter().enumerate().into_iter()
        .map(
            |x|
            {
                return ((x.0 + 1) * x.1.2) as u32;
            }
        ).sum();
}