#![allow(non_snake_case)]
use std::fs;
use regex::Regex;

fn main() {


    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

//    let re = Regex::new(r"mul([0-9]+,[0-9]+)");
let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
let reOperands = Regex::new(r"\d+,\d+").unwrap();

    let mut total = 0;
    //iterate line by line
    let line_contents = contents.lines();
    for line in line_contents{
        let matches : Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        for i in matches {
            let operandVec : Vec<_> = reOperands.find_iter(i).map(|m| m.as_str()).collect();
            for  factors in operandVec {
                let (left,right) = factors.split_once(",").expect("Where is the comma?");
                total += left.parse::<i32>().expect("Left is not valid") * 
                         right.parse::<i32>().expect("right is not valid");

            } 
        }
    }
    println!("Total multiplied: {total}");
}
