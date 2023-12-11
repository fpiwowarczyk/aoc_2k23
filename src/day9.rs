use std::{fs, u128};

pub fn day9_1() {
    // 1641934227 low
    let content = fs::read_to_string("./inputs/input9").expect("to read file");
    let lines = content.trim().split("\n");
    let mut total_sum: i128 = 0;
    for line in lines {
        let mut history: Vec<i64> = line
            .split_whitespace()
            .map(|el| el.parse::<i64>().expect("To have i64"))
            .collect();
        history.reverse();
        let mut history_derivs: Vec<Vec<i64>> = vec![history.clone()];
        let mut to_derive = history.clone();
        let mut need_to_run = true;
        while need_to_run {
            let mut derive: Vec<i64> = vec![];
            for i in 0..to_derive.len() - 1 {
                derive.push(to_derive[i + 1] - to_derive[i]);
            }

            history_derivs.push(derive.clone());

            if all_zeros(&derive) {
                need_to_run = false;
            }

            to_derive = derive;
        }

        println!("{:?}", history_derivs);

        let mut last_el: i64 = 0;
        for hist in history_derivs {
            last_el += hist[hist.len() - 1];
        }
        history.push(last_el);
        let last_el_as_u: i128 = last_el.try_into().expect("to work");
        total_sum += last_el_as_u;
        println!("{:?}", history)
    }

    println!("{total_sum}")
}

fn all_zeros(vector: &Vec<i64>) -> bool {
    for v in vector {
        if *v != 0 {
            return false;
        }
    }
    return true;
}
