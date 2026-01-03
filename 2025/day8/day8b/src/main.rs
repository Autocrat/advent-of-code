use std::fs;
use std::collections::HashMap;

fn get_rough_distance(p1 : (i64,i64,i64), p2 : (i64,i64,i64)) -> i64 {
    return (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1)  *(p1.1 - p2.1) + (p1.2 - p2.2) * (p1.2 - p2.2);
}

fn main() {
    //read strings from a file
    let contents = fs::read_to_string("input.txt").expect("Yo! could not read file");
    let lines = contents.lines();
    let mut junction_boxes : Vec<(i64,i64,i64)> = Vec::new();

    //read line, convert to vector of coordinates, stick in vector of boxes
    for i in lines {
        let junction_box : Vec<_> = i.split(',').map(|x| x.parse::<i64>().expect("found non digit")).collect();
        junction_boxes.push( (junction_box[0],junction_box[1],junction_box[2]));
    }

    //go through all the boxes, creating a Hash so as to be able to look up the distance between
    //any 2 points by using the 2 sets of coordinates as the key
    //don't add it if the 2 points are the same or if the reverse of the same coordinates has been added
    let mut distance_map : HashMap<((i64,i64,i64),(i64,i64,i64)),i64> = HashMap::new();
    for i in 0..(junction_boxes.len()) {
        for j in 0..junction_boxes.len() {
            if junction_boxes[i] != junction_boxes[j] {
                if let None = distance_map.get(&(junction_boxes[j],junction_boxes[i])) {
                    let distance = get_rough_distance(junction_boxes[i],junction_boxes[j]);
                    distance_map.insert((junction_boxes[i],junction_boxes[j]), distance);
                }
            }
        }
    }
    //End size of the "grid" should  = (num_lines^2 - numlines)/2

    //flatten the "grid" and sort by distance
    //vector should be something like Vec<((i64,i64,i64),(i64,i64,i64),i64>  (a tuple containing 2 tuples and a distance)
    let mut sorted_vector : Vec<_> = distance_map.iter().collect();
    sorted_vector.sort_by(|a,b| a.1.cmp(&b.1));

    let mut circuits_map : HashMap<(i64,i64,i64),usize> = HashMap::new();
    let mut next_circuit_num : usize = 0;
    let mut circuit_counts : Vec<usize> = vec![0;sorted_vector.len()];

    // go through the top most elements of the array.  Look it up in a map - if it's not there, check its partner.   If neither is there, assign a new circuit number and insert both into the map
    // if either one is, then its partner has to be part of the same circuit.
    // keep a count (of how many nodes are in the each circuit - circuit id would be the vector index)

    let mut last_connection : ((i64,i64,i64), (i64,i64,i64)) = ((0,0,0), (0,0,0));
    for i in sorted_vector.iter() {
        let mut circuit_num : usize = next_circuit_num;
        if let Some(found_circuit_num) = circuits_map.get(&i.0.0) {
            circuit_num = *found_circuit_num;
            if let Some(found_other_circuit_num) = circuits_map.get(&i.0.1) {
                if found_circuit_num != found_other_circuit_num {
                    //Combine the 2 circuits
                    //Iterate over the map, and wherever the 2nd circuit is found, change it to the 1st circuit
                    //Add to the circuit count for the first circuit, but the 2nd circuit needs to change to 0
                    let tmp_circuit_num = *found_other_circuit_num;
                    circuit_counts[tmp_circuit_num] = 0;
                    for (_key,data) in circuits_map.iter_mut()
                    {
                        if *data == tmp_circuit_num {
                            *data = circuit_num;
                             circuit_counts[circuit_num] += 1;
                        }
                    }
                }
            } else {
                circuits_map.insert(i.0.1,circuit_num);
                circuit_counts[circuit_num] += 1;
            }
        }  else if let Some(found_circuit_num) = circuits_map.get(&i.0.1) {
            circuit_num = *found_circuit_num;
            circuits_map.insert(i.0.0,circuit_num);
            circuit_counts[circuit_num] += 1;
        } else {
            circuits_map.insert(i.0.0,circuit_num);
            circuits_map.insert(i.0.1,circuit_num);
            circuit_counts[circuit_num] += 2;
            next_circuit_num += 1;
        }

        if circuit_counts[circuit_num] == junction_boxes.len() && last_connection == ((0,0,0),(0,0,0)) {
            last_connection = (i.0.0, i.0.1);      
        }

    }
    
    circuit_counts.sort_by(|a,b| b.cmp(a));
    println!("last_connection: {:?}", last_connection);
    println!("{} * {} = {}", last_connection.0.0, last_connection.1.0, last_connection.0.0 * last_connection.1.0);

}
