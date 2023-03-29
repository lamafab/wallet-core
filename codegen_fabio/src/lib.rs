mod converter;
mod driver_impl;
#[cfg(test)]
mod tests;

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, Read};
use std::str::{self, Utf8Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Todo,
    Eof,
    Io(IoError),
    Utf8Error(Utf8Error),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Ast {
    list: Vec<AstVariants>,
}

impl Ast {
    fn new() -> Self {
        Ast { list: vec![] }
    }
    fn push(&mut self, variant: AstVariants) {
        self.list.push(variant);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum AstVariants {
    Function(Function),
    Other(Other),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CommentBlock(String);

// TODO: Not fully complete yet.
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct FunctionNameWithParams {
    name: String,
    params: Vec<FunctionParam>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Marker {
    Other(String),
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
    // TODO: Const should not be implemented for each type variant individually.
    ConstPrimitive(Primitive),
    ConstOther(Other),
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Other(String);

impl Display for Other {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn start_parser(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut walker = Walker::new(file);
    let ast = Ast::drive(&mut walker)?;

    Ok(converter::rust::convert(&ast))
}

/// Implementors of this trait consume data from the `Walker` and use the
/// provided information to generate the parsed type (or might fail trying so).
/// Type implementations are in the 'driver_impl' module.
trait Driver {
    type Parsed;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed>;
}

impl<'a> From<&'a str> for Walker<&'a [u8]> {
    fn from(buffer: &'a str) -> Self {
        Walker::new(buffer.as_bytes())
    }
}

/// Low-level reader over a stream of bytes. This is primarily how a `Driver`
/// consumes data.
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
        let buffer = self.reader.fill_buf()?;
        let decoded = str::from_utf8(buffer)?;

        let mut completed = false;
        let mut content_reached = false;
        let mut counter = 0;
        for char in decoded.chars() {
            // Explicitly ignore leading spaces/newlines.
            if !content_reached && (char == ' ' || char == '\n') {
                counter += 1;
                continue;
            }

            content_reached = true;
            counter += char.len_utf8();

            if custom(char) {
                completed = true;
                break;
            }
        }

        self.last_read_amt = counter;

        if !eof_ok && !completed {
            return Err(Error::Eof);
        }

        // Return read content and remove remaining space/newline (if used in `custom`).
        Ok(decoded[..counter].trim())
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
    // Convenience method.
    fn ensure_eof(&mut self) -> Result<()> {
        let read = self.read_until_fn(|_| false, true)?;
        if read.is_empty() {
            Ok(())
        } else {
            Err(Error::Todo)
        }
    }
    // Consume reader and move on with the next data.
    fn next(&mut self) {
        self.reader.consume(self.last_read_amt);
        self.last_read_amt = 0;
    }
}

struct WalkerTwo<R: Read> {
    reader: BufReader<R>,
    amt_read: usize,
}

impl<R: Read> WalkerTwo<R> {
    pub fn new(reader: R) -> Self {
        WalkerTwo {
            reader: BufReader::new(reader),
            amt_read: 0,
        }
    }
    pub fn read_keyword(&mut self) -> Result<()> {
        let keyword = self.read_until_fn(|char| char == ' ' || char == '\n')?;
        todo!()
    }
    pub fn read_until(&mut self, token: char) -> Result<Option<(usize, &str)>> {
        self.read_until_fn(|char| char == token)
    }
    pub fn read_until_fn<F>(&mut self, custom: F) -> Result<Option<(usize, &str)>>
    where
        F: Fn(char) -> bool,
    {
        self.reader.consume(self.amt_read);

        let reader_buf = self.reader.fill_buf()?;
        let decoded = str::from_utf8(reader_buf)?;

        let pos = decoded
            .char_indices()
            .find(|(_, char)| custom(*char))
            .map(|(pos, _)| pos);

        if pos.is_none() {
            return Ok(None);
        }

        let pos = pos.unwrap() + 1;
        self.amt_read = pos;

        // Return read content and remove remaining space/newline (if used in `custom`).
        Ok(Some((pos, &decoded[..pos])))
    }
    pub fn read_until_one_of(&mut self, tokens: &[char]) -> Result<Option<(usize, &str)>> {
        self.read_until_fn(|char| tokens.contains(&char))
    }
}

#[test]
#[ignore]
fn test_parser() {
    let out = start_parser("../include/TrustWalletCore/TWString.h").unwrap();
    println!("{out}");
}
