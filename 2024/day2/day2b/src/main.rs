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
        let mut offset = 0;
        
        for i in 1..report.len()
        {
//            println!{"Element: {}  {}", i, i-1-offset};
            let currentDelta = report[i] - report[i-1-offset];
            
            if currentDelta.abs() > 3 {
                safe = false; 
            } else if currentDelta == 0 {
                safe = false; 
            } else if prevDelta < 0 && currentDelta > 0 {
                safe = false; 
            } else if prevDelta > 0 && currentDelta < 0 {
                safe = false; 
            } else {
                prevDelta = currentDelta;
            }
            
            if ! safe && offset == 0 && i > 1 {
                safe = true;
                offset = 1;
            }            

        }

        if safe {
            safeCount += 1;
        } 
    
    }
    println!("Safecount: {}", safeCount);

}
