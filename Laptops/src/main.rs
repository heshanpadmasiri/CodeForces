use std::io;

#[derive(PartialEq, Eq)]
struct Laptop {
    price: usize,
    quality: usize
}

impl PartialOrd for Laptop {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { 
        return Some(self.cmp(other));
    }
}

impl Ord for Laptop {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.price != other.price {
            return self.price.cmp(&other.price);
        }
        return match self.quality.cmp(&other.quality) {
            std::cmp::Ordering::Less => { std::cmp::Ordering::Greater },
            std::cmp::Ordering::Greater => { std::cmp::Ordering::Less },
            std::cmp::Ordering::Equal => { std::cmp::Ordering::Greater } 
        }
    }
}

fn main() {
    let mut laptops = read_input();
    laptops.sort();
    for i in 0 .. laptops.len() - 1 {
        if laptops[i].quality > laptops[i+1].quality {
            println!("Happy Alex");
            return;
        }
    }
    println!("Poor Alex");
}

fn read_input() -> Vec<Laptop> {
    let n = read_int();
    let mut laptops = Vec::with_capacity(n);
    for _ in 0 .. n {
        let data = read_array();
        laptops.push(Laptop{ price: data[0], quality: data[1] });
    }
    return laptops;
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
