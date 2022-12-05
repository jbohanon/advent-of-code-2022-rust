use std::collections::HashMap;
use std::ops::BitAnd;
use bitmaps;
use std::fs;
use std::env;
use std::str::Split;

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
        calculate(spl);
    } else if args[1] == "parttwo" {
        calculate(spl);
    } else {
        panic!("unknown arg {}", args[1]);
    }
}

fn calculate(spl: Split<char>) {
    let mut total = 0;
    for line_val in spl {
        if line_val == "" {
            continue;
        }
        dbg!(line_val);
        total += check_rucksack(String::from(line_val))
    }
    println!("{}", total);
}

fn check_rucksack(val: String) -> i32 {
    let item_map = new_item_map();
    let mut compartment_1: bitmaps::Bitmap<52> = bitmaps::Bitmap::new();
    let mut compartment_2: bitmaps::Bitmap<52> = bitmaps::Bitmap::new();
    let midpoint = val.len()/2;
    let mut i = 1;
    for c in val.chars() {
        if i as usize > midpoint {
        compartment_2.set((item_map[&String::from(c)]-1) as usize, true);
        } else {
        compartment_1.set((item_map[&String::from(c)]-1) as usize, true);
        }
        i += 1;
    }
    dbg!(compartment_1);
    dbg!(compartment_2);

    let res = compartment_1.bitand(compartment_2);
    let prio =res.first_index().unwrap_or_default() as i32 +1;
    dbg!(prio);
    return prio;
}

fn new_item_map() -> HashMap<String, i32> {
    return HashMap::from([
                         (String::from("a"), 1),
                         (String::from("b"), 2),
                         (String::from("c"), 3),
                         (String::from("d"), 4),
                         (String::from("e"), 5),
                         (String::from("f"), 6),
                         (String::from("g"), 7),
                         (String::from("h"), 8),
                         (String::from("i"), 9),
                         (String::from("j"), 10),
                         (String::from("k"), 11),
                         (String::from("l"), 12),
                         (String::from("m"), 13),
                         (String::from("n"), 14),
                         (String::from("o"), 15),
                         (String::from("p"), 16),
                         (String::from("q"), 17),
                         (String::from("r"), 18),
                         (String::from("s"), 19),
                         (String::from("t"), 20),
                         (String::from("u"), 21),
                         (String::from("v"), 22),
                         (String::from("w"), 23),
                         (String::from("x"), 24),
                         (String::from("y"), 25),
                         (String::from("z"), 26),
                         (String::from("A"), 27),
                         (String::from("B"), 28),
                         (String::from("C"), 29),
                         (String::from("D"), 30),
                         (String::from("E"), 31),
                         (String::from("F"), 32),
                         (String::from("G"), 33),
                         (String::from("H"), 34),
                         (String::from("I"), 35),
                         (String::from("J"), 36),
                         (String::from("K"), 37),
                         (String::from("L"), 38),
                         (String::from("M"), 39),
                         (String::from("N"), 40),
                         (String::from("O"), 41),
                         (String::from("P"), 42),
                         (String::from("Q"), 43),
                         (String::from("R"), 44),
                         (String::from("S"), 45),
                         (String::from("T"), 46),
                         (String::from("U"), 47),
                         (String::from("V"), 48),
                         (String::from("W"), 49),
                         (String::from("X"), 50),
                         (String::from("Y"), 51),
                         (String::from("Z"), 52),
    ])
}
