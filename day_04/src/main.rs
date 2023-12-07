use std::fs;

fn main() {
    let read_file = fs::read_to_string("input").expect("should able to read");
    println!("Solution 1: {}", problem_one(&read_file));
    println!("Solution 2: {}", problem_two(&read_file));
}

fn convert_input(input: &String) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut scratchcards = Vec::new();
    for line in input.lines() {
        let mut parts = line.split([':', '|']);
        parts.next();
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let left: Vec<i32> = left.split(' ').filter_map(|s| {
            let new_s = s.trim();
            if new_s == "" {
                None
            } else {
                Some(new_s.parse::<i32>().unwrap())
            }
        }).collect();

        let right: Vec<i32> = right.split(' ').filter_map(|s| {
            let new_s = s.trim();
            if new_s == "" {
                None
            } else {
                Some(new_s.parse::<i32>().unwrap())
            }
        }).collect();

        scratchcards.push((left, right));
    }

    scratchcards
}

fn problem_one(input: &String) -> i32 {
    let scratchcards = convert_input(input);

    scratchcards.iter().map(|par| {
        let exp = par.1.iter().map(|n| {
            if par.0.contains(n) {
                1
            } else {
                0
            }
        }).sum::<i32>() - 1;

        let base: i32 = 2;
        if exp >= 0 {
            base.pow(exp as u32)
        } else {
            0
        }
    }).sum::<i32>()
}

fn problem_two(input: &String) -> i32 {
    let scratchcards = convert_input(input);
    let mut cards = [1; 214];

    let mut card = 0;
    for par in scratchcards {
        let mut wins = 0;
        for n in par.1 {
            if par.0.contains(&n) {
                wins += 1;
                cards[card + wins] += cards[card];
            }
        }
        
        card += 1;
    }

    cards.iter().sum::<i32>()
}
