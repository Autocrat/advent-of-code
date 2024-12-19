#![allow(non_snake_case)]
use std::fs;

fn applyRules(input : &Vec<String>)->Vec<String> {
    let mut output : Vec<String> = Vec::new();
    for i in input.iter() {
        //if stone is a 0, it becomes a 1
        if i == "0" {
            output.push("1".to_string());
        } else if i.len()%2 == 0 {
            //if stone has an even number of digits, split it into 2 stones.  Remove leading 0's
            let splitpoint = i.len()/2;
            let mut left = i.clone();
            let right = left.split_off(splitpoint).parse::<i64>().expect("Bad split").to_string();
            output.push(left);
            output.push(right);
        } else {
            // multiply by 2024
            let value = (i.parse::<i64>().expect("Not a number") * 2024).to_string();
            output.push(value);
        }
    }
    return output;
}


fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
    .expect("Yo! could not read file");

    let linesOfStrings = contents.lines();

    for lines in linesOfStrings {
        let mut rocks : Vec<String> = lines.split(' ').map(|x| x.to_string() ).collect();

        let numIterations = 75;
        for _i in 0..numIterations {
            rocks = applyRules(&rocks);
            println!("Num rocks: {}", rocks.len());
        }
    }    
}