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
                    let max_pattern_length = value_str_len/2; //can't have a pattern longer than half string
                    //loop down from max patter length, grabbing slices of size 1.  Run matches() and look at the sum of the resultant string sizes to see if it matches the total length
                    for pattern_length in (1..=max_pattern_length).rev() {               
                        if value_str_len % pattern_length  == 0 {
                            if let Some(pattern) = value_str.get(0..pattern_length) {
                                let match_string_length = value_str.matches(pattern).collect::<Vec<_>>().join("").len();
                                if match_string_length == value_str_len {
                                    total += value;
                                    break;
                                }
                            } else {
                                println!("Unable to get substring");
                            }

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