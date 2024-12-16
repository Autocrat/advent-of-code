#![allow(non_snake_case)]
use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
    .expect("Yo! could not read file");
    let linesOfStrings = contents.lines();
    let mut decompressedIntVec : Vec<(i64,i64)> = Vec::new();

    for lines in linesOfStrings {
        let mut x = 0;
        let mut id : i64 = 0;
        for character in lines.chars() {
            let length : i64 = character.to_digit(10).unwrap().into();
            let mut valueToAdd : i64 = -1;
            if (x%2) == 0 {
                valueToAdd = id;
                id += 1;
            }
            decompressedIntVec.push((valueToAdd,length));
            x +=1;
        }
 
        //search from the end for a file to move
        for mut endIndex in (0..decompressedIntVec.len()).rev() {
            if decompressedIntVec[endIndex].0 != -1 { //indicates a file
                //get the filesize
                let fileSize = decompressedIntVec[endIndex].1;
                for blockIndex in 0..endIndex { //don't look past the end index
                    if decompressedIntVec[blockIndex].0 == -1 {
                        let spaceSize = decompressedIntVec[blockIndex].1;
                        if fileSize <= spaceSize {
                            if fileSize < spaceSize {
                                decompressedIntVec.insert(blockIndex+1,(-1,spaceSize - fileSize));
                                endIndex += 1;
                            }
                            decompressedIntVec[blockIndex]=decompressedIntVec[endIndex];
                            decompressedIntVec[endIndex].0 = -1;
                            break;
                        }
                    }
                }

            }
            
        }

        //go through the whole damn thing and consolidate spaces
        let mut endIndex = decompressedIntVec.len()-1;
        let mut i = 0;
        while i < endIndex {
            if decompressedIntVec[i].0 == -1 && decompressedIntVec[i+1].0 == -1 {
                //found adjacent.  Combine.
                decompressedIntVec[i].1 += decompressedIntVec[i+1].1;
                decompressedIntVec.remove(i+1);
                i -= 1; 
                endIndex -= 1;
            }
            i+=1;  
        }
    }

    let mut stringRep :String = "".to_string();
    for (val, length) in decompressedIntVec.iter() {
        for _i in 0..*length {
            if *val == -1 {
                stringRep += ".";
            } else {
                stringRep += &val.to_string();
            }
        }
    }

    let mut total : i64 = 0;
    let mut index : i64= 0;
    for (id,length) in decompressedIntVec.iter() {
        for _i in 0..*length {
            if *id != -1 {
                total += index * id;
            }
            index += 1;
        }
    }
//    println!("String: {}", stringRep);
    println!("Total: {}", total);
}