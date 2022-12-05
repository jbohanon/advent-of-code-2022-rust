use std::fs;
use std::env;
use std::str::Split;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("must have at least one arg: \n partone|parttwo\n (optionally) trivial")
    }
    let mut trivial = false;
    if args.len() > 2 && args[2] == "trivial" {
        trivial = true;
    }
    let filename = format!("src/{}input", || -> String { if trivial {return "trivial".to_string()}; return "".to_string();}());
    let inp = fs::read_to_string(filename).unwrap_or_default();
    let spl = inp.split('\n');

    if args[1] == "partone" {
        calculate(spl, new_part_one_map());
    } else if args[1] == "parttwo" {
        calculate(spl, new_part_two_map());
    } else {
        panic!("unknown arg {}", args[1]);
    }
}

fn calculate(spl: Split<char>, point_val_map: HashMap<String, Play>) {
    let mut score = 0;

    let mut i = 0;
    for val in spl {
        if val == "" {
            continue;
        }
        i += 1;
        dbg!(val);
        let kv = val.split_once(' ').unwrap_or_default();
        let play = &point_val_map[kv.0];
        score += play[&String::from(kv.1)];
    }

    println!("{} in {} plays", score, i);
}

// A Play represents a rock, paper, or scissors (A/X, B/Y, C/Z) play
type Play = HashMap<String, i32>;

// These values represent the opponent's play (A, B, C) and the points garnered for each possible
// strategy-guide proposed counter play
fn new_part_one_map() -> HashMap<String, Play> {
    return HashMap::from([
                              (String::from("A"), HashMap::from([(String::from("X"), (3+1)), (String::from("Y"), (6+2)), (String::from("Z"), (0+3))])),
                              (String::from("B"), HashMap::from([(String::from("X"), (0+1)), (String::from("Y"), (3+2)), (String::from("Z"), (6+3))])),
                              (String::from("C"), HashMap::from([(String::from("X"), (6+1)), (String::from("Y"), (0+2)), (String::from("Z"), (3+3))])),
    ]);
}

// These values represent the opponent's play (A, B, C) and the points garnered for each possible
// strategy-guide proposed desired outcome
fn new_part_two_map() -> HashMap<String, Play> {
    return HashMap::from([
                              (String::from("A"), HashMap::from([(String::from("X"), (3+0)), (String::from("Y"), (1+3)), (String::from("Z"), (2+6))])),
                              (String::from("B"), HashMap::from([(String::from("X"), (1+0)), (String::from("Y"), (2+3)), (String::from("Z"), (3+6))])),
                              (String::from("C"), HashMap::from([(String::from("X"), (2+0)), (String::from("Y"), (3+3)), (String::from("Z"), (1+6))])),
    ]);
}
