enum Error {}
type Result<T> = std::result::Result<T, Error>;

enum AST {
    Include,
    Marker,
    Function,
}

struct Primitive;
struct Typedef;
struct Include;
struct Function;
struct Other;

trait Driver {
    type Parsed;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed>;
}

impl Driver for Function {
    type Parsed = ();

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let keyword = walker.read_keyword();

        if let Ok(x) = Primitive::drive(walker) {

        } else if let Ok(x) = Other::drive(walker) {

        }

        todo!()
    }
}

impl Driver for Other {
    type Parsed = ();

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for Primitive {
    type Parsed = ();

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for AST {
    type Parsed = ParsedAST;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let keyword = walker.read_keyword();
        let amt_read = keyword.len();

        match keyword {
            "#pragma" => {
                todo!()
            },
            "include" => {
                todo!()
            }
            _ => {
                walker.consume(amt_read);

                if let Ok(_parsed) = Function::drive(walker) {
                    Ok(ParsedAST::Function)
                } else {
                    // TODO: Error
                    todo!()
                }
            },
        }
    }
}

enum ParsedAST {
    Function
}

use std::io::{Read, BufRead, BufReader};
use std::str;

struct Walker<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> Walker<R> {
    fn read_keyword(&mut self) -> &str {
        let buffer = self.reader.fill_buf().unwrap();
        let decoded = str::from_utf8(buffer).unwrap();

        let mut completed = false;
        let mut to_take = 1;
        for char in decoded.chars() {
            if char != ' ' || char != '\n' {
                completed = true;
                break;
            }

            to_take += 1;
        }

        if !completed {
            // TODO: Error
        }

        &decoded[..to_take]
    }
    fn read_until(&mut self, chars: &[char]) -> &str {
        let buffer = self.reader.fill_buf().unwrap();
        let decoded = str::from_utf8(buffer).unwrap();

        let mut completed = false;
        let mut to_take = 1;
        for char in decoded.chars() {
            if chars.contains(&char) {
                completed = true;
                break;
            }

            to_take += 1;
        }

        if !completed {
            // TODO: Error
        }

        &decoded[..to_take]
    }
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt);
    }
    fn read_parentheses(&mut self) -> &str {
        todo!()
    }
}
