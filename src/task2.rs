use std::{collections::HashMap, fs};

pub fn task_2() {
    let content = fs::read_to_string("./src/input2").expect("to read file");
    let lines: Vec<_> = content.trim().split("\n").collect();

    // let mut sum_of_possible_game: i32 = 0;
    let mut sum_of_powers: i32 = 0;

    // 'line_loop: for line in lines {
    for line in lines {
        // let game_id = get_game_id(line);
        let vec_games = get_games_vector(line);
        let mut red: i32 = 0;
        let mut blue: i32 = 0;
        let mut green: i32 = 0;
        for game in vec_games {
            let map_of_colors = split_game_into_parts(&game);

            let mut cubes_in_g = HashMap::new();

            for (color, number) in map_of_colors {
                match color {
                    "blue" => match cubes_in_g.get("blue") {
                        Some(&val) => {
                            if number > val {
                                cubes_in_g.insert(color, number);
                            }
                        }
                        None => {
                            cubes_in_g.insert(color, number);
                        }
                    },
                    "green" => match cubes_in_g.get("green") {
                        Some(&val) => {
                            if number > val {
                                cubes_in_g.insert(color, number);
                            }
                        }
                        None => {
                            cubes_in_g.insert(color, number);
                        }
                    },
                    "red" => match cubes_in_g.get("red") {
                        Some(&val) => {
                            if number > val {
                                cubes_in_g.insert(color, number);
                            }
                        }
                        None => {
                            cubes_in_g.insert(color, number);
                        }
                    },
                    _ => println!("unexpected"),
                }
            }

            for (k, v) in cubes_in_g {
                if k == "green" {
                    if v > green {
                        green = v
                    }
                }
                if k == "blue" {
                    if v > blue {
                        blue = v
                    }
                }
                if k == "red" {
                    if v > red {
                        red = v;
                    }
                }
            }
        }
        sum_of_powers = sum_of_powers + (red * green * blue)
    }

    println!("{sum_of_powers}")
}

fn get_game_id(text: &str) -> i32 {
    let mut text_iterator = text.split(":");
    let game_with_id = text_iterator.next().unwrap();
    game_with_id
        .split_whitespace()
        .last()
        .expect("to have string number")
        .parse()
        .unwrap()
}

fn get_games_vector(text: &str) -> Vec<String> {
    let mut text_iterator = text.split(":");

    let _ = text_iterator.next().unwrap();
    let games = text_iterator.next().unwrap();

    let vec_games = games.split(";").map(str::to_string).collect();
    vec_games
}

fn split_game_into_parts(text: &str) -> HashMap<&str, i32> {
    let mut color_and_number = HashMap::new();
    let k_v_vec: Vec<_> = text.split(",").collect();

    for k_v in k_v_vec {
        let mut splitted = k_v.split_whitespace();

        let number: i32 = splitted.next().unwrap().trim().parse().unwrap();
        let color = splitted.next().unwrap();
        color_and_number.insert(color, number.clone());
    }

    color_and_number
}
