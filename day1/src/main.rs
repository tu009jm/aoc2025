use std::fs;

fn main() {
    auf_2_2();
    println!("Variante 2");
    _auf_2();
}

fn fetch_data() -> Vec<String> {
    let data = fs::read_to_string("/Users/jannesmuller/Developer/rust/aoc2025/day1/input_test.txt")
        .unwrap();
    let data_vec: Vec<String> = data.lines().into_iter().map(|f| f.to_string()).collect();
    return data_vec;
}

fn auf_1() -> u32 {
    let data = fetch_data();
    let mut dial: i32 = 50;
    let mut counter = 0;
    let mut gez = false;
    for i in data {
        let (x, y) = i.split_at(1);
        if x == "R" {
            if dial + y.parse::<i32>().unwrap() > 99 {
                counter += 1;
                gez = true;
            }
            dial = (dial + y.parse::<i32>().unwrap()) % 100;
        } else {
            if dial - y.parse::<i32>().unwrap() < 0 {
                counter += 1;
                gez = true;
            }
            dial = dial - y.parse::<i32>().unwrap() % 100;
            if dial < 0 {
                dial = 100 + dial;
            }
        }
        if dial == 0 || dial == 100 {
            if gez == true {
                gez = false
            } else {
                counter += 1;
            }
        }
        // dbg!(dial);
    }
    counter as u32
}

fn _auf_2() {
    let data = fetch_data();
    let mut dial: i32 = 50;
    let mut counter: u32 = 0;
    let mut gez = false;
    for i in data {
        let (x, y) = i.split_at(1);
        if y.parse::<i32>().unwrap() > 99 {
            let yy96 = y.parse::<u32>().unwrap() % 100;
            let yyy4000 = y.parse::<u32>().unwrap() - yy96;
            let yyyy40 = yyy4000 / 100;
            counter += yyyy40;
            dbg!("yyy40=", yyyy40);
        }
        let yy96 = y.parse::<i32>().unwrap() % 100;

        if x == "R" {
            if dial + yy96 > 99 {
                counter += 1;
                dbg!("counter up on:", dial);
            }
            dial = (dial + yy96) % 100;
        } else {
            if dial - yy96 <= 0 {
                counter += 1;
                dbg!("counter up on:", dial);
            }
            dial = dial - yy96 % 100;
            if dial < 0 {
                dial = 100 + dial;
            }
        }
        // if dial == 0 || dial == 100 {
        //     if gez == true {
        //         gez = false
        //     } else {
        //         counter += 1;
        //     }
        // }
        // dbg!(counter);
    }
    dbg!(counter);
}

fn auf_2_2() {
    let data = fetch_data();
    let vec_r: Vec<u32> = (0..=99).collect();
    let mut vec_l: Vec<u32> = vec_r.clone();
    vec_l.reverse();
    let mut dial: u32 = 49;

    let mut counter = 0;
    for i in data {
        let mut r = vec_r.iter().cycle().skip(dial as usize);
        let mut l = vec_l.iter().cycle().skip(dial as usize);
        let (x, y) = i.split_at(1);
        if x == "R" {
            for _ in 0..y.parse::<u32>().unwrap() {
                if *r.next().unwrap() == 0 {
                    counter += 1;
                }
            }
            dial = *r.next().unwrap();
            if dial == 0 {
                dial = 99;
            } else {
                dial = dial - 1;
            }
        } else {
            for _ in 0..y.parse::<u32>().unwrap() {
                if *l.next().unwrap() == 0 {
                    counter += 1;
                }
            }
            dial = *l.next().unwrap();
            dial = dial + 1;
        }
    }
    dbg!(counter);
}
