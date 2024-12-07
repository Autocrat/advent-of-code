#![allow(non_snake_case)]
use std::fs;

fn getCharacterAtPosition(fileAsALump : &Vec<&str>, x : usize, y : usize) -> char {
    let retVal = fileAsALump[y].chars().nth(x).expect("Wtf did you expect");
    return retVal;
}


fn getNorthWest(fileAsALump : &Vec<&str>, x : usize ,y : usize) -> (bool, char)
{
    if y > 0 && x > 0 {
        let shiftedX = x - 1;
        let shiftedY = y - 1;
        return (true, getCharacterAtPosition(fileAsALump,shiftedX,shiftedY));
    } else {
        return (false, ' ');
    }    
}

fn getNorthEast(fileAsALump : &Vec<&str>, x : usize ,y : usize) -> (bool, char)
{
    if y > 0 && x < (fileAsALump[y].len()-1) {
        let shiftedX = x + 1;
        let shiftedY = y - 1;
        return (true, getCharacterAtPosition(fileAsALump,shiftedX,shiftedY));
    } else {
        return (false, ' ');
    }
}

fn getSouthWest(fileAsALump : &Vec<&str>, x : usize ,y : usize) -> (bool, char)
{
    if y < (fileAsALump.len()-1) && x > 0 {
        let shiftedX = x - 1;
        let shiftedY = y + 1;
        return (true, getCharacterAtPosition(fileAsALump,shiftedX,shiftedY));
    } else {
        return (false, ' ');
    }
}

fn getSouthEast(fileAsALump : &Vec<&str>, x : usize ,y : usize) -> (bool, char)
{
    if y < (fileAsALump.len()-1) && x < (fileAsALump[y].len()-1) {
        let shiftedX = x + 1;
        let shiftedY = y + 1;
        return (true, getCharacterAtPosition(fileAsALump,shiftedX,shiftedY));
    } else {
        return (false, ' ');
    }
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
            if foundChar == 'A' {
                let (_nwError,nw) = getNorthWest(&linesOfStrings,x,y);
                let (_seError,se) = getSouthEast(&linesOfStrings,x,y);
                let (_swError,sw) = getSouthWest(&linesOfStrings,x,y);
                let (_neError,ne) = getNorthEast(&linesOfStrings,x,y);

                let check1 = (nw == 'S' && se == 'M') || (nw == 'M' && se == 'S');
                let check2 = (ne == 'S' && sw == 'M') || (ne == 'M' && sw == 'S');

                if check1 && check2 {
                    totalWords += 1;
                }
            }
        }
    }
    println!("{totalWords}");
}
