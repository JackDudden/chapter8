use std::io;

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
}

fn mean_fnc(vect: &Vec<i32>) -> f64 {
    let mut total = 0;
    for value in vect {
        total += value
    };
    let total = total as f64;
    total / vect.len() as f64
}