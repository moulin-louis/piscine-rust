use ftkit::{read_line, read_number};

#[allow(dead_code)]
enum Command {
	Todo(String),
	Done(usize),
	Purge,
	Quit,
}

#[allow(dead_code)]
impl Command {
	fn prompt() -> Self {
		loop {
			let input = read_line();
			let input = input.trim_end().to_string();
			if input.is_empty() { return Command::Quit; }
			if input == "Todo" {
				println!("Please write your task");
				let mut task = read_line();
				if task.is_empty() {
					println!("Problem, quiting the prog");
					return Command::Quit;
				}
				task = task.trim_end().to_string();
				return Command::Todo(task);
			}
			if input == "Purge" { return Command::Purge; }
			if input == "Done" {
				println!("Please input the index of the done task");
				let nbr = read_number();
				if nbr < 0 {
					return Command::Done(std::usize::MAX);
				}
				return Command::Done(nbr as usize);
			}
			println!("Only command Avaible = Quit(CTRL + D), Purge, Add, Done");
		}
	}
}

#[allow(dead_code)]
struct TodoList {
	todos: Vec<String>,
	dones: Vec<String>,
}

#[allow(dead_code)]
impl TodoList {
	fn new() -> Self {
		Self { todos: Vec::new(), dones: Vec::new() }
	}
	fn display(&self) {
		let mut index = 0;
		for task in &self.todos {
			println!("[{}] = {}", index, task);
			index +=1;
		}
	}
	fn add(&mut self, todo: String) {
		self.todos.push(todo);
	}
	fn done(&mut self, index: usize) {
		self.dones.push(self.todos[index].clone(),);
		self.todos.remove(index);
	}
	fn purge(&mut self) {
		self.todos.clear();
	}
}

fn main() {

	let mut tdlist = TodoList::new();

	loop {
		tdlist.display();
		println!("Please input your command");
		let cmd = Command::prompt();
		match cmd {
			Command::Quit => break,
			Command::Purge=> tdlist.purge(),
			Command::Done(index) => tdlist.done(index),
			Command::Todo(task) => tdlist.add(task),
		};
	}
}
