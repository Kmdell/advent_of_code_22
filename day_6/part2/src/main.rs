use std::fs;

fn main() {
    let file: Vec<u8> = fs::read("code.txt").unwrap();
    
    let index = file.windows(14).position(|x| {
        let mut temp: u64 = 0;
        for i in x {
            if temp >> (*i - 65) & 1 == 1 {
                return false;
            } else {
                temp |= 1 << *i - 65;
            }
        }
        return true;
    }).map(|x| x + 14).unwrap();
    println!("{:?}", index);
}
