use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let data = line.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
    let m = data[1];

    line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let mut prices = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
    prices.sort();
    
    let ans:i64 = prices.into_iter().take(m).filter(|&price| price < 0).sum::<i64>() * -1;
    // let tmp = prices.into_iter().take(m).filter(|&price| price < 0).collect::<Vec<_>>();
    println!("{}", ans);
}
