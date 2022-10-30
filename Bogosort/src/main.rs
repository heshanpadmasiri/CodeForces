use std::io;

fn main() {
    let t = read_int();
    for _ in 0..t {
        let array = solve();
        println!("{}", array_to_str(&array));
    }
}

fn solve() -> Vec<i64> {
    let _ = read_int();
    let mut array = read_array();
    array.sort_unstable_by(|a, b| b.cmp(a));
    return array;
}

fn array_to_str(array: &Vec<i64>) -> String {
    let string_array = array.iter().map(i64::to_string).collect::<Vec<String>>();
    return string_array.join(" ");
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
