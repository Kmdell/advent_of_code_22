use std::fs;

fn main() {
    let file_contents = fs::read_to_string("calories.txt").unwrap();
    let file_contents_split: Vec<&str> = file_contents.split("\n\n").collect();
    let mut ordered_vec: Vec<i32> = vec![];
    for temp_str in &file_contents_split {
        let mut value: i32 = 0;
        let elf_calories: Vec<&str> = temp_str.split('\n').collect(); 
        for num in &elf_calories {
            let num_str = num.to_string();
            match num_str.parse::<i32>() {
                Ok(x) => value += x,
                Err(_) => value = value,
            };
            ordered_vec.push(value);
        }
    }
    ordered_vec.sort_by(|a, b| b.cmp(a));
    let mut value: i32 = 0;
    if ordered_vec.len() > 2 {
        value = ordered_vec[0] + ordered_vec[1] + ordered_vec[2];
    }
    println!("{}", value);
}
