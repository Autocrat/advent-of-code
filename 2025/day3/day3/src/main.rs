use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines_of_strings = contents.lines();

    let mut total : i64 = 0;
    let num_digits : usize = 12; //change to 2 for part 1, 12 for part 2
    
    for lines in lines_of_strings {
        let digits : Vec <_> = lines.matches(char::is_numeric).map(|x| x.parse::<i64>().expect("Found non digit")).collect();
        
        let mut digit_index : usize = 0;
        let mut joltage : i64 = 0;

        //have to find all the digits
        for index in 0..num_digits {
            let mut digits_joltage : i64 = 0;
            
            let start = if index > 0 { joltage *= 10; digit_index + 1 } else {0};
            let end = digits.len() - (num_digits - 1 - index);

            for i in start..end {
                if digits[i] > digits_joltage {
                    digits_joltage = digits[i];
                    digit_index = i;
                    if digits_joltage == 9 {
                        break;
                    }
                }
            }
            joltage += digits_joltage;
        }
        total += joltage;
    }
    println!("{}", total);
 }