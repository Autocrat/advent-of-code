#![allow(non_snake_case)]
use std::fs;
use std::collections::HashMap;

fn applyRules(input : &i64, iterations: &i64, cache : &mut HashMap<(i64,i64),i64> )-> i64 {
    if let Some(cachedValue) = cache.get(&(*input,*iterations)) {
        return *cachedValue;
    }

    let mut numStones = 1;
    let mut stoneValue = input.clone();
    for i in 0..*iterations {
        //if stone is a 0, it becomes a 1
        if stoneValue == 0 {
            stoneValue = 1;
        } else {
            let remainingIterations = iterations -i -1;
            let mut count = 1;
            let mut value = stoneValue.clone();
            while value > 9 {
                count += 1;
                value /= 10;
            }
            if count % 2 == 0 { //even number of digits
                let exponent = count/2;
                let left = stoneValue /10_i64.pow(exponent);
                let right = stoneValue - left * 10_i64.pow(exponent);
                stoneValue = left;
                numStones += applyRules(&right,&remainingIterations, cache);
            } else {
                stoneValue *= 2024;
            }
        }
    }
    cache.insert((*input,*iterations),numStones);
    return numStones as i64;
}


fn main() {
    let mut myHashMap : HashMap<(i64,i64),i64> = HashMap::new();

    //read strings from a file
    let contents = fs::read_to_string("input.txt")
    .expect("Yo! could not read file");

    let linesOfStrings = contents.lines();
    let mut rockCount = 0;
    for lines in linesOfStrings {
        let rocks : Vec<i64> = lines.split(' ').map(|x| x.parse::<i64>().expect("Bad input") ).collect();
 
        let numIterations = 75;
        for stone in rocks.iter() {
            rockCount += applyRules(&stone, &numIterations, &mut myHashMap);
        }
    }
    println!("Num rocks: {}", rockCount); 
   
}