
#![allow(non_snake_case)]
use std::fs;

//convert a number to a trinary representation in a vector of unsigned integers, with the. . tribits? in reverse 
//order
fn format_trinary(mut x: usize) -> Vec<usize> {
    let mut result = vec![];

    loop {
        let m : usize = x % 3;
        x = x / 3;
        result.push(m);
        if x == 0 {
            break;
        }
    }
    return result;
}


fn main() {

    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let mut totalValue = 0;

    //iterate line by line
    let lineContents = contents.lines();
    for line in lineContents{
        let Some((targetValueString, operands))= line.split_once(':') else { todo!(); };
        let operands : Vec<i64> = operands.trim().split_whitespace().map(|x| x.parse::<i64>().expect("not a number")).collect();

        let targetValue = targetValueString.parse::<i64>().expect("Target value should be an i64");

        let temp : u32 = 3;
        let temp2: u32 = operands.len() as u32;
        let foo = temp.pow(temp2);
        let maxPossibilities = foo as usize; //operands.len();

        for i in 0..(maxPossibilities) {
            let mut resultStringVector = vec![operands[0].to_string()];
            let operators = format_trinary(i);
            let mut value : i64 = operands[0];
            for operandIndex in 1..(operands.len()) {
                if operandIndex > operators.len() {
                    value += operands[operandIndex];
                    resultStringVector.push("+".to_string());
                } else if operators[operandIndex-1] == 0 {
                    value += operands[operandIndex];
                    resultStringVector.push("+".to_string());                
                } else if operators[operandIndex-1] == 1 {
                    value *= operands[operandIndex];
                    resultStringVector.push("*".to_string());                
                } else {
                    //do the concatenation thing
                    let tempStringValue = value.to_string() + &operands[operandIndex].to_string();
                    value = tempStringValue.parse::<i64>().expect("Better be a number");
                }
                resultStringVector.push(operands[operandIndex].to_string());
            }

            if value == targetValue {
                totalValue += targetValue;
                break;
            }
        }
    }
    println!("Total: {}", totalValue);
}