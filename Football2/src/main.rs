use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read n");
    let n: i64 = n.trim().parse().expect("Expect n to be integer");

    println!("{}", solve(n));
}

struct TeamCount {
    name:String,
    count: i64
}

fn solve(n:i64) -> String {
    let mut teams:Vec<TeamCount> = Vec::new();
    for _ in 0..n {
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read team name");
        let mut found = false;
        for mut team in teams.iter_mut() {
            if team.name == name {
                team.count += 1;
                found = true;
                break;
            }
        }
        if !found {
            teams.push(TeamCount {
                name, 
                count: 1
            });
        }
    }
    let m = teams.iter().max_by_key(|team| team.count).unwrap();
    return m.name.to_string();
}
