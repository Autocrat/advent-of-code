use std::fs;

fn main() {
    //total
    let mut sum = 0;
    
    //read strings from a file
    let contents = fs::read_to_string("input.txt")
        .expect("Yo! could not read file");

    //iterate line by line
    let line_contents = contents.lines();
    for line in line_contents{
        let mut first_digit_found = false;
        let mut last_digit = '0';
        //variable for the number on the line
        let mut number: String = Default::default();

        //iterate character by character
        let char_iter = line.chars();
        for character in char_iter{
            if character.is_numeric() {
                if first_digit_found {
                    last_digit = character;
                }
                else {
                    first_digit_found = true;
                    number.push(character);
                    last_digit = character;
                }
            }
        } 
        number.push(last_digit);
        let value:i32 = number.parse().unwrap();
        println!("Value: {value} ");
        sum+=value;
    
    }
    println!("Total: {sum}");

}