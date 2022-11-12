use std::{io, collections::HashMap};

fn main() {

    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("Numbers could not be read");
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mean = mean_fnc(&numbers);
    println!("Mean is {}", mean);

    let median = median_fnc(&numbers);
    println!("Median is {}", median);

    let mode = mode_fnc(&numbers);
    println!("Mode is {:?}", mode);
}

fn mean_fnc(vect: &Vec<i32>) -> f64 {
    let mut total = 0;
    for value in vect {
        total += value
    };
    let total = total as f64;
    total / vect.len() as f64
}

fn median_fnc(vect: &Vec<i32>) -> f64 {
    let mut copy = vect.clone();
    copy.sort();
    let size = copy.len();
    if size % 2 == 0 {
        let sum = copy[size / 2] + copy[(size / 2) - 1];
        return sum as f64 / 2.0
    }
    copy[size / 2] as f64
}

fn mode_fnc(vect: &Vec<i32>) -> Vec<i32> {
    let mut counts = HashMap::new();
    for &value in vect {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
    }

    let top = counts.values().max().unwrap();
    let mut modes: Vec<i32> = Vec::new();
    for (k, v) in counts.iter() {
        if v == top {
            modes.push(*k);
        }
    }
    return modes

}