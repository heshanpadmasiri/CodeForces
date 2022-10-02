use std::io;

const CHARS: &'static [&'static char] = &[&'h', &'e', &'l', &'l', &'o'];

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read n");
    let mut i = 0;
    for char in s.chars() {
        if char == *CHARS[i] {
            i += 1;
            if i == 5 {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
