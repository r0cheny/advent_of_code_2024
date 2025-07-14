fn check_row(input_string: &str) -> i32 {
    let target_forward = "XMAS".as_bytes();
    let target_backward = "SAMX".as_bytes();
    let bytes = input_string.as_bytes();
    bytes
        .windows(target_forward.len())
        .filter(|w| *w == target_forward || *w == target_backward)
        .count() as i32
}

fn get_column(matrix: &[&str], col_index: usize) -> String {
    matrix
        .iter()
        .filter_map(|row| row.chars().nth(col_index))
        .collect()
}

fn check_diagonal(matrix: &[&str]) -> i32 {
    let pattern = ['M', 'A', 'S'];
    let pattern_backward = ['A', 'M', 'X'];
    let mut count = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, col) in row.chars().enumerate() {
            if col == 'X' {
                if row_index + pattern.len() <= matrix.len()
                    && col_index + pattern.len() <= row.len()
                {
                    if pattern
                        .iter()
                        .enumerate()
                        .all(|(i, &s)| matrix[row_index + i].chars().nth(col_index + i) == Some(s))
                    {
                        count = count + 1;
                    }
                }
            } else if col == 'S' {
                if row_index + pattern.len() <= matrix.len()
                    && col_index + pattern.len() <= row.len()
                {
                    if pattern_backward
                        .iter()
                        .enumerate()
                        .all(|(i, &s)| matrix[row_index + i].chars().nth(col_index + i) == Some(s))
                    {
                        count = count + 1;
                    }
                }
            }
        }
    }
    count
}

fn main() {
    let sample = vec![
        "MMMSXXMASM",
        "MSAXXMSMSA",
        "AMXSMMAAMM",
        "MSAMAAMSMX",
        "XMASAMSAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMAA",
        "MXMXAXMASX",
    ];
    //check each row
    let mut count = 0;
    for row in &sample[0..] {
        count = count + check_row(row);
    }
    //check columns - works only if all rows are equally long
    let row_len = sample[1].chars().count();
    for i in 0..row_len {
        let row = get_column(&sample, i);
        count = count + check_row(&row);
    }
    count = count + check_diagonal(&sample);
    println!("{:}", count);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_check_forward() {
        let input_string = String::from("SAMX is here, XMAS time is the best! xmasxmas");
        let result = check_row(input_string.as_str());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_check_column() {
        let sample = vec!["MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM"];
        let row_len = sample[1].chars().count();
        let mut count = 0;
        for i in 0..row_len {
            let row = get_column(&sample, i);
            count = count + check_row(&row);
        }
        assert_eq!(count, 1);
    }
}
