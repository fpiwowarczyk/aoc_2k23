use std::collections::HashMap;
use std::fs;

pub fn day1_2() {
    let content = fs::read_to_string("./src/input1").expect("to read file");

    let lines: Vec<_> = content.split("\n").collect();
    let mut sum: i32 = 0;
    for text in lines {
        if text.trim().len() == 0 {
            continue;
        }
        let result = find_number_and_merge2(text.trim());
        if result.len() == 0 {
            continue;
        }
        let number: i32 = result.trim().parse().expect("correct number");
        sum = sum + number
    }

    println!("{sum}")
}

// fn find_number_and_merge(text: &str) -> String {
//     let mut first_number: char = char::default();
//     let mut second_number: char = char::default();
//
//     for c in text.chars() {
//         if c.is_numeric() {
//             if first_number == char::default() {
//                 first_number = c
//             }
//             second_number = c
//         }
//     }
//
//     format!("{first_number}{second_number}")
// }

fn find_number_and_merge2(text: &str) -> String {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut first_number: char = char::default();
    let mut second_number: char = char::default();

    while start != text.len() {
        let s: String = text.chars().skip(start).take(end - start).collect();
        if s.chars().count() == 1 {
            for a in s.chars() {
                if a.is_numeric() {
                    if first_number == char::default() {
                        first_number = a
                    }
                    second_number = a
                }
            }
        } else {
            let mut digits = HashMap::new();
            digits.insert("one", '1');
            digits.insert("two", '2');
            digits.insert("three", '3');
            digits.insert("four", '4');
            digits.insert("five", '5');
            digits.insert("six", '6');
            digits.insert("seven", '7');
            digits.insert("eight", '8');
            digits.insert("nine", '9');
            for (k, v) in digits {
                if s.contains(k) {
                    if first_number == char::default() {
                        first_number = v
                    }
                    second_number = v
                }
            }
        }

        if end != text.len()
            && text.chars().nth(end).expect("to be possible").is_numeric()
            && end > start
        {
            start = start + 1
        } else if end == text.len() {
            start = start + 1
        } else {
            end = end + 1
        }
    }

    format!("{first_number}{second_number}")
}
