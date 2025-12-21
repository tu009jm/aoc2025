use std::fs;

fn main() {
    _auf_2();
}

fn fetch_data() -> Vec<Vec<u8>> {
    let data_string =
        fs::read_to_string("/Users/jannesmuller/Developer/rust/aoc2025/day3/input.txt").unwrap();
    let mut vec_data: Vec<Vec<u8>> = Vec::new();
    for lines in data_string.lines() {
        vec_data.push(
            lines
                .chars()
                .map(|f| f.to_string().parse::<u8>().unwrap())
                .collect(),
        );
    }
    vec_data
}

fn _auf_1() {
    let data = fetch_data();
    let mut counter = 0;

    for i in 0..data.len() {
        let (mut first, mut pos) = (0, 0);
        let mut uebergabe = 0;
        for j in 0..data[i].len() {
            if &data[i].get(j).unwrap() > &&first {
                (first, pos) = (**&data[i].get(j).unwrap(), j);
                uebergabe = pos;
            }
        }

        let mut second = 0;

        if uebergabe == data[i].len() - 1 {
            for k in 0..data[i].len() - 1 {
                if &data[i].get(k).unwrap() > &&second {
                    second = *data[i].get(k).unwrap();
                }
            }
            //switch first and second
            (first, second) = (second, first);
        } else {
            for k in uebergabe + 1..data[i].len() {
                if &data[i].get(k).unwrap() > &&second {
                    second = *data[i].get(k).unwrap();
                }
            }
        }
        // dbg!(first, second);
        let name: String = first.to_string() + &second.to_string();
        counter += name.parse::<u64>().unwrap();
    }
    dbg!(counter);
}

fn _auf_2() {
    let data = fetch_data();
    let mut counter: u128 = 0;

    for i in &data {
        let mut rounds = data[0].len() - 12;
        let curr_vec = i.clone();
        let mut start = 0;
        let mut finish_vec: Vec<u8> = Vec::new();
        let mut pos = 0;
        let mut temp_max = 0;
        for _ in 0..=11 {
            for j in start..=rounds {
                if *curr_vec.get(j).unwrap() > temp_max {
                    (temp_max, pos) = (*curr_vec.get(j).unwrap(), j);
                }
            }
            finish_vec.push(temp_max);
            temp_max = 0;
            start = pos + 1;
            rounds += 1;
        }
        // dbg!(finish_vec);
        let new_number: String = finish_vec
            .into_iter()
            .map(|f: u8| f.to_string())
            .collect::<String>();
        // dbg!(&new_number);
        counter += new_number.parse::<u128>().unwrap();
    }
    dbg!(counter);
}
