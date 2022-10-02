use std::io;

fn main() {
    let t = read_int();
    
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    let n = read_int();
    let max7:i64 = n / 7;
    let max5:i64 = n / 5;
    let max3:i64 = n / 3;
    for a in 0..=max7 {
        for b in 0..=max5 {
            for c in 0..=max3 {
                if a * 7 + b * 5 + c * 3 == n {
                    println!("{c} {b} {a}");
                    return;
                }
            }
        }
    } 

    println!("-1");
}

fn read_int() -> i64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");
    return line.trim().parse().expect("expect line to be an integer");
}
