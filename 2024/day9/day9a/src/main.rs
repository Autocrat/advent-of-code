#![allow(non_snake_case)]
use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
    .expect("Yo! could not read file");
    let linesOfStrings = contents.lines();

    for lines in linesOfStrings {
        let mut x = 0;
        let mut id : i64 = 0;
        let mut decompressedIntVec : Vec<i64> = Vec::new();
        for character in lines.chars() {
            let length = character.to_digit(10).unwrap();
            let mut valueToAdd = -1;
            if (x%2) == 0 {
                valueToAdd = id;
                id += 1;
            }
            for _i in 0..length {
                decompressedIntVec.push(valueToAdd);
            }
            x +=1;
        }

        let mut endIndex = decompressedIntVec.len() - 1;
        for index in 0..decompressedIntVec.len() {
            if decompressedIntVec[index] == -1 {
                //grab a character from the end and swap them
                while decompressedIntVec[endIndex] == -1 {
                    endIndex -= 1;
                }
                if index >= endIndex {
                    break;
                }
                decompressedIntVec[index] = decompressedIntVec[endIndex];
                decompressedIntVec[endIndex]=-1;
                endIndex = decompressedIntVec.len() - 1;
            }
        }

        let mut total = 0;
        for (i,value) in decompressedIntVec.iter().enumerate() {
            if *value > -1 {
                total += i as i64 * value;
            }
        }
        println!("Total: {}", total);
    }
}