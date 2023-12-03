
use std::fs;
fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");
    let line_contents = contents.lines();
    
    let mut power_total = 0;
    //go through every line
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

            //take the right hand side of the colon
            let Some(element2) = first_iter.next() else { panic!()};
            let element2_trimmed = element2.trim();

            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;
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
//                    println!("Looking at Rocks: {rocks_trimmed}");
                    
                    //split the rocks into the count and the color
                    let mut count_color_iter = rocks_trimmed.split(' ');
                    let count = count_color_iter.next().unwrap().trim();
                    let color = count_color_iter.next().unwrap().trim();
//                    println!("Count: {count} Color: {color}");
                    let count:i32 = count.parse().unwrap();
                    
                    //get the min count for each color across grabs
                    if color == "red" {
                        min_red = count.max(min_red);
                    } else if color == "blue" {
                        min_blue = count.max(min_blue);
                    } else if color == "green" {
                        min_green = count.max(min_green);
                    }
                }
            }
            let power_game = min_red * min_blue * min_green;
            println!("Power = {power_game}    Red: {min_red} Blue: {min_blue} Green: {min_green}");
            power_total += power_game;
        }  
    }
    println!("Final total power: {power_total}");
}
