fn main() {
    let file: String = std::fs::read_to_string("tournament.txt").unwrap();
    let file: Vec<&str> = file.split('\n').collect();
    let mut contents: Vec<(char, char)> = vec![];
    for line in file {
        if line != "" {
            let con_line: Vec<&str> = line.split(' ').collect();
            contents.push((con_line[0].parse::<char>().unwrap(), con_line[1].parse::<char>().unwrap()));
        }
    }
    let mut points = 0;
    for line in contents {
        points += match line {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => 0,
        };
        points += match line.1 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };
    }
    println!("{}", points);
}
