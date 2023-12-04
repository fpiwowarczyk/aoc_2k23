use std::collections::HashMap;
use std::fs;

pub fn day4_2() {
    let content = fs::read_to_string("./inputs/input4").expect("to read file");
    let cards: Vec<_> = content.trim().split("\n").collect();

    let mut total_sum: i32 = 0;

    let mut card_multiplier = HashMap::new();

    for (i, card) in cards.iter().enumerate() {
        card_multiplier.insert((i + 1).try_into().unwrap(), 1);
    }

    for card in cards {
        let (card_id_str, card_winning, card_numbers) = split_card_into_three(card);

        let card_id: i32 = card_id_str
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let winning_numbers: Vec<i32> = card_winning
            .split_whitespace()
            .map(|data| {
                let data_i: i32 = data.parse().unwrap();
                data_i
            })
            .collect();

        let card_hits = card_numbers
            .split_whitespace()
            .map(|data| {
                let data_i: i32 = data.parse().unwrap();
                if winning_numbers.contains(&data_i) {
                    return 1;
                }
                0
            })
            .reduce(|acc, e| if e == 1 { acc + 1 } else { acc })
            .expect("to finish");
        println!("Card {card_id}: hits {card_hits}");
        for n in 1..card_hits + 1 {
            match card_multiplier.get(&(card_id + n)) {
                // Set to prev item val plus 1 times current card multiplier
                Some(v) => card_multiplier.insert(
                    card_id + n,
                    v + 1 * card_multiplier.get(&card_id).expect("to exist some"),
                ),
                // Set item to 1 times current cart multiplier
                None => card_multiplier.insert(
                    card_id + n,
                    1 * card_multiplier.get(&card_id).expect("to exist none"),
                ),
            };
        }

        println!("{:?}", card_multiplier);
    }
    for (k, v) in card_multiplier {
        total_sum = total_sum + v
    }
    println!("{}", total_sum)
}

pub fn day4_1() {
    let content = fs::read_to_string("./inputs/input4").expect("to read file");
    let cards: Vec<_> = content.trim().split("\n").collect();

    let mut total_sum: i32 = 0;

    for card in cards {
        let (_, card_winning, card_numbers) = split_card_into_three(card);

        let winning_numbers: Vec<i32> = card_winning
            .split_whitespace()
            .map(|data| {
                let data_i: i32 = data.parse().unwrap();
                data_i
            })
            .collect();

        let card_sum = card_numbers
            .split_whitespace()
            .map(|data| {
                let data_i: i32 = data.parse().unwrap();
                if winning_numbers.contains(&data_i) {
                    return 1;
                }
                return 0;
            })
            .fold(0, |acc, el| {
                if acc == 0 && el == 1 {
                    1
                } else if el == 1 {
                    acc * 2
                } else {
                    acc
                }
            });
        println!("{:?}", card_sum);
        total_sum = total_sum + card_sum;
    }
    println!("{total_sum}")
}

fn split_card_into_three(card_line: &str) -> (String, String, String) {
    let mut splitted = card_line.split(":");
    let card_with_id = splitted.next().unwrap();
    let card_winning_and_game = splitted.next().unwrap();
    let mut splitted = card_winning_and_game.split("|");
    let card_winning_numbers = splitted.next().unwrap();
    let card_game_numbers = splitted.next().unwrap();

    return (
        String::from(card_with_id),
        String::from(card_winning_numbers),
        String::from(card_game_numbers),
    );
}
