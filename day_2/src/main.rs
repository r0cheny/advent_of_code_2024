use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_sorted_ascending<T: Ord>(list: &[T]) -> bool {
    list.windows(2).all(|w| w[0] <= w[1])
}

fn is_sorted_descinding<T: Ord>(list: &[T]) -> bool {
    list.windows(2).all(|w| w[0] >= w[1])
}

fn check_safety(list: Vec<i32>) -> bool {
    //check if list is sorted
    for (index, value) in list.iter().enumerate() {
        if index < list.len() - 1 {
            let delta = (*value - list[index + 1]).abs();
            if delta == 0 || delta > 4 {
                return false;
            }
        }
    }
    true
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rochen/Documents/advent_of_code_2024/day_2/input/in.tx")?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if is_sorted_ascending(&numbers) || is_sorted_descinding(&numbers) {
            if check_safety(numbers.clone()) {
                count = count + 1;
            }
        }
    }
    println!("{:}", count);
    Ok(())
}
