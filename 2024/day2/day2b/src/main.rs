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
        let mut bustedOnce = false;

        for i in 1..report.len()
        {
//            println!{"Element: {}  {}", i, i-1-offset};
            let currentDelta = report[i] - report[i-1-offset];
            
            if currentDelta.abs() > 3 {
                println!("Fail at too far");
                safe = false; 
            } else if currentDelta == 0 {
                println!("Fail at too close");
                safe = false; 
            } else if prevDelta < 0 && currentDelta > 0 {
                println!{"Fail at deltas 1: {}   {}", prevDelta, currentDelta};
                  if i == 2
                  { prevDelta = 0;}
                safe = false; 
            } else if prevDelta > 0 && currentDelta < 0 {
                println!{"Fail at deltas 2: {}   {}", prevDelta, currentDelta};
                  if i == 2
                  { prevDelta = 0;}
                safe = false; 
            } else {
                prevDelta = currentDelta;
                offset = 0;
            }

            if !safe && bustedOnce == false {
                safe = true;
                bustedOnce = true;

                if i > 2 {
                    offset = 1;
                    println!("Setting offset");
                }
            }
        }

        if safe {
            safeCount += 1;
            println!{"{:?}", report};
} 
        else {
        }    
    }
    println!("Safecount: {}", safeCount);

}
