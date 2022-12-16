#[cfg(test)]
mod tests;

enum Error {
    Todo,
}
type Result<T> = std::result::Result<T, Error>;

enum AST {
    Include,
    Marker,
    Function,
}

enum Primitive {
    Char,
    UnsignedChar,
    Int,
    UnsignedInt,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    Bool,
}

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
    Other(Other),
}

impl Driver for Marker {
    type Parsed = Self;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

enum SpecialMarker {}

struct Struct(String);

impl Driver for Struct {
    type Parsed = Self;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

// TODO: Handle pointers.
enum Type {
    Primitive(Primitive),
    Struct(Struct),
    Custom(Other),
}

impl Driver for Type {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let ty = if let Ok(primitive) = Primitive::drive(walker) {
            Type::Primitive(primitive)
        } else if walker.read_until_separator() == "struct" {
            walker.next();
            Type::Struct(Struct::drive(walker)?)
        } else if let Ok(other) = Other::drive(walker) {
            Type::Custom(other)
        } else {
            panic!()
        };

        walker.next();

        Ok(ty)
    }
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
            } else {
                panic!()
            }

            walker.next();
            walker.ensure_space();
        }

        // Check for possible marker
        if let Ok(marker) = Marker::drive(walker) {
            markers.push(marker);
            walker.next();
        }

        // Expect semicolon.
        walker.ensure_semicolon();

        Ok(Function {
            name,
            params,
            return_ty,
        })
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

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let word = walker.read_until_non_alphanumeric();

        let primitive = match word {
            "unsigned" => {
                walker.next();

                if let Ok(primitive) = Primitive::drive(walker) {
                    match primitive {
                        Primitive::Char => Primitive::UnsignedChar,
                        Primitive::Int => Primitive::UnsignedInt,
                        Primitive::Short => Primitive::UnsignedShort,
                        Primitive::Long => Primitive::UnsignedLong,
                        Primitive::Bool => panic!(),
                        // Explicitly disallow all other.
                        _ => todo!(),
                    }
                } else {
                    panic!()
                }
            }
            "signed" => {
                walker.next();

                if let Ok(primitive) = Primitive::drive(walker) {
                    match primitive {
                        Primitive::Char | Primitive::Int | Primitive::Short | Primitive::Long => {
                            primitive
                        }
                        // Explicitly disallow all other.
                        _ => todo!(),
                    }
                } else {
                    panic!()
                }
            }
            "char" => Primitive::Char,
            "int" => Primitive::Int,
            "short" => Primitive::Short,
            "long" => Primitive::Long,
            _ => todo!(),
        };

        walker.next();

        Ok(primitive)
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
    fn read_until_fn<F>(&mut self, custom: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        let buffer = self.reader.fill_buf().unwrap();
        let decoded = str::from_utf8(buffer).unwrap();

        let mut completed = false;
        let mut to_take = 1;
        for char in decoded.chars() {
            if custom(char) {
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
    fn read_until(&mut self, token: char) -> &str {
        self.read_until_fn(|char| char == token)
    }
    fn read_until_non_alphanumeric(&mut self) -> &str {
        self.read_until_fn(|char| !char.is_alphanumeric())
    }
    fn read_until_separator(&mut self) -> &str {
        self.read_until_fn(|char| char == ' ' || char == '\n')
    }
    fn ensure_fn<F>(&mut self, custom: F, ensure: EnsureVariant) -> Result<usize>
    where
        F: Fn(char) -> bool,
    {
        let buffer = self.reader.fill_buf().unwrap();
        let decoded = str::from_utf8(buffer).unwrap();

        let mut counter = 0;
        for char in decoded.chars() {
            if !custom(char) {
                return Err(Error::Todo);
            }

            counter += 1;

            if let EnsureVariant::Exactly(exact) = ensure {
                if exact == counter {
                    return Ok(counter);
                }
            }
        }

        if let EnsureVariant::AtLeast(at_least) = ensure {
            if counter >= at_least {
                return Ok(counter);
            }
        }

        Err(Error::Todo)
    }
    fn ensure_space(&mut self) -> Result<()> {
        let amt = self.ensure_fn(|char| char == ' ', EnsureVariant::AtLeast(1))?;
        self.reader.consume(amt);
        Ok(())
    }
    fn ensure_semicolon(&mut self) -> Result<()> {
        let amt = self.ensure_fn(|char| char == ';', EnsureVariant::Exactly(1))?;
        self.reader.consume(amt);
        Ok(())
    }
    fn next(&mut self) {
        self.reader.consume(self.last_read_amt);
    }
}

enum EnsureVariant {
    AtLeast(usize),
    Exactly(usize),
}
