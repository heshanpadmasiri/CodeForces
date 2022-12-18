use std::io;

fn main() {
    let _ = read_int();
    let mut error_0 = read_array();
    let mut error_1 = read_array();
    let mut error_2 = read_array();
    error_0.sort();
    error_1.sort();
    error_2.sort();
    println!("{}", missing_one(&mut error_0, &mut error_1));
    println!("{}", missing_one(&mut error_1, &mut error_2));
}

fn missing_one(larger: &mut Vec<usize>, smaller: &mut Vec<usize>) -> usize {
    let mut large_index = 0;
    let mut small_index = 0;
    while small_index < smaller.len() {
        if larger[large_index] == smaller[small_index] {
            large_index += 1;
            small_index += 1;
        }
        else {
            return larger[large_index];
        }
    }
    larger[large_index]
}

fn read_int() -> usize {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_array() -> Vec<usize> {
    let line = read_line();
    return line.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
