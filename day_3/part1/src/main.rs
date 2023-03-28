use std::fs;

fn main() {
    // get the data from the file
    let file: String = fs::read_to_string("rucksack.txt").unwrap();
    // split the contents of the file into a vector of &str where the split was at newlines
    let mut file: Vec<&str> = file.split('\n').collect();
    // pops off the end of the vector if its empty
    if file.last().unwrap().is_empty() {
        file.pop();
    }
    // split the the strings into a tuple that has half of string in each
    let mut split_file: Vec<(&str, &str)> = vec![];
    for line in file {
        split_file.push(line.split_at(line.len() / 2));
    }
    // 
    let mut value: u32 = 0;
    for line in split_file {
        let mut left: u64 = 0;
        let mut right: u64 = 0;
        // get the chars and iterator through filling value with the amount to the left the ascii is 
        for elem in line.0.chars() {
            if elem >= 'a' {
                left |= 1 << ((elem as u32) - 96);
            } else {
                left |= 1 << ((elem as u32) - 38);
            }
        }
        for elem in line.1.chars() {
            if elem >= 'a' {
                right |= 1 << ((elem as u32) - 96);
            } else {
                right |= 1 << ((elem as u32) - 38);
            }
        }
        // and together to find the char that is the same for both sides
        let mut temp: u64 = left & right;
        while temp & 1 == 0 {
            value += 1;
            temp >>= 1;
        }
    }
    println!("{}", value);
}
