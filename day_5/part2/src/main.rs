use std::fs;

fn move_stacks(num: usize, from: usize, to: usize, mut cargo: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut temp: Vec<char> = vec![];
    for _ii in 0..num {
        temp.push(cargo[from - 1].pop().unwrap());
    }
    for crat in temp.iter().rev() {
        cargo[to - 1].push(*crat);
    }
    cargo
}

fn main() {
    // read the file to a string
    let file: String = fs::read_to_string("stacks.txt").unwrap();
    let file: Vec<&str> = file.split("\n\n").collect();
    let mut cargo: Vec<&str> = file[0].split('\n').collect();
    let mut instr: Vec<&str> = file[1].split('\n').collect();
    if instr.last().unwrap().is_empty() {
        instr.pop();
    }
    
    let num_cats: usize = cargo.pop().unwrap().split("   ").collect::<Vec<&str>>().len();
    let mut vec_cargo: Vec<Vec<char>> = vec![vec![]; num_cats];

    for line in cargo.iter().rev() {
        let mut vec_index: usize = 0;
        let mut str_index: usize = 1;
        while str_index < num_cats * 4 {
            let temp = line.chars().collect::<Vec<_>>()[str_index];
            if temp != ' ' {
                vec_cargo[vec_index].push(temp);
            }
            vec_index += 1;
            str_index += 4;
        }
    }
    
    let mov_instr: Vec<(usize, usize, usize)> = instr.iter().map( |x|
        {
            let temp: Vec<&str> = x.split(" ").collect::<Vec<&str>>();
            (temp[1].parse::<usize>().unwrap(), temp[3].parse::<usize>().unwrap(), temp[5].parse::<usize>().unwrap())
        }).collect();

    for inst in &mov_instr {
        vec_cargo = move_stacks(inst.0, inst.1, inst.2, vec_cargo);
    }
    println!("{:?}", vec_cargo);
}
