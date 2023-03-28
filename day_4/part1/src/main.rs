use std::fs;

fn main() {
    // get the contents of the file into file
    let file: String = fs::read_to_string("ids.txt").unwrap();
    let mut file: Vec<&str> = file.split('\n').collect();
    if file.last().unwrap().is_empty() {
        file.pop();
    }
    // break down the file into the ranges
    let mut ranges: Vec<(i32, i32, i32, i32)> = vec![];
    for line in file {
        let temp: Vec<&str> = line.split(&['-', ','][..]).collect();
        ranges.push((temp[0].parse::<i32>().unwrap(), temp[1].parse::<i32>().unwrap(), temp[2].parse::<i32>().unwrap(), temp[3].parse::<i32>().unwrap()));
    }
    let mut value = 0;
    for set in ranges {
        if set.0 <= set.2 && set.1 >= set.3 {
            value += 1;
        } else if set.2 <= set.0 && set.3 >= set.1 {
            value += 1;
        }
    }
    println!("{}", value);
}
