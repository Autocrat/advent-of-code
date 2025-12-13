use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines = contents.lines();
    let mut data : Vec <Vec<char>> = Vec::new();

    for text in lines {
        let digits : Vec<_> = text.chars().collect();
        data.push(digits);
    }

    let mut total : i64 = 0;
    let num_rows = data.len();
    let last_element_index = data[0].len();
    let mut column_values_vec : Vec<i64> = Vec::new();
    for j in (0..last_element_index).rev() {
        let mut num_string = String::new();
        let mut symbol : char = ' ';
        for i in 0..num_rows {
            let &character = &data[i][j];
            if character.is_digit(10) {
                num_string.push(character);
            } else if character == '*' || character == '+' {
                symbol = character.clone();
            }
        }
        if let Ok(value) = num_string.parse::<i64>() {
            column_values_vec.push(value);
        }

        if symbol == '*' || symbol == '+' {
            if symbol == '+' {
                let local_total : i64 = column_values_vec.iter().sum();
                total += local_total;
            } else if symbol == '*' {
                let mut local_total : i64 = 1;
                column_values_vec.iter().for_each(|x| local_total *= x);
                total += local_total;
            }
            column_values_vec.clear();
        }
    }
    println!("{}",total);
}
