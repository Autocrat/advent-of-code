use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines_of_strings = contents.lines();

    let mut total : i64 = 0;
    for lines in lines_of_strings {
        //split on comma to get ranges
        let ranges : Vec<_> = lines.split(',').collect();

        for range in ranges {
            //split on - to get start and end ranges
            if let Some((start,end)) = range.split_once('-') {
                let start_val : i64 = start.parse::<i64>().expect("Start is not a valid number");
                let end_val : i64 = end.parse::<i64>().expect("End is not a valid number");
                for value in start_val..=end_val {
                    let value_str : String = value.to_string();
                    let value_str_len = value_str.len();
                    //if the string has an even length, there's a possibility
                    if (value_str_len % 2 ) == 0 {
                        let half_string_length = value_str_len /2 ;
                        let (left,right) = value_str.split_at(half_string_length);
                        if left == right {
                            total += value;
                        }
                    }
                }
            } else {
                println!("Error splitting {}", range);
            }
        }
    }
    println!("{}", total);
 }