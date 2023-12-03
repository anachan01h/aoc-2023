use std::fs;


fn main() {
    let read_file = fs::read_to_string("input").expect("should able to read");
    println!("Solution 1: {}", problem_one(&read_file));
    println!("Solution 2: {}", problem_two(&read_file));
}

fn convert_input(input: &String) -> Vec<(i32, Vec<(i32, i32, i32)>)> {
    let mut results = Vec::new();
    for line in input.lines() {
        let mut result = (0, Vec::new());
        for elem in line.split(|c: char| [':', ';'].contains(&c)) {
            if elem.contains("Game") {
                result.0 = elem.replace("Game", "").trim().parse().expect("Expect find a number not a string");
                continue;
            }

            let mut colors = (0, 0, 0);
            for color in elem.split(',') {
                if color.contains("red") {
                    colors.0 = color.replace("red", "").trim().parse().expect("Expect find a number not a string");
                } else if color.contains("green") {
                    colors.1 = color.replace("green", "").trim().parse().expect("Expect find a number not a string");
                } else if color.contains("blue") {
                    colors.2 = color.replace("blue", "").trim().parse().expect("Expect find a number not a string");
                }
            }

            result.1.push(colors);
        }

        results.push(result);
    }

    results
}

fn problem_one(input: &String) -> i32 {
    let results = convert_input(input);

    results.iter().filter_map(|game| {
        let value = game.1.iter().map(|round| {
            if round.0 <= 12 && round.1 <= 13 && round.2 <= 14 {
                true
            } else {
                false
            }
        }).min().unwrap();
        
        if value {
            Some(game.0)
        } else {
            None
        }
    }).sum::<i32>()
}

fn problem_two(input: &String) -> i32 {
    let results = convert_input(input);

    results.iter().map(|game| {
        let mut minimum_set = [0, 0, 0];
        minimum_set[0] = game.1.iter().map(|t| t.0).max().unwrap();
        minimum_set[1] = game.1.iter().map(|t| t.1).max().unwrap();
        minimum_set[2] = game.1.iter().map(|t| t.2).max().unwrap();
        minimum_set.iter().product::<i32>()
    }).sum::<i32>()
}
