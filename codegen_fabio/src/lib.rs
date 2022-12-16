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
struct Other;
// TODO: Rename this
struct FunctionParams;

enum Type {
    Primitive(Primitive),
    Custom(Other),
}

struct Function {
    name: String,
    params: Vec<FunctionParam>,
    return_ty: Type,
}

struct FunctionParam {
    name: String,
    ty: (),
    markers: Vec<()>,
}

trait Driver {
    type Parsed;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed>;
}

impl Driver for FunctionParams {
    type Parsed = Vec<FunctionParam>;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for Function {
    type Parsed = ();

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let return_ty = if let Ok(primitive) = Primitive::drive(walker) {
            Type::Primitive(primitive)
        } else if let Ok(other) = Other::drive(walker) {
            Type::Custom(other)
        } else {
            panic!()
        };

        walker.next();
        walker.ensure_space();

        let mut markers = vec![];
        let params;

        loop {
            if let Ok(p) = FunctionParams::drive(walker) {
                params = p;
                break;
            } else if let Ok(other) = Other::drive(walker) {
                markers.push(other);
            }
        }

        walker.next();

        todo!()
    }
}

impl Driver for Other {
    type Parsed = Other;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for Primitive {
    type Parsed = Primitive;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for AST {
    type Parsed = ParsedAST;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let token = walker.read_until_separator();
        let amt_read = token.len();

        match token {
            "#pragma" => {
                todo!()
            }
            "include" => {
                todo!()
            }
            _ => {
                walker.next();

                if let Ok(_parsed) = Function::drive(walker) {
                    Ok(ParsedAST::Function)
                } else {
                    // TODO: Error
                    todo!()
                }
            }
        }
    }
}

enum ParsedAST {
    Function,
}

use std::io::{BufRead, BufReader, Read};
use std::str;

struct Walker<R: Read> {
    reader: BufReader<R>,
    last_read_amt: usize,
}

impl<R: Read> Walker<R> {
    fn read_until(&mut self, token: char) -> &str {
        let buffer = self.reader.fill_buf().unwrap();
        let decoded = str::from_utf8(buffer).unwrap();

        let mut completed = false;
        let mut to_take = 1;
        for char in decoded.chars() {
            if char == token {
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
    fn ensure_space(&mut self) {
        todo!()
    }
    fn read_until_non_alphanumeric(&mut self) {
        todo!()
    }
    fn read_until_separator(&mut self) -> &str {
        todo!()
    }
    fn next(&mut self) {
        self.reader.consume(self.last_read_amt);
    }
}
