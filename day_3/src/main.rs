use regex::Regex;
use std::fs;

fn find_matches(instructions: String) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(don't)|(do)").unwrap();
    return re
        .find_iter(&instructions.to_string())
        .map(|m| m.as_str().to_string())
        .collect();
}

fn summation(instructions: Vec<String>) -> u32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let mut result: u32 = 0;
    let mut mul_flag: bool = true;
    for instruction in instructions {
        println!("{:}", instruction);
        //only multiply if the flag is true
        //set the flag false if the current instruction == dont
        //set the flag true if the current instruction is do
        if instruction != "don't" && instruction != "do" && mul_flag == true {
            let mul: u32 = re
                .find_iter(&instruction.to_string())
                .filter_map(|m| m.as_str().parse::<u32>().ok())
                .product();
            result = result + mul;
            println!("{:}", result);
        } else if instruction == "don't" {
            mul_flag = false;
        } else if instruction == "do" {
            mul_flag = true;
        }
    }
    return result;
}

fn main() {
    let filename = "/home/rochen/Documents/advent_of_code_2024/day_3/input/data.txt";
    let instructions = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading File: {}", err);
            return;
        }
    };
    let decoded_instructions = find_matches(instructions);
    let result = summation(decoded_instructions);
    println!("{:}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_groups() {
        //should return a single string element for each match
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let result = find_matches(input);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_summation() {
        let input = find_matches(String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        ));
        let result = summation(input);
        println!("{:?}", result);
        assert_eq!(result, 48);
    }
}
