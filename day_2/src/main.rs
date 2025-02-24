use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn check_safety(list: Vec<i32>) -> bool {
    //check if list is sorted
    list.windows(2)
        .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
}

fn check_safety_reverse(list: Vec<i32>) -> bool {
    list.windows(2)
        .rev()
        .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
}

fn check_delta(a: i32, b: i32) -> bool {
    if (a - b == 0 || a - b > 3) || (b - a == 0 || b - a > 3) {
        return true;
    }
    false
}

fn check_loop(list: Vec<i32>) -> bool {
    for (index, _) in list.iter().enumerate() {
        if index < list.len() - 1 {
            if check_delta(list[index], list[index + 1]) {
                let mut new_list = list.clone();
                new_list.remove(index);
                if check_safety(new_list.clone()) || check_safety_reverse(new_list.clone()) {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
    true
}

fn check(list: Vec<i32>) -> bool {
    if check_safety(list.clone()) || check_safety_reverse(list.clone()) {
        return true;
    } else {
        return check_loop(list.clone());
    }
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
        if check(numbers.clone()) {
            count = count + 1;
        } else {
            println!("{:?}", numbers)
        }
    }

    println!("{:}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_ascending() {
        let input = vec![48, 50, 51, 53, 55, 56, 59, 58];
        let result = is_sorted_ascending(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_sorted_descending() {
        let input = vec![48, 50, 51, 53, 55, 56, 59, 58];
        let result = is_sorted_descinding(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_safety_true() {
        let input = vec![7, 6, 4, 2, 1];
        //let a = vec![27, 31, 28, 31, 32];
        //assert_eq!(check_safety(a), true);
        assert_eq!(check_safety(input), true);
    }

    #[test]
    fn test_check_safety_false() {
        let input = vec![9, 7, 6, 2, 1];
        let a = vec![48, 50, 50, 51, 51];
        assert_eq!(check_safety(input), false);
        assert_eq!(check_safety(a), false)
    }

    #[test]
    fn test_check_is_sorted_ascending() {
        let input = vec![19, 21, 24, 27, 24];
        let result = is_sorted_ascending(&input);
        assert_eq!(result, false);
    }
}
