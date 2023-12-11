use std::fs;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    value: char,
}

pub fn day10_1() {
    let conent = fs::read_to_string("./inputs/input10").expect("to be able to read file");

    let lines: Vec<_> = conent.trim().split("\n").collect();

    println!("{:?}", lines);

    let mut board: Vec<Vec<char>> = vec![];

    for line in lines {
        let mut characters = vec![];
        for s in line.chars() {
            characters.push(s);
        }
        board.push(characters);
    }
    // bard[y,x] starting from top left
    let starting_point = find_starting_point(board);

    println!("Starting point {:?}", starting_point)
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

// if x,y -1, dead end.
fn get_next_point(p: Point, dir: Direction, board: &Vec<Vec<char>>) {
    if dir == Direction::None {
        panic!("Direction none!")
    }
    match p.value {
        '|' => {
            let next_dir = match dir {
                Direction::Down => Direction::Down,
                Direction::Up => Direction::Up,
                _ => {
                    println!("Unwanted direction");
                    Direction::None
                }
            };

            let next_point = get_point_dir(p, &next_dir, board);
            match next_point.value {
                'L' | 'J' | '|' | '7' | 'F' => get_next_point(next_point, next_dir, board),
                _ => (), // Other options are dead ends
            }
        }
        '-' => {
            let next_dir = match dir {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                _ => {
                    println!("Unwanted direction");
                    Direction::None
                }
            };
            let next_point = get_point_dir(p, &next_dir, board);
            match next_point.value {
                'L' | 'J' | '-' | '7' | 'F' => println!("Dead end {:?}", next_point),
                _ => get_next_point(next_point, next_dir, board), // Other options are dead ends
            }
        }
        // | is a vertical pipe connecting north and south.
        // - is a horizontal pipe connecting east and west.
        // L is a 90-degree bend connecting north and east.
        // J is a 90-degree bend connecting north and west.
        // 7 is a 90-degree bend connecting south and west.
        // F is a 90-degree bend connecting south and east.
        // . is ground; there is no pipe in this tile.
        // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
        'L' => {
            let next_dir = match dir {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => {
                    println!("Unwanted direction");
                    Direction::None
                }
            };
            let next_point = get_point_dir(p, &next_dir, board);
            match next_point.value {
                'L' | 'J' | '|' => get_next_point(next_point, next_dir, board),
                _ => (), // Other options are dead ends
            }
        }
        'J' => {
            let next_dir = match dir {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => {
                    println!("Unwanted direction");
                    Direction::None
                }
            };
            let next_point = get_point_dir(p, &next_dir, board);
            match next_point.value {
                'L' | 'J' | '|' => get_next_point(next_point, next_dir, board),
                _ => (), // Other options are dead ends
            }
        }
        '7' => {
            let next_dir = match dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                _ => {
                    println!("Unwanted direction");
                    Direction::None
                }
            };
            let next_point = get_point_dir(p, &next_dir, board);
            match next_point.value {
                'L' | 'J' | '|' => get_next_point(next_point, next_dir, board),
                _ => (), // Other options are dead ends
            }
        }
        'F' => {
            let next_dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                _ => {
                    println!("Unwanted direction");
                    Direction::None
                }
            };
            let next_point = get_point_dir(p, &next_dir, board);
            match next_point.value {
                'L' | 'J' | '|' => get_next_point(next_point, next_dir, board),
                _ => (), // Other options are dead ends
            }
        }
        'S' => {
            panic!("LOOP")
        }
        _ => {
            println!("Dead end {}", p.value)
        }
    }
}

fn find_starting_point(board: Vec<Vec<char>>) -> Point {
    for (y, line) in board.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if ch == &'7' {
                return Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    value: '7',
                };
            }
        }
    }
    panic!("I shoudn't be here");
}

fn check_field(x: i32, y: i32) -> char {
    let mut check_x = x;
    let mut check_y = y;
    if x < 0 {
        check_x = 0;
    }
    if y < 0 {
        check_y = 0;
    }

    return '.';
}

fn get_point_dir(cur_p: Point, dir: &Direction, board: &Vec<Vec<char>>) -> Point {
    match dir {
        Direction::Up => get_point(cur_p.x, cur_p.y - 1, board),
        Direction::Down => get_point(cur_p.x, cur_p.y + 1, board),
        Direction::Left => get_point(cur_p.x - 1, cur_p.y, board),
        Direction::Right => get_point(cur_p.x + 1, cur_p.y, board),
        Direction::None => panic!("None direction!"),
    }
}

fn get_point(x: i32, y: i32, board: &Vec<Vec<char>>) -> Point {
    let ux: usize = x.try_into().unwrap();
    let uy: usize = y.try_into().unwrap();

    let mut value = '.';
    if !(x < 0 || y < 0) {
        // Maybe I need to add len of board
        value = board[uy][ux];
    }
    return Point { x, y, value };
}
