#![allow(non_snake_case)]
use std::fs;


fn main() {


    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let mut safeCount = 0;
    
    //iterate line by line
    let line_contents = contents.lines();
    for line in line_contents{
        let report : Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().expect("Not a number")).collect();
        let mut prevDelta = 0;
        let mut safe = true;
        for i in 1..report.len()
        {
            let newDelta = report[i] - report[i-1];
            if newDelta.abs() > 3 {
                safe = false; 
            } else if newDelta.abs() < 1{
                safe = false; 
            } else if prevDelta < 0 && newDelta > 0{
                safe = false; 
            } else if prevDelta > 0 && newDelta < 0 {
                safe = false; 
            } else {
                prevDelta = newDelta;
            }
        }

        if safe {
            safeCount += 1;
        }
    }
    println!("Safecount: {}", safeCount);

}
