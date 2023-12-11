use std::collections::HashMap;
use std::sync::mpsc::{self, channel};
use std::time::Duration;
use std::{fs, u128};

#[derive(Debug)]
struct Route {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Result {
    step: usize,
    value: String,
}
#[derive(Debug)]
struct Starting_Position {
    start_step: usize,
    value: String,
}

pub fn day8_1() {
    let conent = fs::read_to_string("./inputs/input8").expect("to be able to read file");
    let mut lines_iter = conent.trim().split("\n");

    let directions = lines_iter.next().expect("to have orders");
    lines_iter.next().expect("to skip empty line");

    let mut map_of_routes: HashMap<String, Route> = HashMap::new();
    for line in lines_iter {
        let mut two_parts = line.split("=");
        let key = two_parts.next().expect("to have key").trim();

        let without_one_brace = two_parts.next().expect("to have routes").replace("(", "");
        let without_braces = without_one_brace.replace(")", "");
        let only_routes = without_braces.replace(",", "");
        let mut only_routes_iter = only_routes.split_whitespace();

        map_of_routes.insert(
            String::from(key),
            Route {
                left: String::from(only_routes_iter.next().expect("to have left")),
                right: String::from(only_routes_iter.next().expect("to have right")),
            },
        );
    }

    let mut starting_positios: Vec<Starting_Position> = vec![];
    for k in map_of_routes.keys() {
        if k.chars().last().expect("to exist") == 'A' {
            let s = Starting_Position {
                start_step: 0,
                value: String::from(k),
            };
            starting_positios.push(s)
        }
    }

    let mut finish = true;

    while finish {
        let mut res_vec: Vec<Result> = vec![];

        println!("{:?}, {:?}", starting_positios, res_vec);
        for start in &starting_positios {
            let res = go_to_next_route(
                &start.value,
                start.start_step,
                &map_of_routes,
                String::from(directions),
                true,
            );
            res_vec.push(res)
        }
        println!("{:?}", res_vec);
        let mut output: u128 = 1;
        for r in &res_vec {
            let step_as_u: u128 = r.step.try_into().expect("to work");
            output = output * step_as_u;
        }
        println!("OUTPUT--------> {output}"); // Ok I needed to calculate lcm from outputs in here.
                                              // Did it in browser

        if all_elements_the_same(&res_vec) {
            finish = false;
            println!("FINISHED! {:?}", res_vec);
        } else {
            starting_positios.clear();
            for res in &res_vec {
                starting_positios.push(Starting_Position {
                    value: res.value.clone(),
                    start_step: res.step,
                })
            }
            res_vec.clear();
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn all_elements_the_same(v: &Vec<Result>) -> bool {
    let mut v_iter = v.iter();
    let first_el = v_iter.next().expect("To get first element");
    for el in v_iter {
        if el.step != first_el.step {
            return false;
        }
    }
    return true;
}

fn go_to_next_route(
    next: &String,
    step: usize,
    map_of_routes: &HashMap<String, Route>,
    directions: String,
    first_iter: bool,
) -> Result {
    if next.chars().last().expect("to exist") == 'Z' && !first_iter {
        let clone_val = next.clone();
        return Result {
            step,
            value: clone_val,
        };
    }
    let route = map_of_routes.get(next).expect("to have next route");

    let next_next = match directions
        .chars()
        .nth(step % directions.len())
        .expect("to have next order")
    {
        'L' => &route.left,
        'R' => &route.right,
        _ => panic!("UNEXPECTED ARM"),
    };

    go_to_next_route(next_next, step + 1, &map_of_routes, directions, false)
}
