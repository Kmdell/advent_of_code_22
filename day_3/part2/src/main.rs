use std::fs;

fn get_id(sack: &str) -> u64 {
    let mut value: u64 = 0;
    for elem in sack.chars() {
        if elem >= 'a' {
            value |= 1 << ((elem as u32) - 96);
        } else {
            value |= 1 << ((elem as u32) - 38);
        }
    }
    value
}

fn main() {
    // get the data from the file
    let file: String = fs::read_to_string("rucksack.txt").unwrap();
    // split the contents of the file into a vector of &str where the split was at newlines
    let mut file: Vec<&str> = file.split('\n').collect();
    // pops off the end of the vector if its empty
    if file.last().unwrap().is_empty() {
        file.pop();
    }
    let mut value: u32 = 0;
    for triplet in file.chunks(3) {
        let mut temp: u64 = get_id(triplet[0]) & get_id(triplet[1]) & get_id(triplet[2]);
        while temp & 1 == 0 {
            value += 1;
            temp >>= 1;
        }
    }
    println!("{}", value);
}
