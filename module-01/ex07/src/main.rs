use ftkit::{read_line, ARGS};
// use unicode_width::UnicodeWidthStr;

fn parse_input(input: &[String]) -> Vec<String> {
    let lines: Vec<&String> = input.iter().collect();
    // println!();
    // dbg!(lines.clone());
    let lines: Vec<&str> = lines.iter().flat_map(|&line| line.split(' ')).collect();
    let lines: Vec<&&str> = lines.iter().filter(|&line| !(*line).is_empty()).collect();
    let mut lines: Vec<String> = lines.iter().map(|&line| line.to_string()).collect();
    for line in lines.iter_mut() {
        if line.len() > 1 && line.ends_with('\n') {
            line.pop();
        }
    }
    let mut i = 0;
    while i < lines.len() {
        let mut j = i + 1;
        while j < lines.len() {
            if lines[i] == lines[j] {
                lines.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    lines
}

fn main() {
    if ARGS.len() != 2 {
        panic!("Wrong nbr of argument!");
    }
    let nbr_column: usize = match ARGS[1].parse() {
        Err(e) => panic!("{e}"),
        Ok(t) => t,
    };
    let lines: Vec<String>;
    {
        let mut lines_input: Vec<String> = Vec::new();
        loop {
            let line: String = read_line();
            if line.is_empty() {
                break;
            }
            lines_input.push(line);
        }
        lines = parse_input(&lines_input);
    }
    let mut char_written = 0;
    for idx in 0..lines.len() {
        if lines[idx] == "\n" {
            println!();
            println!();
            continue;
        }
        if char_written + lines[idx].len() < nbr_column {
            print!("{}", lines[idx]);
            char_written += lines[idx].len();
            if idx + 1 < lines.len() && lines[idx + 1].len() + char_written + 1 < nbr_column {
                print!(" ");
                char_written += 1;
            }
        } else {
            println!();
            print!("{}", lines[idx]);
            print!(" ");
            char_written = lines[idx].len();
        }
    }
    println!();
}
