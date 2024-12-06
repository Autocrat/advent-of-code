#![allow(non_snake_case)]
use std::fs;

fn isIncreasing(report : &Vec<i32>) -> bool
{
    for i in 1..report.len()
    {
        if report[i] - report[i-1] <= 0 {
            return false;
        }
    }
    return true;
}

fn isDecreasing(report : &Vec<i32>) -> bool
{
    for i in 1..report.len()
    {
        if report[i] - report[i-1] >= 0 {
            return false;
        }
    }
    return true;
}

fn isCloseEnough(report : &Vec<i32>) -> bool
{
    for i in 1..report.len()
    {
        if (report[i] - report[i-1]).abs() > 3 {
            return false;
        }
    }
    return true;
}

fn isSafe(report : &Vec<i32>) -> bool
{
    if (isIncreasing(report) || isDecreasing(report)) && isCloseEnough(report)
    {
        return true;
    }
    return false;
}

fn main() {


    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let mut safeCount = 0;
    
    //iterate line by line
    let line_contents = contents.lines();
    for line in line_contents{
        let report : Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().expect("Not a number")).collect();


        if isSafe(&report) {
            safeCount += 1;
        } 
        else {
            for i in 0..report.len()
            {
                let mut reportCopy = report.clone();
                reportCopy.remove(i);
                if isSafe(&reportCopy){
                    safeCount += 1;
                    break;
                }
            }
        }


    }
    println!("Safecount: {}", safeCount);

}
