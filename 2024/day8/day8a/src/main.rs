#![allow(non_snake_case)]
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn getDistanceToNext( first : (i32, i32), second : (i32,i32)) -> (i32,i32) {
    return ((second.0 - first.0).abs(), (second.1 - first.1).abs());
}

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
    .expect("Yo! could not read file");

    let mut antennaPositionsMap : HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    let mut setOfAntinodes : HashSet<(i32,i32)> = HashSet::new();

    let linesOfStrings = contents.lines();
    let mut y = 0;
    let mut x = 0;
     for lines in linesOfStrings {

        x = 0;
        for character in lines.chars() {
            if character.is_alphanumeric() {
                match antennaPositionsMap.get_mut(&character) {
                    Some(tempVector) => tempVector.push((x,y)),
                    None => {
                        let newVector = vec![(x,y)];
                        antennaPositionsMap.insert(character, newVector);
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }

    let maxX = x ;
    let maxY = y ;

    for (_key, value) in antennaPositionsMap.iter() {
        let length = value.len();
        for vectorIndex in 0..value.len()-1 {
            let mut nextVectorIndex = vectorIndex + 1;
            while nextVectorIndex != length {
                let (nodeX,nodeY) = &value[vectorIndex];
                let (nextX,nextY) = &value[nextVectorIndex];
                let (distX,distY) = getDistanceToNext( value[vectorIndex], value[nextVectorIndex]);
                 
                //Y of the next node will alway be greater than the Y of the current node
                //X we have to determine the direction for
                let mut antinode1X = nodeX + distX;
                if nodeX < nextX{
                    antinode1X = nodeX - distX;
                }
                let antinode1Y = nodeY - distY;

                let mut antinode2X = nextX - distX;
                if nodeX < nextX{
                    antinode2X = nextX + distX;
                }
                let antinode2Y = nextY + distY;
                //make sure the antinode is in the bounds of the map
                if antinode1X > -1 && antinode1X < maxX && antinode1Y > -1 && antinode1Y < maxY {
                    //is valid, can add this antinode to a HashSet
                    setOfAntinodes.insert((antinode1X,antinode1Y));
                }
                
                if antinode2X > -1 && antinode2X < maxX && antinode2Y > -1 && antinode2Y < maxY {
                    //is valid, can add this antinode to a HashSet
                    setOfAntinodes.insert((antinode2X,antinode2Y));
                }
                

                nextVectorIndex += 1;

            }
        }
        
    }
    println!("{}", setOfAntinodes.len());
    //println!("Hashset size: {}    {:?}", setOfAntinodes.len(),setOfAntinodes);
    
}