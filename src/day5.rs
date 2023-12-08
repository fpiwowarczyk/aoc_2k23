use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Mappendo {
    destination: i64,
    source: i64,
    range: i64,
}

pub fn day5_1() {
    let mut maps: HashMap<String, Vec<Mappendo>> = HashMap::new();
    let start_vaules = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];
    let mut seeds: Vec<i64> = vec![];
    let content = fs::read_to_string("./inputs/input5").expect("to read file");
    let seeds_and_maps: Vec<_> = content.trim().split("\n\n").collect();

    for seed_or_map in seeds_and_maps {
        if seed_or_map.starts_with("seeds:") {
            let temp_seeds: Vec<i64> = seed_or_map
                .replace("seeds: ", "")
                .split_whitespace()
                .map(|el| el.parse().unwrap())
                .collect();
            let mut start: i64 = 0;
            for (i, val) in temp_seeds.iter().enumerate() {
                if i % 2 == 0 {
                    start = *val;
                } else {
                    for n in 1..*val {
                        seeds.push(start + n)
                    }
                }
            }
        }
        // println!("{:?}", seeds);
        for val in &start_vaules {
            if seed_or_map.contains(val) {
                let maps_smth_iter = seed_or_map.replace(val, "");
                let map_split_by_line = maps_smth_iter.split("\n");
                let map_lines: Vec<String> = map_split_by_line.map(|el| String::from(el)).collect();
                let mut maps_in_block: Vec<Mappendo> = vec![];
                for map in map_lines {
                    if map.len() == 0 {
                        continue;
                    }
                    let mut map_iter = map.split_whitespace();
                    let m = Mappendo {
                        destination: map_iter
                            .next()
                            .expect("to expist")
                            .parse()
                            .expect("to work"),
                        source: map_iter
                            .next()
                            .expect("to expist")
                            .parse()
                            .expect("to work"),
                        range: map_iter.next().expect("to exist").parse().expect("to work"),
                    };

                    maps_in_block.push(m)
                }
                maps.insert(String::from(*val), maps_in_block);
            }
        }
    }

    println!("{}", seeds.len());

    let mut lowest_number: i64 = i64::MAX;
    let mut i: i64 = 0;
    for seed in seeds {
        let mut number: i64 = seed;
        let mut to_add: i64 = 0;
        for val in &start_vaules {
            let ms: &Vec<Mappendo> = maps.get(*val).expect("to exist");
            let mut possible_numbers: Vec<i64> = Vec::new();
            for m in ms {
                if m.source + m.range > number && number >= m.source {
                    to_add = number - m.source;
                    possible_numbers.push(m.destination + to_add);
                }
            }
            if possible_numbers.len() > 0 {
                number = possible_numbers.pop().expect("to exist");
            }
            for poss in possible_numbers {
                if number > poss {
                    number = poss;
                }
            }
        }

        if number < lowest_number {
            println!("New number! {number}");
            lowest_number = number;
        }
        i = i + 1;
    }
    println!("{lowest_number}",)
}
