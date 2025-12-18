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
            if false && is_repetition(d.to_string().as_str()) {
                counter += d;
            }
            counter += validate_2(d);
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

fn validate_2(input: u128) -> u128 {
    // check for one in an Row
    let number = input.to_string();
    // let number_vec: Vec<char> = number.chars().collect();
    let mut is_the_same = true;
    // for f in 0..number_vec.len() - 1 {
    //     if number_vec.get(0) != number_vec.get(f) {
    //         is_the_same = false;
    //     }
    // }
    // if is_the_same {
    //     return input;
    // }
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
fn is_repetition(s: &str) -> bool {
    (1..=s.len() / 2)
        .any(|n| s.len() % n == 0 && s.as_bytes().chunks(n).all(|c| c == &s.as_bytes()[..n]))
}
#[cfg(test)]
mod tests {
    use crate::validate_2;

    #[test]
    fn check_validate2() {
        assert_eq!(validate_2(565656), 565656);
        assert_eq!(validate_2(100), 0);
    }
}
