use std::fs;

fn main() {
    //vector of strings to use forward value names
    let mut string_vector: Vec<String> = Vec::new();
    string_vector.push("zero".to_string());    
    string_vector.push("one".to_string());
    string_vector.push("two".to_string());
    string_vector.push("three".to_string());
    string_vector.push("four".to_string());
    string_vector.push("five".to_string());
    string_vector.push("six".to_string());
    string_vector.push("seven".to_string());
    string_vector.push("eight".to_string());
    string_vector.push("nine".to_string());

    let mut rev_string_vector: Vec<String> = Vec::new();
    rev_string_vector.push("orez".to_string());    
    rev_string_vector.push("eno".to_string());
    rev_string_vector.push("owt".to_string());
    rev_string_vector.push("eerht".to_string());
    rev_string_vector.push("ruof".to_string());
    rev_string_vector.push("evif".to_string());
    rev_string_vector.push("xis".to_string());
    rev_string_vector.push("neves".to_string());
    rev_string_vector.push("thgie".to_string());
    rev_string_vector.push("enin".to_string());
 
    //total
    let mut sum = 0;
    
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    //iterate line by line
    let line_contents = contents.lines();
    for line in line_contents{
        //need a string to hold the total number
        let mut number: String = Default::default();

        //find the index of the lowest numeric search string
        let mut first_digit='0';
        let mut word_ordinal_value = -1;
        let mut lowest_index:i32 = 1000;
        for search_string in &string_vector{
            word_ordinal_value+=1;
            let find_result = line.find(search_string);
            if find_result.is_some()
            {
                let found_index = find_result.unwrap().try_into().unwrap();
                if found_index < lowest_index {
                    lowest_index = found_index;
                    //convert word to value - maybe use ordinal position in vector?
                    //the word ordinal value IS the value, and since we've just been searching it in order,
                    //we have it
                    first_digit=word_ordinal_value.to_string().chars().next().unwrap();
                }
            }
        }
        //then look for the first digit
        let mut char_index:i32 =-1;
        let char_iter = line.chars();
        for character in char_iter{
            char_index+=1;
            if character.is_numeric(){
                //found a digit, check if the index is lower than what we found previous
                if char_index < lowest_index {
                    //if it's lower, we use the digit instead of the word
                    first_digit=character;
                    println!("Using digit {first_digit}");
                    break;
                }
            }
        }
        number.push(first_digit);


/////////////////////////////////////////////////////////////////////////////////////////
        
        
        //copy the line in reverse to get the 2nd digit
        let reverse_line = line.chars().rev().collect::<String>();
        let mut last_digit = '0';
        //reset some variables.  Still looking left to right, so it works the same way
        lowest_index = 1000;
        word_ordinal_value = -1;
        for search_string in &rev_string_vector{
            println!("Searching for {search_string} in {reverse_line}");
            word_ordinal_value+=1;
            let find_result = reverse_line.find(search_string);
            if find_result.is_some()
            {
                println!("found {search_string}");
                let found_index = find_result.unwrap().try_into().unwrap();
                if found_index < lowest_index {
                    lowest_index = found_index;
                    //convert word to value - maybe use ordinal position in vector?
                    //the word ordinal value IS the value, and since we've just been searching it in order,
                    //we have it
                    last_digit=word_ordinal_value.to_string().chars().next().unwrap();
                }
            }
        }
        
        char_index = -1;
        let char_iter = reverse_line.chars();
        for character in char_iter{
            char_index+=1;
            if character.is_numeric(){
                //found a digit, check if the index is lower than what we found previous
                if char_index < lowest_index {
                    //if it's lower, we use the digit instead of the word
                    last_digit=character;
                    break;
                }
            }
        }
        number.push(last_digit);
        println!("Number: {number}");
        let value:i32 = number.parse().unwrap();
        sum+=value;
     }
     println!("Sum: {sum} ");

}