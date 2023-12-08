use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::hash::Hasher;

#[derive(Debug)]
struct Hand {
    value: String,
    bid: i64,
    hand_type: String,
    hand_power: i64,
}

pub fn day7_1() {
    // 7_2 242855559
    let cards_powers: HashMap<char, i64> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);
    let content = fs::read_to_string("./inputs/input7").expect("to read file");
    let lines = content.trim().split("\n");
    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        let mut iter = line.split_whitespace();
        let hand = iter.next().expect("to have hand");
        let bid = iter.next().expect("to have bid");

        let (typ, power) = calculate_figure(&String::from(hand));
        hands.push(Hand {
            value: String::from(hand),
            bid: bid.parse().expect("to be able to"),
            hand_type: typ,
            hand_power: power,
        });
    }

    hands.sort_by(|a, b| {
        if a.hand_power > b.hand_power {
            return Ordering::Greater;
        } else if a.hand_power == b.hand_power {
            for (i, _) in a.value.chars().enumerate() {
                let a_pow = cards_powers
                    .get(&a.value.chars().nth(i).expect("to have"))
                    .expect("to exist");
                let b_pow = cards_powers
                    .get(&b.value.chars().nth(i).expect("to have"))
                    .expect("to exist");
                if a_pow == b_pow {
                    continue;
                } else if a_pow > b_pow {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }
        Ordering::Less
    });

    println!("{:?}", hands);
    let mut total_sum = 0;

    for (i, h) in hands.iter().enumerate() {
        let i_as_i64: i64 = i.try_into().unwrap();
        total_sum += h.bid * (i_as_i64 + 1);
    }
    println!("{total_sum}");
}

fn calculate_figure(hand: &String) -> (String, i64) {
    match () {
        _ if is_five(hand) => (String::from("five"), 6),
        _ if is_four(hand) => (String::from("four"), 5),
        _ if is_full(hand) => (String::from("full"), 4),
        _ if is_three(hand) => (String::from("three"), 3),
        _ if is_two_pairs(hand) => (String::from("two_pairs"), 2),
        _ if is_pair(hand) => (String::from("pair"), 1),
        _ if is_high(hand) => (String::from("high"), 0),
        _ => panic!("UNEXPECTED hand! {hand}"),
    }
}

#[derive(Debug)]
struct Card {
    value: char,
    amount: i64,
}

fn is_five(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    let j_number = contains_joker(hand);
    for card in cards_counted {
        if card.amount == 5 || card.amount + j_number == 5 {
            return true;
        }
    }
    return false;
}

fn is_four(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    let j_number = contains_joker(hand);

    let mut cards_vec: Vec<i64> = vec![];
    for card in cards_counted {
        cards_vec.push(card.amount)
    }

    if (cards_vec.len() == 2 && j_number == 0 && cards_vec.contains(&4))
        || (cards_vec.len() == 3 && j_number == 2)
        || (cards_vec.contains(&3) && j_number == 1)
        || (cards_vec.len() == 3 && j_number == 3)
    {
        return true;
    }

    return false;
}

fn is_full(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    let mut cards_vec: Vec<i64> = vec![];
    for card in cards_counted {
        cards_vec.push(card.amount)
    }

    let j_number = contains_joker(hand);
    if (cards_vec.contains(&2) && cards_vec.contains(&3) && j_number == 0)
        || (cards_vec.len() == 3 && j_number == 1 && cards_vec.contains(&2))
    {
        return true;
    }

    return false;
}

fn is_three(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    let mut cards_vec: Vec<i64> = vec![];

    let j_number = contains_joker(hand);
    for card in cards_counted {
        cards_vec.push(card.amount)
    }
    if (cards_vec.contains(&3) && cards_vec.contains(&1) && j_number == 0)
        || (j_number == 2 && cards_vec.len() == 4)
        || (cards_vec.len() == 4 && j_number == 1)
    {
        return true;
    }
    return false;
}

fn is_two_pairs(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    let mut cards_vec: Vec<i64> = vec![];
    let j_number = contains_joker(hand);
    for card in cards_counted {
        cards_vec.push(card.amount)
    }
    if cards_vec.contains(&2) && cards_vec.contains(&1) && cards_vec.len() == 3 && j_number == 0 {
        return true;
    }
    return false;
}

fn is_pair(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    let mut cards_vec: Vec<i64> = vec![];

    let j_number = contains_joker(hand);
    for card in cards_counted {
        cards_vec.push(card.amount)
    }
    if (cards_vec.contains(&2) && cards_vec.contains(&1) && cards_vec.len() == 4 && j_number == 0)
        || (cards_vec.len() == 5 && j_number == 1)
    {
        return true;
    }

    return false;
}

fn is_high(hand: &String) -> bool {
    let cards_counted = count_cards(hand);
    for card in cards_counted {
        if card.amount != 1 || card.value == 'J' {
            return false;
        }
    }
    return true;
}

fn count_cards(hand: &String) -> Vec<Card> {
    let mut output: Vec<Card> = vec![];
    'cards: for card in hand.chars() {
        for (i, o) in output.iter().enumerate() {
            if o.value == card {
                output[i].amount += 1;
                continue 'cards;
            }
        }
        output.push(Card {
            value: card,
            amount: 1,
        });
    }
    output
}

fn contains_joker(hand: &String) -> i64 {
    let cards_counted = count_cards(hand);
    for card in cards_counted {
        if card.value == 'J' {
            return card.amount;
        }
    }
    return 0;
}
