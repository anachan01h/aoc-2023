use std::fs;

const LINE_SIZE: i32 = 141;
const LINE_NUM: i32 = 140;

fn main() {
    let read_file = fs::read_to_string("input").expect("should able to read");
    let mut i = -1;
    let read_file: Vec<_> = read_file
        .chars()
        .map(|c| {
            i += 1;
            (i, c)
        })
        .collect();

    println!("Solution 1: {}", problem_one(&read_file));
    println!("Solution 2: {}", problem_two(&read_file));
}

fn problem_one(input: &Vec<(i32, char)>) -> i32 {    
    let mut number = 0;
    let mut start = -1;
    let mut end;
    let mut sum = 0;

    for (i, c) in input {
        if c.is_digit(10) {
            if start < 0 {
                start = *i;
            }
            number = number * 10 + c.to_digit(10).unwrap() as i32;
        } else if start >= 0 {
            end = *i as i32;
            let l = start / LINE_SIZE;
            let cmin = start % LINE_SIZE - 1;
            let cmax = end % LINE_SIZE;

            'my_loop: for linha in l - 1..l + 2 {
                if linha < 0 || linha >= LINE_SIZE - 1 {
                    continue;
                }
                for coluna in cmin..cmax + 1 {
                    if coluna < 0 || coluna >= LINE_NUM {
                        continue;
                    }

                    if !input[(linha * LINE_SIZE + coluna) as usize].1.is_digit(10)
                        && !input[(linha * LINE_SIZE + coluna) as usize]
                            .1
                            .is_whitespace()
                        && input[(linha * LINE_SIZE + coluna) as usize].1 != '.'
                    {
                        sum += number;
                        break 'my_loop;
                    }
                }
            }

            number = 0;
            start = -1;
        }
    }

    sum
}

fn problem_two(input: &Vec<(i32, char)>) -> i32 {
    let mut gear_adj = Vec::new();

    for (i, c) in input {
        if *c == '*' {
            let mut gear = Vec::new();
            let line = i / LINE_SIZE;
            let col = i % LINE_SIZE;
            for linha in line - 1 .. line + 2 {
                let mut j = col - 1;
                while j < col + 2 {
                    if input[(linha * LINE_SIZE + j) as usize].1.is_digit(10) {
                        let mut k = linha * LINE_SIZE + j;
                        while input[k as usize].1.is_digit(10) && k != 0 {
                            k -= 1;
                        }
                        let (number, end) = get_number(input, k + 1);
                        j = end % LINE_SIZE;
                        gear.push(number);
                    } else {
                        j += 1;
                    }
                }
            }
            gear_adj.push(gear);
        }
    }

    gear_adj.iter().filter_map(|v| {
        if v.len() == 2 {
            Some(v.iter().product::<i32>())
        } else {
            None
        }
    }).sum::<i32>()
}

fn get_number(input: &Vec<(i32, char)>, start: i32) -> (i32, i32) {
    let mut i = start;
    let mut number = 0;

    while input[i as usize].1.is_digit(10) {
        number = number * 10 + input[i as usize].1.to_digit(10).unwrap() as i32;
        i += 1;
    }

    (number, i)
}