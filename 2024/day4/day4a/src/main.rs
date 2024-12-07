#![allow(non_snake_case)]
use std::fs;

fn getCharacterAtPosition(fileAsALump : &Vec<&str>, x : usize, y : usize) -> char {
    let retVal = fileAsALump[y].chars().nth(x).expect("Wtf did you expect");
    return retVal;
}

fn getNextLetter(character : char) -> char {
    match character {
        'X'=>return 'M',
        'M'=>return 'A',
        'A'=>return 'S',
        'S'=>return '_',
          _=>return '*',
    }
}

fn checkNorth(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if y > 0 {
        let shiftedY = y - 1;
        if getCharacterAtPosition(fileAsALump,x,shiftedY) == nextLetter {
            return checkNorth(fileAsALump,x,shiftedY,nextLetter);
        }
    }
    return false;
}

fn checkSouth(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if y < (fileAsALump.len()-1) {
        let shiftedY = y + 1;
        if getCharacterAtPosition(fileAsALump,x,shiftedY) == nextLetter {
            return checkSouth(fileAsALump,x,shiftedY,nextLetter);
        }
    }
    return false;
}

fn checkEast(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if x < (fileAsALump[y].len()-1) {
        let shiftedX = x + 1;
        if getCharacterAtPosition(fileAsALump,shiftedX,y) == nextLetter {
            return checkEast(fileAsALump,shiftedX,y,nextLetter);
        }
    }
    return false;
}

fn checkWest(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if x > 0 {
        let shiftedX = x - 1;
        if getCharacterAtPosition(fileAsALump,shiftedX,y) == nextLetter {
            return checkWest(fileAsALump,shiftedX,y,nextLetter);
        }
    }
    return false;
}


fn checkSouthEast(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if y < (fileAsALump.len()-1) && x < (fileAsALump[y].len()-1) {
        let shiftedX = x + 1;
        let shiftedY = y + 1;
        if getCharacterAtPosition(fileAsALump,shiftedX,shiftedY) == nextLetter {
            return checkSouthEast(fileAsALump,shiftedX,shiftedY,nextLetter);
        }
    }
    return false;
}

fn checkSouthWest(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if y < (fileAsALump.len()-1) && x > 0 {
        let shiftedX = x - 1;
        let shiftedY = y + 1;
    if getCharacterAtPosition(fileAsALump,shiftedX,shiftedY) == nextLetter {
            return checkSouthWest(fileAsALump,shiftedX,shiftedY,nextLetter);
        }
    }
    return false;
}


fn checkNorthEast(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if y > 0 && x < (fileAsALump[y].len()-1) {
        let shiftedX = x + 1;
        let shiftedY = y - 1;
        if getCharacterAtPosition(fileAsALump,shiftedX,shiftedY) == nextLetter {
            return checkNorthEast(fileAsALump,shiftedX,shiftedY,nextLetter);
        }
    }
    return false;
}

fn checkNorthWest(fileAsALump : &Vec<&str>, x : usize ,y : usize, c : char) -> bool {
    let nextLetter = getNextLetter(c);
    if nextLetter == '_' {
        return true;
    } else if y > 0 && x > 0 {
        let shiftedX = x - 1;
        let shiftedY = y - 1;
        if getCharacterAtPosition(fileAsALump,shiftedX,shiftedY) == nextLetter {
            return checkNorthWest(fileAsALump,shiftedX,shiftedY,nextLetter);
        }
    }
    return false;
}


fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let linesOfStrings : Vec<_> = contents.lines().collect();
    let mut totalWords = 0;
    for y in 0..linesOfStrings.len() {
        for x in 0..linesOfStrings[0].len() {
            let foundChar = getCharacterAtPosition(&linesOfStrings,x,y);
            if foundChar == 'X' {
                if checkNorth(&linesOfStrings,x,y,foundChar)
                {
                    totalWords += 1;
                }
                if checkSouth(&linesOfStrings,x,y,foundChar) {
                    totalWords += 1;
                }  

                if checkEast(&linesOfStrings,x,y,foundChar) {
                    totalWords += 1;
                }  

                if checkWest(&linesOfStrings,x,y,foundChar) {
                    totalWords += 1;
                }  

                if checkNorthEast(&linesOfStrings,x,y,foundChar)
                {
                    totalWords += 1;
                }
                if checkSouthEast(&linesOfStrings,x,y,foundChar) {
                    totalWords += 1;
                }  

                if checkNorthWest(&linesOfStrings,x,y,foundChar) {
                    totalWords += 1;
                }  

                if checkSouthWest(&linesOfStrings,x,y,foundChar) {
                    totalWords += 1;
                }  


            }
        }
    }
    println!("{totalWords}");
}
