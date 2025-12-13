use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines = contents.lines();

    let mut rows : Vec <Vec<i64>> = Vec::new();
    let mut symbols_row : Vec<char> = Vec::new();

    for text in lines {
        let fields_iter = text.split_whitespace();
        let mut row : Vec<i64> = Vec::new();
        for i in fields_iter {
            if let Ok(number) = i.parse::<i64>() {
                row.push(number);
            } else {
                symbols_row.push(i.chars().nth(0).unwrap());
            }
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }

    let num_rows_of_numbers = rows.len();
    let num_columns = rows[0].len();
    let mut total : i64 = 0;
    for j in 0..num_columns {
        let mut column_total : i64 = rows[0][j];
        for i in 1..num_rows_of_numbers {
            if symbols_row[j] == '*' {
                column_total *= rows[i][j];
            }
            else if symbols_row[j] == '+' {
                column_total += rows[i][j];
            }
        }
        total += column_total;
    }

    println!("{}", total);
}