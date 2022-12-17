mod converter;
mod driver_impl;
#[cfg(test)]
mod tests;

use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::{str, vec};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    Todo,
    Eof,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct AST {
    list: Vec<AstVariants>,
}

impl AST {
    fn new() -> Self {
        AST { list: vec![] }
    }
    fn push(&mut self, variant: AstVariants) {
        self.list.push(variant);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum AstVariants {
    Function(Function),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CommentBlock(String);

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

impl Display for Other {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FunctionNameWithParams {
    name: String,
    params: Vec<FunctionParam>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Marker {
    Recognized(SpecialMarker),
    // TODO: Should this be `Other`?
    Other(Other),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum SpecialMarker {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Struct(String);

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: Handle pointers and consts.
#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Primitive(Primitive),
    Struct(Struct),
    Custom(Other),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Function {
    name: String,
    params: Vec<FunctionParam>,
    return_ty: Type,
    markers: Vec<Marker>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FunctionParam {
    name: String,
    ty: Type,
    markers: Vec<Marker>,
}

trait Driver {
    type Parsed;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed>;
}

impl<'a> From<&'a str> for Walker<&'a [u8]> {
    fn from(buffer: &'a str) -> Self {
        Walker::new(buffer.as_bytes())
    }
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

        self.last_read_amt = counter;

        if !eof_ok && !completed {
            return Err(Error::Eof);
        }

        Ok(&decoded[..counter])
    }
    // Convenience method.
    fn read_until(&mut self, token: char) -> Result<&str> {
        self.read_until_fn(|char| char == token, false)
    }
    // Convenience method.
    fn read_until_separator(&mut self) -> Result<&str> {
        self.read_until_fn(|char| char == ' ' || char == '\n', true)
    }
    // Convenience method.
    fn read_eof(&mut self) -> Result<&str> {
        self.read_until_fn(|_| false, true)
    }
    // TODO: Maybe rename this, given that it does not consume the reader.
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
    // Convenience method.
    fn ensure_consume_fn<F>(&mut self, custom: F, ensure: EnsureVariant) -> Result<usize>
    where
        F: Fn(char) -> bool,
    {
        let amt = self.ensure_fn(custom, ensure)?;
        self.reader.consume(amt);
        Ok(amt)
    }
    // Convenience method.
    fn ensure_eof(&mut self) -> Result<()> {
        let read = self.read_until_fn(|_| false, true)?;
        if read.is_empty() {
            Ok(())
        } else {
            Err(Error::Todo)
        }
    }
    // Convenience method.
    fn ensure_separator(&mut self) -> Result<()> {
        let amt = self.ensure_fn(
            |char| char == ' ' || char == '\n',
            EnsureVariant::AtLeast(1),
        )?;
        self.reader.consume(amt);
        Ok(())
    }
    // Convenience method.
    fn ensure_one_semicolon(&mut self) -> Result<()> {
        let amt = self.ensure_fn(|char| char == ';', EnsureVariant::Exactly(1))?;
        self.reader.consume(amt);
        Ok(())
    }
    // Consume reader and move on with the next data.
    fn next(&mut self) {
        self.reader.consume(self.last_read_amt);
        self.last_read_amt = 0;
    }
}

enum EnsureVariant {
    AtLeast(usize),
    Exactly(usize),
}

use std::fs::File;

struct Engine {
    // TODO: Use `Path`
    paths: Vec<String>,
}

impl Engine {
    // TODO: Take `Path` here.
    fn new_path(path: &str) -> Self {
        // TODO: Maybe should check whether the path actually exists.
        Engine {
            paths: vec![path.to_string()],
        }
    }
    fn start(&self) -> Result<()> {
        for path in &self.paths {
            let file = File::open(path).unwrap();
            let mut walker = Walker::new(file);
            let ast = AST::drive(&mut walker)?;

            dbg!(&ast);
            let out = converter::rust::convert(&ast);
            println!("{out}");
        }

        Ok(())
    }
}

#[test]
fn test_engine() {
    let engine = Engine::new_path("../include/TrustWalletCore/TWString.h");
    engine.start().unwrap();
}
