use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines = contents.lines();
    let mut range_vec : Vec<_> = Vec::new();
    
    let mut ingredients_fresh : i64 = 0;

    let mut separator_found = false;

    for i in lines {
        if i == "" {
            separator_found = true;
        } else {
            if !separator_found {
                if let Some ((start,stop)) = i.split_once('-') {
                    let fresh_range = start.parse::<i64>().expect("Start not a number")..=stop.parse::<i64>().expect("Stop not a number");
                    range_vec.push(fresh_range);
                }
            } else {
                let ingredient = i.parse::<i64>().expect("Ingredient is not a number");
                for ranges in range_vec.iter() {
                    if ranges.contains(&ingredient) {
                        ingredients_fresh += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ingredients_fresh);
}