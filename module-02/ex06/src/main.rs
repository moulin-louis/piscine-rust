use ftkit::ARGS;

#[derive(Debug)]
enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

fn is_token(letter: char) -> bool {
    matches!(letter, '|' | '>' | '<')
}

fn next_token<'a>(s: &'a mut &str) -> Option<Token<'a>> {
    if s.is_empty() {
        return None;
    }
    let mut idx_cl: usize = 0;
    for (_idx, letter) in s.char_indices() {
        if letter.is_whitespace() {
            idx_cl += 1;
            continue;
        }
        break;
    }
    *s = &s[idx_cl..s.len()];
    if s.is_empty() {
        return None;
    }
    let mut idx_end: usize = 0;
    for (idx, letter) in s.char_indices() {
        if is_token(letter) || letter.is_whitespace() {
            break;
        }
        idx_end = idx;
    }
    idx_end += 1;
    let result: &str = &s[0..idx_end];
    *s = &s[idx_end..s.len()];
    match result {
        "|" => Some(Token::Pipe),
        ">" => Some(Token::RedirectStdout),
        "<" => Some(Token::RedirectStdin),
        _ => Some(Token::Word(result)),
    }
}

fn print_all_tokens(mut s: &str) {
    while let Some(token) = next_token(&mut s) {
        println!("{token:?}");
    }
    println!("None");
}

fn main() {
    if ARGS.len() != 2 {
        eprintln!("Error: exactly one argument expected");
        eprintln!("Usage: cargo run -- \"insert command here\" ");
        return;
    }
    print_all_tokens(&ARGS[1]);
}
