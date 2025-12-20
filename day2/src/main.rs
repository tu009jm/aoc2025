use regex::Regex;
use std::fs;

// für Tom: i als Laufvariable wird im Folgenden als f definiert
// alle weiteren wiederholen sich -> also f,ff,fff,ffff

fn main() {
    auf1();
}

fn auf1() {
    let mut data_total =
        fs::read_to_string("/Users/jannesmuller/Developer/rust/aoc2025/day2/input.txt").unwrap();
    data_total = data_total.replace('\n', "");
    let data_vec_1: Vec<&str> = data_total.split(',').collect();
    let mut counter = 0;
    for f in data_vec_1 {
        let data_vec_2: Vec<&str> = f.split('-').collect();
        let x: u128 = data_vec_2.get(0).unwrap().parse::<u128>().unwrap();
        // dbg!(data_vec_2.get(1).unwrap());
        let y: u128 = data_vec_2.get(1).unwrap().parse::<u128>().unwrap();

        for d in x..y + 1 {
            if is_repetition(d.to_string().as_str()) {
                counter += d;
            }
            // counter += _validate_3(d);
        }
    }
    dbg!(counter);
}

fn _validate(input: u128) -> u128 {
    let number = input.to_string();
    let number_length = number.chars().count() / 2;
    let (x, y) = number.split_at(number_length);
    if x == y {
        // dbg!(x);
        return input;
    } else {
        return 0;
    }
}

fn _validate_2(input: u128) -> u128 {
    let number = input.to_string();
    let mut is_the_same = true;

    for i in 1..number.len() {
        if number.len() % i as usize == 0 {
            let mut reg = "^".to_string();
            for _ in 0..number.len() / i {
                reg.push_str(r"(\d{");
                reg.push_str(i.to_string().as_str());
                reg.push_str("})");
            }
            reg.push('$');
            // dbg!(&input);
            let re = Regex::new(&reg).unwrap();
            let cap = re.captures(number.as_str()).unwrap();

            let mut f: usize = 2;
            let cap0 = &cap[1];
            loop {
                let bel = &cap.get(f);
                if bel.is_some() {
                    if cap0 == bel.unwrap().as_str() {
                        f += 1;
                    } else {
                        is_the_same = false;
                        break;
                    }
                } else {
                    is_the_same = true;
                    break;
                }
            }
        }
        if is_the_same {
            break;
        }
    }
    //check for two in an Row
    if is_the_same {
        dbg!("added");
        input
    } else {
        0
    }
}

fn _validate_3(input: u128) -> u128 {
    let input_str = input.to_string();
    let mut is_same = true;
    // dbg!(&input);

    for i in 1..=input_str.len() {
        if input_str.len() % i == 0 {
            let vec: Vec<char> = input_str.chars().collect();
            let vec_chunks: Vec<Vec<char>> = vec.chunks(i).map(|chunk| chunk.to_vec()).collect();
            let mut vec_compare: Vec<String> = Vec::new();
            for k in &vec_chunks {
                vec_compare.push(k.iter().map(|f| f.to_string()).collect::<String>());
            }
            for j in 1..vec_compare.len() {
                if vec_compare[0] != vec_compare[j] {
                    is_same = false;
                    break;
                } else {
                    is_same = true;
                }
            }
        }
        if is_same {
            break;
        }
    }
    if is_same {
        dbg!("added");
        return input;
    }
    0
}

fn is_repetition(s: &str) -> bool {
    (1..=s.len() / 2)
        .any(|n| s.len() % n == 0 && s.as_bytes().chunks(n).all(|c| c == &s.as_bytes()[..n]))
}

#[cfg(test)]
mod tests {
    use crate::{_validate_2, _validate_3};

    #[test]
    fn check_validate2() {
        assert_eq!(_validate_2(565656), 565656);
        assert_eq!(_validate_2(100), 0);
    }

    #[test]
    fn check_validate3() {
        assert_eq!(_validate_3(565656), 565656);
        assert_eq!(_validate_3(100), 0);
    }
}
