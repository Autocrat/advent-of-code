#![allow(non_snake_case)]
use std::fs;
use std::collections::HashMap;

fn getCharacterAtPosition(fileAsALump : &Vec<&str>, position : &(usize, usize)) -> char {
    let retVal = fileAsALump[position.1].chars().nth(position.0).expect("Wtf did you expect");
    return retVal;
}

fn checkNextPosition(fileAsALump : &Vec<&str>, position : &(usize, usize), direction : char) -> char {
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

fn willHitBarrel(position : &(usize, usize), direction : char, barrelPosition : &(usize,usize) ) -> bool {
    match direction {
        '^'=> { 
                if position.1 != 0 {
                    return  (position.0,position.1 - 1 ) == *barrelPosition;
                }
            },
        '>'=> {
                return (position.0 + 1,position.1 ) == *barrelPosition;                    
            }
        'v'=> {
                return (position.0 ,position.1 + 1 ) == *barrelPosition;                    
            }
        '<'=> { 
                if position.0 != 0 {
                    return (position.0 -1 ,position.1 ) == *barrelPosition;
                }
            },
          _=> return false,
    }
    return false;
}

//Be careful where you run this, so as not to drive usize negative (Rust doesn't like that)
//checkNextPosition will return '*' if it goes off the map (or negative), preventing the code in main from hitting this
fn getMoveToPosition(position : &(usize, usize), direction : char)  -> (usize,usize){
    let mut futurePosition = position.clone();
    match direction {
        '^'=> futurePosition.1 -= 1,
        '>'=> futurePosition.0 += 1,
        'v'=> futurePosition.1 += 1,
        '<'=> futurePosition.0 -= 1,
          _=> (),
    }
    return futurePosition;
}


fn rotateGuard(direction : char) -> char {
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

    let initialGuardPosition = guardPosition.clone();
    let initialGuardDirection = guardDirection; //don't need to clone a primitive


    let mut positionsVisited : HashMap<(usize,usize),char> = HashMap::new();
    positionsVisited.insert(initialGuardPosition,initialGuardDirection);


    let mut done = false;
    while ! done {
        let nextPosChar = checkNextPosition(&linesOfStrings, &guardPosition, guardDirection );
        if nextPosChar == '*' {
            done = true;
        } else if nextPosChar == '#' {
            guardDirection = rotateGuard(guardDirection);
        } else {
           guardPosition = getMoveToPosition(&guardPosition, guardDirection);
           positionsVisited.insert(guardPosition, guardDirection);
        }
    }


    println!("{}",positionsVisited.len());
    //the old path map can be used as a list of potential blockage locations
    //but have to remove the starting point (can't put the block on the guard)
    positionsVisited.remove(&initialGuardPosition); 


    let mut loopCount = 0;
    for (barrelPosition, _ignoredDirection) in positionsVisited {
        guardPosition = initialGuardPosition.clone();
        guardDirection = initialGuardDirection; //don't need to clone a primitive
        //new map to track where we've visited
        let mut loopPositionsVisited : HashMap<(usize,usize),char> = HashMap::new();
        loopPositionsVisited.insert(guardPosition,guardDirection);

        //don't check for loops until the guard hits a new obstacle (barrel)
        let mut enableLoopCheck = false;
        done = false;
        while ! done {
            let nextPosChar = checkNextPosition(&linesOfStrings, &guardPosition, guardDirection );
            if nextPosChar == '*' {
                done = true;
            } else if nextPosChar == '#' {
                guardDirection = rotateGuard(guardDirection);
            } else if willHitBarrel(&guardPosition, guardDirection, &barrelPosition) {
                guardDirection = rotateGuard(guardDirection);
                enableLoopCheck = true;
            }else {

                guardPosition = getMoveToPosition(&guardPosition, guardDirection);
                if enableLoopCheck {

                    match loopPositionsVisited.get(&guardPosition) {
                       Some(&directionRef) => { 
                           if directionRef == guardDirection {
                               loopCount += 1;
                               done = true;
                           }
                       },
                       _=> (),
                    }
                }
   
                //insert the position into the map after checking everything
                loopPositionsVisited.insert(guardPosition, guardDirection);

            }
        }
    }

    println!("Loopcount: {}", loopCount);


}