use std::{fs, num};

#[derive(Debug)]
struct Element {
    x: i32,
    y: i32,
    val: String,
}

impl Element {
    pub fn new(x: i32, y: i32, val: String) -> Self {
        Element { x, y, val }
    }
}

// 82621207
// 82818007
pub fn day3_2() {
    let content = fs::read_to_string("./src/input3").expect("to read file");
    let lines: Vec<_> = content.trim().split("\n").collect();

    let mut elemets: Vec<Element> = Vec::new();
    let mut total_sum: i64 = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            elemets.push(Element::new(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                String::from(ch),
            ))
        }
    }

    for (y, line) in lines.iter().enumerate() {
        let line_len: usize = line.len();
        for (x, ch) in line.chars().enumerate() {
            if ch == char::from('*') {
                let nums = find_close_numbers(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    line_len,
                    &elemets,
                );
                println!("{:?}", nums);
                if nums.len() == 2 {
                    let mut multiply: i64 = 1;
                    for num in nums {
                        let num_i64: i64 = num.try_into().unwrap();
                        multiply = multiply * num_i64;
                    }
                    println!("{multiply}");
                    total_sum = total_sum + multiply
                }
            }
        }
    }

    println!("{total_sum}")
}

fn find_close_numbers(x: i32, y: i32, line_len: usize, elements: &Vec<Element>) -> Vec<i32> {
    let mut numbers = Vec::new();
    // x-1 y
    let mut num_str: String = loopkup_adj_number(x - 1, y, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x - 2;
        while loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x;
        while loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }

        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    // x-1 y-1
    let mut num_str: String =
        loopkup_adj_number(x - 1, y - 1, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x - 2;
        while loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements),
                num_str
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x;
        while loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            );
            tn_x = tn_x + 1;
        }

        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    // x y-1
    let mut num_str: String = loopkup_adj_number(x, y - 1, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x - 1;
        while loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x + 1;
        while loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }
        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    // x+1 y-1
    let mut num_str: String =
        loopkup_adj_number(x + 1, y - 1, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x;
        while loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x + 2;
        while loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y - 1, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }
        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    // x+1 y
    let mut num_str: String = loopkup_adj_number(x + 1, y, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x;
        while loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x + 2;
        while loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }
        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    //
    // x+1 y+1
    let mut num_str: String =
        loopkup_adj_number(x + 1, y + 1, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x;
        while loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x + 2;
        while loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }
        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    // x y+1
    let mut num_str: String = loopkup_adj_number(x, y + 1, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x - 1;
        while loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x + 1;
        while loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }
        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    // x-1 y+1
    let mut num_str: String =
        loopkup_adj_number(x - 1, y + 1, line_len.try_into().unwrap(), elements);
    if num_str != "a" {
        let mut tn_x: i32 = x - 2;
        while loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements),
                num_str,
            );
            tn_x = tn_x - 1;
        }

        let mut tn_x: i32 = x;
        while loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements)
            .chars()
            .next()
            .expect("abc")
            .is_numeric()
        {
            num_str = format!(
                "{}{}",
                num_str,
                loopkup_adj_number(tn_x, y + 1, line_len.try_into().unwrap(), elements),
            );
            tn_x = tn_x + 1;
        }
        let new_el: i32 = num_str.parse().unwrap();
        if !numbers.contains(&new_el) {
            numbers.push(new_el);
        }
    }
    return numbers;
}

fn loopkup_adj_number(s_x: i32, s_y: i32, line_len: i32, elements: &Vec<Element>) -> String {
    let n_x: i32 = s_x;
    let n_y: i32 = s_y;
    if s_x < 0 {
        return String::from("a");
    }
    if s_y < 0 {
        return String::from("a");
    }
    if s_x > line_len {
        return String::from("a");
    }

    for e in elements {
        if e.x == n_x && e.y == n_y && e.val.chars().next().expect("to exist").is_numeric() {
            return format!("{}", String::from(e.val.chars().next().expect("to exist")));
        }
    }
    return String::from("a");
}

// 538237 too low
// 538237
// pub fn day3_1() {
//     let content = fs::read_to_string("./src/input3").expect("to read file");
//     let lines: Vec<_> = content.trim().split("\n").collect();
//
//     let mut elemets: Vec<Element> = Vec::new();
//     let mut total_sum: i64 = 0;
//
//     for (y, line) in lines.iter().enumerate() {
//         for (x, ch) in line.chars().enumerate() {
//             elemets.push(Element::new(
//                 x.try_into().unwrap(),
//                 y.try_into().unwrap(),
//                 String::from(ch),
//             ))
//         }
//     }
//
//     for (y, line) in lines.iter().enumerate() {
//         let line_len: usize = line.len();
//
//         let mut temp_number_finder: String = String::from("");
//         let mut started_number: bool = false;
//         for (x, ch) in line.chars().enumerate() {
//             if ch.is_numeric() && !started_number {
//                 started_number = true;
//                 temp_number_finder = format!("{}{}", temp_number_finder, String::from(ch))
//             } else if started_number && ch.is_numeric() {
//                 temp_number_finder = format!("{}{}", temp_number_finder, String::from(ch));
//                 if x == line.len() - 1 {
//                     if is_engine_part(
//                         temp_number_finder.clone(),
//                         x,
//                         y.try_into().unwrap(),
//                         &elemets,
//                         line_len,
//                     ) {
//                         let engine_part_number: i64 = temp_number_finder.parse().unwrap();
//                         total_sum = total_sum + engine_part_number;
//                     }
//                 }
//             } else if started_number && (!ch.is_numeric() || (x == line.len() - 1)) {
//                 started_number = false;
//                 println!("{}", temp_number_finder);
//                 if is_engine_part(
//                     temp_number_finder.clone(),
//                     x,
//                     y.try_into().unwrap(),
//                     &elemets,
//                     line_len,
//                 ) {
//                     let engine_part_number: i64 = temp_number_finder.parse().unwrap();
//                     total_sum = total_sum + engine_part_number;
//                 }
//
//                 temp_number_finder = String::from("");
//             }
//         }
//     }
//     println!("{total_sum}")
// }

// fn is_engine_part(
//     number: String,
//     x: usize,
//     y: i32,
//     elements: &Vec<Element>,
//     line_len: usize,
// ) -> bool {
//     // loop from beggining of number till the end
//     for n_x in (x - number.len())..x {
//         let new_x: i32 = n_x.try_into().unwrap();
//
//         // x-1 y
//         if loopkup_adj_character(
//             (new_x - 1).try_into().unwrap(),
//             y.try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x-1 y-1
//         if loopkup_adj_character(
//             (new_x - 1).try_into().unwrap(),
//             (y - 1).try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x y-1
//         if loopkup_adj_character(
//             new_x.try_into().unwrap(),
//             (y - 1).try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x+1 y-1
//         if loopkup_adj_character(
//             (new_x + 1).try_into().unwrap(),
//             (y - 1).try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x+1 y
//         if loopkup_adj_character(
//             (new_x + 1).try_into().unwrap(),
//             y.try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x+1 y+1
//         if loopkup_adj_character(
//             (new_x + 1).try_into().unwrap(),
//             (y + 1).try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x y+1
//         if loopkup_adj_character(
//             new_x.try_into().unwrap(),
//             (y + 1).try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//         // x-1 y+1
//         if loopkup_adj_character(
//             (new_x - 1).try_into().unwrap(),
//             (y + 1).try_into().unwrap(),
//             line_len.try_into().unwrap(),
//             elements,
//         ) {
//             return true;
//         }
//     }
//
//     // println!("NO adjust / {} */", number);
//
//     false
// }
