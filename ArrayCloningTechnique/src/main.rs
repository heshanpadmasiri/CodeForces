use std::io;
use std::cmp;

fn main() {
    let t = read_int();
    for _ in 0..t {
        println!("{}", solve());
    }
}

fn solve() -> i64 {
    let n = read_int();
    let mut array = read_array();
    // println!("{:?}", array);
    let mut x = most_common_count(&mut array);
    let mut op_count = 0;
    while n > x {
        // duplicate
        op_count += 1;
        // swap 
        let swap_count = cmp::min(n-x, x);
        // println!("{n}, {x}, {swap_count}");
        op_count += swap_count;
        x += swap_count;
    }
    return op_count;
}

fn most_common_count(arr: &mut Vec<i64>) -> i64 {
    arr.sort_unstable();
    let mut current_val = arr[0];
    let mut current_count = 1;
    let mut highest_count = 1;
    for i in 1..arr.len() {
        if arr[i] != current_val {
            highest_count = cmp::max(highest_count, current_count);
            current_val = arr[i];
            current_count = 1;
        }
        else {
            current_count += 1;
        }
    }
    highest_count = cmp::max(highest_count, current_count);
    return highest_count;
}

fn read_int() -> i64 {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_array() -> Vec<i64> {
    let line = read_line();
    return line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
