use std::io;

#[derive(PartialEq, Clone, Copy)]
enum Expectation {
    WinOne,
    NoLoss
}

#[derive(PartialEq, Clone, Copy)]
enum GameResult {
    Win,
    Loss,
    Draw
}

fn main() {
    let t = read_int();
    for _ in 0..t {
        let result = solve();
        match result {
            Err(_) =>  {println!("NO")}
            Ok(result) => {
                println!("YES");
                print_result(result);
            }
        }
    }
}

fn print_result(result: Vec<Vec<GameResult>>) {
    let n = result.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                print!("X");
                continue;
            }
            match result[i][j] {
                GameResult::Draw => { print!("=") }
                GameResult::Win => { print!("+") }
                GameResult::Loss => { print!("-") }
            }
        }
        print!("\n");
    }
}

fn solve() -> Result<Vec<Vec<GameResult>>, ()> {
    let n = read_int() as usize;
    let expectations = read_expectation();
    let mut results = Vec::with_capacity(n);
    for player_1 in 0..n {
        let expectation_1 = &expectations[player_1];
        let mut games = Vec::with_capacity(n);
        let mut won_game = false;
        for player_2 in 0..n {
            let expectation_2 = &expectations[player_2];
            match current_result(&results, player_1, player_2) {
                Some(current_result) => {
                    if current_result == GameResult::Win {
                        won_game = true;
                    }
                    match (current_result, expectation_1, expectation_2) {
                        (GameResult::Loss, _, Expectation::NoLoss) | (GameResult::Loss, Expectation::NoLoss, _) => {
                            return Err(());
                        }
                        _ => {
                            games.push(current_result);
                        }
                    }
                }
                None => {
                    match (expectation_1, expectation_2) {
                        // (Expectation::NoLoss, Expectation::NoLoss) => { games.push(GameResult::Draw) }
                        // (Expectation::NoLoss, Expectation::WinOne) => { games.push(GameResult::Win) }
                        (Expectation::NoLoss, _) | (_, Expectation::NoLoss) => { games.push(GameResult::Draw) }
                        (Expectation::WinOne, Expectation::WinOne) if !won_game => {
                            games.push(GameResult::Win);
                            won_game = true;
                        }
                        (Expectation::WinOne, Expectation::WinOne) => {
                            games.push(GameResult::Loss);
                        }
                    }

                }
            }
        }
        if *expectation_1 == Expectation::WinOne && !won_game {
            return Err(());
        }
        results.push(games);
    }
    return Ok(results);
}

fn current_result(state: &Vec<Vec<GameResult>>, player_1: usize, player_2: usize) -> Option<GameResult> {
    if player_1 == player_2 {
        return Some(GameResult::Draw);
    }
    if player_1 > player_2 {
        if player_2 >= state.len() { 
            // Not sure if this can happen
            return None;
        }
        let results = &state[player_2];
        if player_1 >= results.len() {
            // Not sure if this can happen
            return None;
        }
        return Some(match results[player_1] {
            GameResult::Draw => { GameResult::Draw },
            GameResult::Win => { GameResult::Loss },
            GameResult::Loss => { GameResult::Win }
        });
    }
    else {
        if player_1 >= state.len() {
            return None;
        }
        let results = &state[player_1];
        if player_2 >= results.len() {
            return None;
        }
        // not sure if this can happen
        return Some(results[player_2]);
    }
}


fn read_expectation() -> Vec<Expectation> {
    let input = read_line();
    return input.chars().map(|each| { if each == '1' { Expectation::NoLoss } else { Expectation::WinOne }
    }).collect();
}

fn read_int() -> i64 {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
