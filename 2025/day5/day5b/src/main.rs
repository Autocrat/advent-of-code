use std::fs;

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines = contents.lines();
    let mut ranges : Vec<_> = Vec::new();
    
    
    for i in lines {
        if let Some ((start,stop)) = i.split_once('-') {
            let start_i = start.parse::<i64>().expect("Start not a number");
            let stop_i = stop.parse::<i64>().expect("Stop not a number");
            //RangeInclusive doesn't allow you to read the start/end fields
            //increment by 1 to make a regular range behave similarly for this case
            ranges.push( start_i..(stop_i+1));
        }
    }

    ranges.sort_by_key(|r| r.start);
    let mut merged = Vec::new();
    let mut current = ranges[0].clone();

    for r in ranges.into_iter().skip(1) {
        if r.start <= current.end {
            //overlapping or touching.  Extend the current range
            current.end = current.end.max(r.end);
        } else {
            //no overlap.  Push current and start a new one
            merged.push(current);
            current = r;
        }
    }
    merged.push(current);
    let mut total = 0;
    for i in merged.iter() {
        total += i.end - i.start;
    }
    println!("{}",total);
}