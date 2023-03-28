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
            ('A', 'X') | ('B', 'X') | ('C', 'X') => match line.0 {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => 0,
            },
            ('A', 'Y') | ('B', 'Y') | ('C', 'Y') => 3 + match line.0 {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => 0,
            },
            ('A', 'Z') | ('B', 'Z') | ('C', 'Z') => 6 + match line.0 {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => 0,
            },
            _ => 0,
        };
    }
    println!("{}", points);
}
