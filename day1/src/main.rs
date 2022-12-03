use std::convert::TryInto;
use std::fs;
use std::env;
use std::str::Split;

// input is number of calories per elf
// each elf writes down calories/meal line delimited
// elves separated by blank line
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
        find_top(spl, 1);
    } else if args[1] == "parttwo" {
        find_top(spl, 3);
    } else {
        panic!("unknown arg {}", args[1]);
    }
}

fn find_top(spl: Split<char>, num_results: i32) {
    // find which elf/elves carry the most calories

    let mut top_vals = vec![0; num_results.try_into().unwrap()];
    let mut curr_run = 0;
    for line_val in spl {
        if line_val == "" {
            top_vals = check_val(curr_run, top_vals);
            curr_run = 0;
            continue;
        }
        let curr_int = line_val.parse::<i32>().unwrap_or_default();
        if curr_int > 0 {
            curr_run += curr_int;
        }
    }
    let out = format!("{:?}", top_vals);
    println!("{}", out);
    println!("{}", top_vals.iter().sum::<i32>());
}

fn check_val(val: i32, mut top_vals: Vec<i32>) -> Vec<i32> {
    for i in 0..top_vals.len() {
        if val > top_vals[i] {
            let replaced_val = top_vals[i];
            top_vals[i] = val;
            top_vals = check_val(replaced_val, top_vals);
            break
        }
    }
    return top_vals
}
