#![allow(non_snake_case)]
use std::fs;
use std::collections::HashMap;

fn getCharacterAtPosition(fileAsALump : &Vec<&str>, position : &(usize, usize)) -> char {
    let retVal = fileAsALump[position.1].chars().nth(position.0).expect("Wtf did you expect");
    return retVal;
}



fn checkNextPosition(fileAsALump : &Vec<&str>, position : &(usize, usize), direction : &char) -> char {
    match direction {
        '^'=> { 
                if position.1 != 0 {
                    return getCharacterAtPosition(fileAsALump,&(position.0,position.1 - 1 ));
                }
            },
        '>'=> {
                if position.0 != (fileAsALump[position.1].len()-1) {
                    return getCharacterAtPosition(fileAsALump,&(position.0 + 1,position.1 ));                    
                }
            }
        'v'=> {
                if position.1 != (fileAsALump.len()-1) {
                    return getCharacterAtPosition(fileAsALump,&(position.0 ,position.1 + 1 ));                    
                }
            }
        '<'=> { 
                if position.0 != 0 {
                    return getCharacterAtPosition(fileAsALump,&(position.0 -1 ,position.1 ));
                }
            },
          _=> (),
    }
    return '*';
}

fn moveGuard(position : &mut (usize, usize), direction : &char, visitedMap : &mut HashMap<(usize,usize),bool>)  {
    match direction {
        '^'=> position.1 -= 1,
        '>'=> position.0 += 1,
        'v'=> position.1 += 1,
        '<'=> position.0 -= 1,
          _=> (),
    }
    visitedMap.insert((position.0,position.1), true);
}

fn rotateGuard(direction : &char) -> char {
    match direction {
        '^'=> return '>',
        '>'=> return 'v',
        'v'=> return '<',
        '<'=> return '^',
          _=> return '_',
    }
}

    //put the current position in the map to start
    //Read guard position character
    //determine direction.
    //check for character in that direction
    //if character is '.'
    // move
    // add to guardPosition map
    //if character is '#'
    // rotate
    //if character is '*'
    //  use '*' to indicate that guard is moving off the board
    //  print the size of the hashmap
    //  exit loop/done
fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    let mut guardPosition : (usize,usize) = (0,0);
    let mut guardDirection : char = '^';
    let linesOfStrings : Vec<_> = contents.lines().collect();

    for y in 0..linesOfStrings.len() {
        for x in 0..linesOfStrings[0].len() {
            let foundChar = getCharacterAtPosition(&linesOfStrings,&(x,y));
            if foundChar == guardDirection {
                guardPosition = (x,y);
            }
        }
    }

    let mut positionsVisited : HashMap<(usize,usize),bool> = HashMap::new();
    positionsVisited.insert(guardPosition,true);

    let mut done = false;
    while ! done {
        let nextPosChar = checkNextPosition(&linesOfStrings, &guardPosition, &guardDirection );
        if nextPosChar == '*' {
            done = true;
        } else if nextPosChar == '#' {
            guardDirection = rotateGuard(&guardDirection);
        } else {
            if guardPosition.0 == 0 || guardPosition.1 == 0 {
                println!("Right before move: {:?} {}", guardPosition, guardDirection);
                done = true;
            }
        
           moveGuard(&mut guardPosition, &guardDirection,&mut positionsVisited);
           
        }
    }

    println!("{}",positionsVisited.len());
}