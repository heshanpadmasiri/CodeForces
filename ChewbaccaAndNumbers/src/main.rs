use std::{cmp, io};

type Int = u64;

fn main() {
    let n = read_int();
    let ans = get_minimum_inversion(n);
    println!("{}", ans);
}

fn get_minimum_inversion(n: Int) -> Int {
    let digits: Vec<Int> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Int)
        .collect();
    let flipped_digits: Vec<Int> = digits.iter().map(|digit| 9 - digit).collect();
    let mut ans = 0;
    let base: Int = 10;
    for i in 0..digits.len() {
        let digit = if i == 0 && flipped_digits[i] == 0 {
            digits[i]
        } else {
            cmp::min(digits[i], flipped_digits[i])
        };
        let exp = (digits.len() - i - 1).try_into().unwrap();
        ans += digit * base.pow(exp)
    }
    ans
}

fn read_int() -> Int {
    let line = read_line();
    line.trim().parse().expect("expect an integer")
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    line
}
