use std::collections::HashMap;
use std::fs;

// pub fn day6_1() {
//     let mut time_to_distance = HashMap::new();
//     let content = fs::read_to_string("./inputs/input6").expect("to read file");
//     let mut lines = content.trim().split("\n");
//     let times: Vec<u128> = lines
//         .next()
//         .expect("to have time values")
//         .split_whitespace()
//         .filter(|el| !(el == &"Time:"))
//         .map(|el| el.parse().expect("to have number"))
//         .collect();
//     let distances: Vec<u128> = lines
//         .next()
//         .expect("to have time values")
//         .split_whitespace()
//         .filter(|el| !(el == &"Distance:"))
//         .map(|el| el.parse().expect("to have number"))
//         .collect();
//
//     for (i, _) in times.iter().enumerate() {
//         time_to_distance.insert(
//             times.get(i).expect("to have it"),
//             distances.get(i).expect("to have it"),
//         );
//     }
//
//     let mut wins_multiply: u128 = 1;
//
//     for (time, distance) in time_to_distance {
//         // let wins = count_winning_values(time, distance);
//         println!("{time} {distance}:{wins}");
//         wins_multiply = wins_multiply * wins;
//     }
//     println!("{wins_multiply}")
// }

pub fn day6_2() {
    let content = fs::read_to_string("./inputs/input6").expect("to read file");
    let mut lines = content.trim().split("\n");

    let times_str: Vec<&str> = lines
        .next()
        .expect("to have")
        .trim()
        .split(":")
        .filter(|el| !(el == &"Time"))
        .collect();
    let distance_str: Vec<&str> = lines
        .next()
        .expect("to have")
        .trim()
        .split(":")
        .filter(|el| !(el == &"Distance"))
        .collect();

    let times: Vec<u128> = times_str
        .get(0)
        .expect("to have val")
        .replace(" ", "")
        .split_whitespace()
        .map(|el| el.parse().expect("to have number"))
        .collect();
    let distances: Vec<u128> = distance_str
        .get(0)
        .expect("to have val")
        .replace(" ", "")
        .split_whitespace()
        .map(|el| el.parse().expect("to have number"))
        .collect();

    println!("{:?}{:?}", times, distances);
    let time: u128 = *times.get(0).expect("to exist");
    let distance: u128 = *distances.get(0).expect("to exist");

    let wins = count_winning_values(time, distance);
    println!("{wins}")
}

fn count_winning_values(time: u128, distance: u128) -> u128 {
    println!("{time} {distance}");
    let mut wins: u128 = 0;
    for time_press in 0..=time {
        let distance_done = calculate_distance(time_press, time - time_press);
        if distance_done > distance {
            wins = wins + 1;
        }
    }

    return wins;
}

fn calculate_distance(speed: u128, time_left: u128) -> u128 {
    speed * time_left
}
