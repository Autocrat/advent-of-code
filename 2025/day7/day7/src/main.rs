use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let mut lines = contents.lines();
    let row_1 : Vec<char> = lines.next().unwrap().chars().collect();
    let line_length = row_1.len();
    let mut beam_line : Vec<i64> = vec![0;line_length];
    let mut paths_through_columns : Vec<i64> = vec![0;line_length];
    let position_s = row_1.iter().position(|c| *c == 'S').unwrap();

    let mut split_count : i64 = 0;
    
    beam_line[position_s]=1;
    paths_through_columns[position_s] = 1;

    for i in lines {
        let indices : Vec<usize> = i.chars().enumerate().filter_map(|(index,value)| { 
            if value == '^' {
                Some(index)
            } else { None }
         }).collect();
         
         if !indices.is_empty() {
            for splitter_position in indices.iter() {
                if beam_line[*splitter_position] > 0 {
                    split_count += 1;
                    if *splitter_position > 0 && * splitter_position < line_length-1 {
                        beam_line[*splitter_position-1] = 1;
                        beam_line[*splitter_position+1] = 1;

                        paths_through_columns[*splitter_position-1] += paths_through_columns[*splitter_position];
                        paths_through_columns[*splitter_position+1] += paths_through_columns[*splitter_position];
                    }
                    beam_line[*splitter_position] = 0;
                }
                paths_through_columns[*splitter_position] = 0;
            }
         }
    }
    
    let path_count : i64 = paths_through_columns.iter().sum();
 
    println!("Split count: {}   Path count: {}", split_count, path_count);
}