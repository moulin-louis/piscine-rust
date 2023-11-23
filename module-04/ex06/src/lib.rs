#[derive(PartialEq, Debug)]
pub struct FormatError;

type WriteFn = dyn FnMut(&[u8]) -> Result<(), FormatError>;

pub trait Print {
    fn print(&self, write: &mut WriteFn) -> Result<(), FormatError>;
}

impl Print for u32 {
    fn print(&self, write: &mut WriteFn) -> Result<(), FormatError> {
        write(self.to_string().as_bytes())
    }
}

impl Print for &str {
    fn print(&self, write: &mut WriteFn) -> Result<(), FormatError> {
        write(self.as_bytes())
    }
}

pub fn format_with<W>(
    str_to_format: &str,
    values: &[&dyn Print],
    write: W,
) -> Result<(), FormatError>
where
    W: FnOnce(&[u8]) -> Result<(), FormatError>,
{
    let mut str = str_to_format.as_bytes();
    let mut idx = 0;
    let mut idx_val: = 0;
    while idx < str.len() {
        if str[idx] == b'%' {
            str[idx] = values[idx_val].print()
        }
    }
    Ok(())
}

fn format_string(s: &str, values: &[&dyn Print]) -> Result<String, FormatError> {
    Ok(s.to_string())
}

fn format_print(s: &str, values: &[&dyn Print]) -> Result<(), FormatError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdout;
    use std::io::Write;

    fn test_write_fn(buf: &[u8]) -> Result<(), FormatError> {
        match stdout().write(buf) {
            Err(_e) => Err(FormatError),
            Ok(_x) => Ok(()),
        }
    }
    #[test]
    fn it_works() {
        let test: u32 = 42;
        assert_eq!(test.print(&mut test_write_fn), Ok(()));
    }

    #[test]
    fn mandatory_test() {
        let s: String = format_string("salut % les % gens!", &[&14u32, &"Hello!"]).unwrap();
        assert_eq!(s, "salut 14 les Hello! gens!");
    }
}
