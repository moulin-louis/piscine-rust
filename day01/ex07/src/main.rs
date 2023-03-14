use ftkit::{ARGS, read_line};

fn remove_space( paragraphes:&mut Vec<String>, pos:usize ) {
	let mut temp_str = paragraphes[pos].clone();
	let mut mod_space = 0;
	for mut i in 0..temp_str.len() {
		println!("i = {}", i );
		if temp_str.chars().nth(i).unwrap() == ' ' {
			let temp = i + 1;
			loop
			{
				if temp_str.chars().nth(temp).unwrap() == ' ' {
					temp_str.remove(temp);
					i -= 1;
				}
				else {
					break ;
				}
			}
		}
	}

}

fn parsing_input( input:&String ) -> Vec<String> {
	let mut paragraphes:Vec<String> = input.split(&"\n\n".to_string()).map(str::to_string).collect();
	let mut i:usize = 0;
	for i in 0.. paragraphes.len() {
		if paragraphes[i].contains(' ') {
			remove_space(&mut paragraphes, i);
		}
	}
	let mut result:Vec<String> = Vec::new();
	result
}

fn main() {
	if ARGS.len() != 2 { panic!("Wrong nbr of args"); }
	let nbr_colums = &ARGS[1];
	let user_input = read_line();
	let parsed_input:Vec<String> = parsing_input(&user_input);
}
