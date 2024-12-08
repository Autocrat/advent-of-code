#![allow(non_snake_case)]
use std::fs;

fn checkRulesAgainstData( rulesVector : &Vec<(&str,&str)>, data : &Vec<&str>) -> Option<i32>
{
    for i in 0..data.len() {
        for rule in rulesVector {
            if data[i] == rule.1 {
                //check for preceding data
                for j in 0..data.len() {
                    if data[j] == rule.0 {
                        if j > i {
                            return None;
                        }
                    }
                }
            }
        }
    }
    return Some(data[data.len()/2].parse::<i32>().expect("Not a number, dammit"));
    
}

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let mut total = 0;
        
    let mut rulesVec = Vec::new();
    for line in contents.lines() {
        if line != "" {
            match line.split_once("|") {
                Some (result) => rulesVec.push(result),
                None => (),
            }
            let dataVector : Vec<_> = line.split(",").collect();
            if dataVector.len() > 1 {
                match checkRulesAgainstData(&rulesVec,&dataVector){
                    Some(result) => total += result,
                    None => (),
                }
                
            }
        }        
    }

    println!("{}", total);

}
