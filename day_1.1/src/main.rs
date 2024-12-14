use csv::ReaderBuilder;
use std::error::Error;

fn read_csv_to_lists(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let value1: i32 = record[0].parse()?;
        let value2: i32 = record[1].parse()?;

        list1.push(value1);
        list2.push(value2);
    }
    Ok((list1, list2))
}

fn main() {
    match read_csv_to_lists("/home/rochen/Documents/advent_of_code_2024/day_1/input/in.txt") {
        Ok((list1, list2)) => {
            let mut result: Vec<i32> = Vec::new();
            let mut counter: i32 = 0;
            for left in list1 {
                //check if how often value of list1 at index j appears in list2
                for right in &list2 {
                    if left == *right {
                        counter = counter + 1;
                    }
                }
                result.push(left * counter);
                counter = 0;
            }
            println!("{:?}", result.iter().sum::<i32>())
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
