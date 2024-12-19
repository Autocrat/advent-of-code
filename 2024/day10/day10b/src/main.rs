#![allow(non_snake_case)]
use std::fs;
use std::collections::HashMap;


//recursively search for a path to the end of the trail
fn findNextInPath( map : &HashMap<(i32,i32),u32>, 
                   position : (i32,i32), 
                   value : u32) -> i32 {
    let mut numUniquePaths = 0;
    let nextValue = value + 1;
    let mut pathOptionsVector : Vec<(i32,i32)> = Vec::new();
    match map.get(&(position.0-1,position.1)) {
        Some(mapEntry) => { 
            if *mapEntry == nextValue {
                pathOptionsVector.push((position.0-1,position.1));
            }
        },
        None => (),
    }
    match map.get(&(position.0+1,position.1)) {
        Some(mapEntry) => { 
            if *mapEntry == nextValue {
                pathOptionsVector.push((position.0+1,position.1));
            }
        },        None => (),
    }
    match map.get(&(position.0,position.1+1)) {
        Some(mapEntry) => { 
            if *mapEntry == nextValue {
                pathOptionsVector.push((position.0,position.1+1))
            }
        },
        None => (),
    }
    match map.get(&(position.0,position.1-1)) {
        Some(mapEntry) => { 
            if *mapEntry == nextValue {
                pathOptionsVector.push((position.0,position.1-1))
            }
        },
        None => (),
    }
    
    let numOptions = pathOptionsVector.len();
    if numOptions > 0 {
        if nextValue == 9 {
            return numOptions as i32;
        } else {
            for i in pathOptionsVector.iter() {
                numUniquePaths += findNextInPath(map,*i,nextValue);
            }
        }
    }

    return numUniquePaths;
}


fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
    .expect("Yo! could not read file");

    let linesOfStrings = contents.lines();
    let mut y = 0;
    let mut x = 0;

    let mut mountainMap : HashMap<(i32,i32),u32> = HashMap::new();
    for lines in linesOfStrings {
        x = 0;
        for character in lines.chars() {
            let value : u32 = character.to_digit(10).unwrap();
            mountainMap.insert((x,y),value);
            x += 1;
        }
        y += 1;
    }

    let maxX = x;
    let maxY = y;

    let mut totalTrailHeadScore = 0;
    for j in 0..maxY {
        for i in 0..maxX {
            let value = mountainMap.get(&(i,j)).unwrap();
            //for every 0 in the grid, go hunting for the next part of the trail
            if *value == 0 {
                //find next in path
                let trailHeadScore = findNextInPath(&mountainMap, (i,j), 0);
                totalTrailHeadScore += trailHeadScore;
            }
        } 
    }
    println!("Total trailheadscore {}", totalTrailHeadScore);
}