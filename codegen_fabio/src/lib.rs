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
struct FunctionParams {
    name: String,
    params: Vec<FunctionParam>,
}

enum Marker {
    Recognized(SpecialMarker),
    Other(Other)
}

impl Driver for Marker {
    type Parsed = Self;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

enum SpecialMarker {

}

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
    type Parsed = Self;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for Function {
    type Parsed = Function;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // Parse return value.
        let return_ty = if let Ok(primitive) = Primitive::drive(walker) {
            Type::Primitive(primitive)
        } else if let Ok(other) = Other::drive(walker) {
            Type::Custom(other)
        } else {
            panic!()
        };

        walker.next();

        // Expect space.
        walker.ensure_space();

        // Check for possible markers, function name and function params.
        let mut markers = vec![];
        let name;
        let params;

        loop {
            if let Ok(f) = FunctionParams::drive(walker) {
                name = f.name;
                params = f.params;
                break;
            } else if let Ok(other) = Marker::drive(walker) {
                markers.push(other);
            }
        }

        walker.next();

        // Check for possible marker
        if let Ok(marker) = Marker::drive(walker) {
            markers.push(marker);
            walker.next();
        }

        // Expect semicolon.
        walker.ensure_semicolon();

        Ok(
            Function {
                name,
                params,
                return_ty,
            }
        )
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
    fn ensure_semicolon(&mut self) {
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
