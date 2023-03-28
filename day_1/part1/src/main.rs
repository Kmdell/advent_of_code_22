use std::fs;

fn main() {
    let file_contents = fs::read_to_string("calories.txt").unwrap();
    let file_contents_split: Vec<&str> = file_contents.split("\n\n").collect();
    let mut max_value: i32 = 0;
    for temp_str in &file_contents_split {
        let mut value: i32 = 0;
        let elf_calories: Vec<&str> = temp_str.split('\n').collect(); 
        for num in &elf_calories {
            let num_str = num.to_string();
            match num_str.parse::<i32>() {
                Ok(x) => value += x,
                Err(_) => value = value,
            };
            if value > max_value {
                max_value = value;
            }
        }
    }
    println!("{}", max_value);
}
