use std::fs;
use std::collections::HashMap;

fn main() {
    let read_file = fs::read_to_string("input").expect("should able to read");
    println!("Solution 1: {}", problem_one(&read_file));
    println!("Solution 2: {}", problem_two(&read_file));
}

fn problem_one(input: &String) -> i32 {
    let mut list: Vec<i32> = Vec::new();
    let mut number: i32;
    let mut digit1: u32;
    let mut digit0: u32;

    for line in input.split('\n') {
        digit0 = 0;
        digit1 = 0;
        
        for c in line.chars() {
            if c.is_ascii_digit() {
                if digit1 == 0 {
                    digit1 = c.to_digit(10).unwrap();
                } else {
                    digit0 = c.to_digit(10).unwrap();
                }
            }
        }

        number = digit1 as i32 * 10;
        if digit0 != 0 {
            number += digit0 as i32;
        } else {
            number += digit1 as i32;
        }

        list.push(number);
    }

    list.iter().sum::<i32>()
}

fn problem_two(input: &String) -> i32 {
    let mut list: Vec<i32> = Vec::new();
    let mut buffer = String::new();

    let collection: HashMap<String, u32> =  HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    for line in input.split('\n') {
        let mut number: i32 = 0;
        let mut flag: bool = false;
        
        for c in line.chars() {
            if c.is_digit(10) {
                number = c.to_digit(10).unwrap() as i32 * 10;
                break;
            }
            
            buffer.push(c);
            for key in collection.keys() {
                if buffer.contains(key) {
                    number = collection[key] as i32 * 10;
                    flag = true;
                }
            }
            
            if flag {
                break;
            }
        }

        flag = false;
        buffer.clear();
        for c in line.chars().rev() {
            if c.is_digit(10) {
                number += c.to_digit(10).unwrap() as i32;
                break;
            }
            
            buffer.insert(0, c);
            for key in collection.keys() {
                if buffer.contains(key) {
                    number += collection[key] as i32;
                    flag = true;
                }
            }
            
            if flag {
                break;
            }    
        }

        list.push(number);
        buffer.clear();
    }

    list.iter().sum::<i32>()
}
