mod driver_impl;
#[cfg(test)]
mod tests;

use std::io::{BufRead, BufReader, Read};
use std::str;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    Todo,
    Eof,
}
type Result<T> = std::result::Result<T, Error>;

enum AST {
    Include,
    Marker,
    Function,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Other(String);
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Struct(String);

// TODO: Handle pointers.
#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Primitive(Primitive),
    Struct(Struct),
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

enum ParsedAST {
    Function,
}

struct Walker<R: Read> {
    reader: BufReader<R>,
    last_read_amt: usize,
}

impl<R: Read> Walker<R> {
    pub fn new(reader: R) -> Self {
        Walker {
            reader: BufReader::new(reader),
            last_read_amt: 0,
        }
    }
    fn read_until_fn<F>(&mut self, custom: F, eof_ok: bool) -> Result<&str>
    where
        F: Fn(char) -> bool,
    {
        let buffer = self.reader.fill_buf().unwrap();
        let decoded = str::from_utf8(buffer).unwrap();

        let mut completed = false;
        let mut counter = 0;
        for char in decoded.chars() {
            if custom(char) {
                completed = true;
                break;
            }

            counter += 1;
        }

        if !eof_ok && !completed {
            return Err(Error::Eof);
        }

        self.last_read_amt = counter;

        Ok(&decoded[..counter])
    }
    fn read_until(&mut self, token: char) -> Result<&str> {
        self.read_until_fn(|char| char == token, false)
    }
    // TODO: Is this needed?
    fn read_until_non_alphanumeric(&mut self) -> Result<&str> {
        self.read_until_fn(|char| !char.is_alphanumeric(), false)
    }
    // TODO: Write more tests for this.
    fn read_until_separator(&mut self) -> Result<&str> {
        self.read_until_fn(|char| char == ' ' || char == '\n', true)
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
                break;
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
    fn ensure_separator(&mut self) -> Result<()> {
        let amt = self.ensure_fn(|char| char == ' ' || char == '\n', EnsureVariant::AtLeast(1))?;
        self.reader.consume(amt);
        Ok(())
    }
    fn ensure_one_semicolon(&mut self) -> Result<()> {
        let amt = self.ensure_fn(|char| char == ';', EnsureVariant::Exactly(1))?;
        self.reader.consume(amt);
        Ok(())
    }
    // TODO: Is this needed?
    fn ensure_newline(&mut self) -> Result<()> {
        let amt = self.ensure_fn(|char| char == '\n', EnsureVariant::AtLeast(1))?;
        self.reader.consume(amt);
        Ok(())
    }
    // TODO: Test multiple calls.
    fn next(&mut self) {
        self.reader.consume(self.last_read_amt);
        self.last_read_amt = 0;
    }
}

enum EnsureVariant {
    AtLeast(usize),
    Exactly(usize),
}
