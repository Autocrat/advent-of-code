#![allow(non_snake_case)]
use std::fs;

//For this exersize, we only care about the entries that don't initially fit the rules
//so the approach is:
//check the values from the data against the rules that we read in.
//if it fits the rules as it, do nothing.
//if it doesn't, we get the first 2 pieces of data to swap.  Swap them, then run through the
//check/swap check/swap until there are not more violations

//As much as I would like to, I can't find an easy way to do swaps here because of references
//and borrowing and weird Rust memory management
//return a tuple with the indices that need swapping wrapped in an Option.  Or None if it's done
fn checkRulesAgainstData( rulesVector : &Vec<(&str,&str)>, data : &Vec<&str>) -> Option<(usize,usize)>
{
    for i in 0..data.len() {
        for rule in rulesVector {
            if data[i] == rule.1 {
                //check for preceding data
                for j in 0..data.len() {
                    if data[j] == rule.0 {
                        if j > i {
                            return Some((i,j));
                        }
                    }
                }
            }
        }
    }
    return None;    
}



fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    //holds the final value
    let mut total = 0;

    //Read the file
    let mut rulesVec = Vec::new();
    for line in contents.lines() {
        //skip blank lines
        if line != "" {
            //look for successful splits on '|' to find "rules"
            //it won't have one if it's not a rule
            //put all the data into vector of tuples 

            match line.split_once("|") {
                Some (result) => rulesVec.push(result),
                None => (),
            }
            
            let mut dataVector : Vec<_> = line.split(",").collect();

            if dataVector.len() > 1 {                
                if let Some((i,j)) = checkRulesAgainstData(&rulesVec,&dataVector) {
                    dataVector.swap(i,j);
                    while let Some((x,y)) = checkRulesAgainstData(&rulesVec,&dataVector) {
                        dataVector.swap(x,y);
                    }
                    let value = dataVector[dataVector.len()/2].parse::<i32>().expect("Not a number");
                    total += value;                   
                }               
            }
        }        
    }

    println!("{}",total);
}
