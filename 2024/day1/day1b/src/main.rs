#![allow(non_snake_case)]
use std::fs;


fn main() {


    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let mut leftVec : Vec<i32> = Vec::new();
    let mut rightVec : Vec<i32> = Vec::new();
    //iterate line by line
    let line_contents = contents.lines();
    for line in line_contents{
        let fixedLine = line.replace("   "," ");
        let (left,right) = fixedLine.split_once(" ").expect("Split once failed");
        leftVec.push(left.parse::<i32>().expect("Left not a number"));
        rightVec.push(right.parse::<i32>().expect("Right not a number"));
    }

    leftVec.sort();
    rightVec.sort();
    
    let mut similarityScore = 0;
    for x in leftVec.iter() {
        let mut count = 0;
        for y in rightVec.iter(){
            if x == y {
                count += 1;
            }
        }
        similarityScore += x * count;
    }



    println!("Similarity score: {similarityScore}");
}
