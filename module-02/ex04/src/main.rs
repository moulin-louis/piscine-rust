use ftkit::read_line;

enum Command {
    Todo(String),
    Done(usize),
    Purge,
    Quit,
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

fn find_str(str: &String, sep: char) -> Option<usize> {
    for idx in 0..str.len() {
        if str.as_str().as_bytes()[idx] == sep as u8 {
            return Some(idx);
        }
    }
    None
}

impl Command {
    fn prompt() -> Self {
        let mut input: String = read_line();
        if input.is_empty() {
            return Command::Quit;
        }
        input.pop();
        println!("input len = {}", input.len());
        if input.len() < 4 {
            eprintln!("Invalid Format/Command");
            return Command::Quit;
        }
        let pos_space: usize = match find_str(&input, ' ') {
            None => input.len(),
            Some(pos) => pos,
        };
        let command: &str = &input.as_str()[0..pos_space];
        return match command {
            "TODO" => {
                let args: &str = &input.as_str()[pos_space..input.len()];
                Command::Todo(args.to_string())
            }
            "DONE" => {
                let arg: &str = &input.as_str()[(pos_space + 1)..input.len()];
                let idx: usize = match arg.to_string().parse() {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("{e}");
                        return Command::Quit;
                    }
                };
                Command::Done(idx)
            }
            "PURGE" => Command::Purge,
            "QUIT" => Command::Quit,
            _ => {
                eprintln!("Wrong Command ! Aborting...");
                Command::Quit
            }
        };
    }
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            dones: Vec::new(),
        }
    }

    fn display(&self) {
        for idx in 0..self.todos.len() {
            println!("{idx} [ ] {}", self.todos[idx]);
        }
        for idx in 0..self.dones.len() {
            println!("{} [X] {}", self.todos.len() + idx, self.dones[idx]);
        }
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo);
        self.display();
    }

    fn done(&mut self, index: usize) {
        if index > self.todos.len() {
            eprintln!("Wrong index!");
            return;
        }
        let tmp: String = self.todos[index].clone();
        self.todos.remove(index);
        self.dones.push(tmp);
        self.display();
    }

    fn purge(&mut self) {
        self.dones.clear();
        self.display();
    }
}

fn main() {
    let mut todos: TodoList = TodoList::new();
    loop {
        let command: Command = Command::prompt();
        match command {
            Command::Quit => break,
            Command::Purge => {
                todos.purge();
            }
            Command::Done(idx) => todos.done(idx),
            Command::Todo(todo) => todos.add(todo),
        }
    }
}
