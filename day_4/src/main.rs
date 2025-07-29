use std::fs;
use std::io;

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
    let char_matrix: Vec<Vec<char>> = matrix.iter().map(|row| row.chars().collect()).collect();
    let rows = char_matrix.len();
    let cols = if rows > 0 { char_matrix[0].len() } else { 0 };
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            //Down-Right
            if row + 3 < rows && col + 3 < cols {
                let word: String = (0..4).map(|i| char_matrix[row + i][col + i]).collect();
                if word == "XMAS" {
                    count += 1;
                }
            }
            //Down-Left
            if row + 3 < rows && col >= 3 {
                let word: String = (0..4).map(|i| char_matrix[row + i][col - i]).collect();
                if word == "XMAS" {
                    count += 1;
                }
            }
            //Up-Right
            if row >= 3 && col + 3 < cols {
                let word: String = (0..4).map(|i| char_matrix[row - i][col + i]).collect();
                if word == "XMAS" {
                    count += 1;
                }
            }
            //Up-Left
            if row >= 3 && col >= 3 {
                let word: String = (0..4).map(|i| char_matrix[row - i][col - i]).collect();
                if word == "XMAS" {
                    count += 1;
                }
            }
        }
    }
    count
}

fn read_file_to_vec(filename: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(filename)?;
    let lines = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect();
    Ok(lines)
}

fn main() {
    let sample =
        read_file_to_vec("/home/rochen/Documents/advent_of_code_2024/day_4/data/input.txt")
            .unwrap();
    let sample_refs: Vec<&str> = sample.iter().map(|s| s.as_str()).collect();
    //check each row
    let mut count = 0;
    for row in &sample_refs[0..] {
        count = count + check_row(row);
        //let reversed: String = row.chars().rev().collect();
        //count = count + check_row(&reversed);
    }
    let row_len = sample_refs[0].chars().count();
    for i in 0..row_len {
        let col = get_column(&sample_refs, i);
        count = count + check_row(&col);
        //let reversed: String = col.chars().rev().collect();
        //count = count + check_row(&reversed);
    }
    count = count + check_diagonal(&sample_refs);
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
    #[test]
    fn test_diaganol_search() {
        let sample = vec!["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX"];
        let result = check_diagonal(&sample);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_sample() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        let diag_result = check_diagonal(&input);
        let row_len = input[1].chars().count();
        let mut col_result = 0;
        for i in 0..row_len {
            let row = get_column(&input, i);
            col_result = col_result + check_row(&row);
        }
        let mut row_result = 0;
        for i in 0..row_len {
            row_result = row_result + check_row(input[i]);
        }
        println!(
            "Single results: {}, {}, {}",
            diag_result, col_result, row_result
        );
        let result = diag_result + col_result + row_result;
        println!("Final Result: {}", result);
        assert_eq!(result, 18);
    }
    #[test]
    fn test_row_forward() {
        let input = vec!["XMAS", "....", "....", "...."];
        let row_len = input[1].chars().count();
        let mut result = 0;
        for i in 0..row_len {
            result = result + check_row(input[i]);
        }
        assert_eq!(result, 1);
    }
    #[test]
    fn test_row_backward() {
        let input = vec!["SAMX", "....", "....", "...."];
        let row_len = input[1].chars().count();
        let mut result = 0;
        for i in 0..row_len {
            result = result + check_row(input[i]);
        }
        assert_eq!(result, 1);
    }
    #[test]
    fn test_col_forward() {
        let input = vec!["X...", "M...", "A...", "S..."];
        let row_len = input[1].chars().count();
        let mut result = 0;
        for i in 0..row_len {
            let row = get_column(&input, i);
            result = result + check_row(&row);
        }
        assert_eq!(result, 1);
    }
    #[test]
    fn test_col_backward() {
        let input = vec!["S...", "A...", "M...", "X..."];
        let row_len = input[1].chars().count();
        let mut result = 0;
        for i in 0..row_len {
            let row = get_column(&input, i);
            result = result + check_row(&row);
        }
        assert_eq!(result, 1);
    }
    #[test]
    fn test_diag_forward() {
        let input = vec!["X...", ".M..", "..A.", "...S"];
        let result = check_diagonal(&input);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_diag_backward() {
        let input = vec!["...S", "..A.", ".M..", "X..."];
        let result = check_diagonal(&input);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_diag_forward1() {
        let input = vec!["...X", "..M.", ".A..", "S..."];
        let result = check_diagonal(&input);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_diag_backward1() {
        let input = vec!["S...", ".A..", "..M.", "...X"];
        let result = check_diagonal(&input);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_edgecase_diag() {
        let input = vec!["S..S", ".AA.", ".MM.", "X..X"];
        let result = check_diagonal(&input);
        assert_eq!(result, 2);
    }
}
