
use std::fs;
fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");
    let line_contents = contents.lines();
    
    //go through every line
    let mut game_total_possible:i32 = 0;
    for line in line_contents{
        //split on colon, to get the game number section separate from the rest
        let mut first_iter = line.split(':');
        while let Some(element) = first_iter.next(){
            //get the game number from the first half
            let mut game_counter_iter = element.split(' ');
            game_counter_iter.next();
            let Some(game_number_string) = game_counter_iter.next() else { panic!()};
            let game_number_trimmed = game_number_string.trim();
            println!("Game number is {game_number_trimmed}");

            //flag to determine if the game is possible
            let mut game_possible = true;

            //take the right hand side of the colon
            let Some(element2) = first_iter.next() else { panic!()};
            let element2_trimmed = element2.trim();

            //split the right hand side of the colon by semi-colons, and loop
            //through them (each grab into the sack, I guess)
            let mut bag_grab_iter = element2_trimmed.split(';'); 
            while let Some(grab_result) = bag_grab_iter.next(){
                let grab_result_trimmed = grab_result.trim();
                println!("Handle a handful {grab_result_trimmed}");

                //split the grab result by comma to get each number and color
                let mut rocks_iter = grab_result_trimmed.split(',');
                while let Some(rocks) = rocks_iter.next(){
                    let rocks_trimmed = rocks.trim();
                    println!("Looking at Rocks: {rocks_trimmed}");
                    
                    //split the rocks into the count and the color
                    let mut count_color_iter = rocks_trimmed.split(' ');
                    let count = count_color_iter.next().unwrap().trim();
                    let color = count_color_iter.next().unwrap().trim();
                    println!("Count: {count} Color: {color}");
                    let count:i32 = count.parse().unwrap();
                    if color == "red" && count > 12 {
                        game_possible = false;
                    } else if color == "green" && count > 13 {
                        game_possible = false;
                    }else if count > 14 { //this covers blue
                        game_possible = false;
                    } 
                    if !game_possible {
                        break;
                    }
                }
                if !game_possible {
                    break;
                }
            }
            //if the game was possible, add the game number to the running total
            if game_possible {
                let game_number:i32 = game_number_trimmed.parse().unwrap();
                game_total_possible += game_number;
                println!("Game {game_number} was possible.  Total: {game_total_possible}");
            }
        }  
    }
    println!("Game total possible: {game_total_possible}");
}
