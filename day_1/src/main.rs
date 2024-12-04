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

fn sum(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}
fn main() {
    match read_csv_to_lists("/home/rochen/Documents/advent_of_code_2024/day_1/input/in.txt") {
        Ok((mut list1, mut list2)) => {
            let mut result: Vec<i32> = Vec::new();
            list1.sort();
            list2.sort();
            for (index, _) in list1.iter().enumerate() {
                if list1[index] > list2[index] {
                    result.push(list1[index] - list2[index]);
                } else {
                    result.push(list2[index] - list1[index]);
                }
            }
            println!("{:?}", sum(&result))
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
