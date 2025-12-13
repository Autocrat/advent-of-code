use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");

    //read the line and convert it to vec<char>
    let mut data : Vec<Vec<char>> = Vec::new();

    for line_result in contents.lines() {
        data.push(line_result.chars().collect());
    }

    let mut num_stacks = 0;

    let y_max = data.len();
    let x_max = data[0].len();

    let mut done : bool = false;

    while done != true {
        done = true;
        'outer: for y in 0..y_max {
            for x in 0..x_max {
                //found a stack, check for surrounding stacks
                if data[y][x] == '@' {
                    let mut num_surrounding = 0;
                    let min_i = if x > 0 {x -1} else {x};
                    let max_i = if x == (x_max -1) {x_max} else {x+2};

                    let min_j = if y > 0 { y -1} else {y};
                    let max_j = if y == (y_max -1) {y_max} else {y+2};

                    for j in min_j..max_j {
                        for i in min_i..max_i {
                            if data[j][i] == '@' {
                                num_surrounding += 1;
                            }
                        }
                    }
                    if num_surrounding < 5 {
                        data[y][x]='x';
                        num_stacks += 1;
                        done = false;
                        break 'outer;
                    }
                }
            }
        } 
    }
    println!("{}",num_stacks);
}