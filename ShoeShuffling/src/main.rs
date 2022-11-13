use std::io;

fn main() {
    let t = read_int();
    for _ in 0..t {
        println!("{}", match solve() {
            None => "-1".to_string(),
            Some(array) => array_to_string(&array)
        });
    }
}

fn solve() -> Option<Vec<usize>> {
    let n = read_int() as usize;
    let shoes = read_array();
    let mut i = 0;
    let mut ans = Vec::with_capacity(n);
    while i < n {
        let current_size = shoes[i];
        let mut j = i;
        while j < n {
            if shoes[j] != current_size {
                let end_index = j - 1;
                if end_index == i {
                    return None;
                }
                ans.extend_from_slice(&shuffled_indicies(j, i+1));
                i = j;
                break;
            }
            j += 1;
            if j == n {
                let end_index = j - 1;
                if end_index == i {
                    return None;
                }
                ans.extend_from_slice(&shuffled_indicies(n, i+1));
                i = j;
                break;
            }
        }
    }
    Some(ans)
}

fn shuffled_indicies(end_index: usize, start_index: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (start_index..(end_index+1)).collect();
    v.rotate_right(1);
    return v;
}

fn array_to_string(array: &[usize]) -> String {
    let string_vec: Vec<String> = array.into_iter().map(usize::to_string).collect();
    return string_vec.join(" ");
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
