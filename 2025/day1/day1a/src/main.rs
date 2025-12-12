use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines_of_strings = contents.lines();

    let mut position : i32 = 50;
    let mut zero_count : i32 = 0;

    for lines in lines_of_strings {
        let (direction,amount_str) = lines.split_at(1);
        let amount = amount_str.parse::<i32>().expect("Amount isn't a number");
        //let cycle_count : i32 = amount/100; //uncomment for part 2
        let remainder = amount % 100;
        //zero_count += cycle_count; //uncomment for part 2

        if direction.parse::<char>().expect("Expected a letter") == 'R' {
            if (position + remainder) > 100 {
                //zero_count += 1; //uncomment for part 2
            }
            position += remainder;
        }else {
            if (position > 0) && ( (position - remainder) < 0) {
                //zero_count += 1; //uncomment for part 2
            }
            position -= remainder;
        }

        if position > 99 {
            position -= 100;
        }
        if position < 0 {
            position += 100;
        }
        if position == 0 {
            zero_count += 1;
        }
    }
    println!("{:?}", zero_count);
}
