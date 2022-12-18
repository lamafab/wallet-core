use crate::{Error, Walker};

#[test]
fn read_until() {
    let sample = "this is\nsome\ndata\n";
    let mut walker = Walker::new(sample.as_bytes());

    // Returns the value multiple times until `next` is called.
    assert_eq!("this is", walker.read_until('\n').unwrap());
    assert_eq!("this is", walker.read_until('\n').unwrap());
    assert_eq!("this is", walker.read_until('\n').unwrap());
    walker.next();

    assert_eq!("some", walker.read_until('\n').unwrap());
    walker.next();

    assert_eq!("data", walker.read_until('\n').unwrap());
    walker.next();
}

#[test]
fn read_until_token_first() {
    let sample = " some\n";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("some", walker.read_until_separator().unwrap());
    walker.next();

    // Returns empty value after
    assert_eq!("", walker.read_until_separator().unwrap());
    walker.next();
}

#[test]
fn read_until_eof_error() {
    let sample = "this is some data";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!(Error::Eof, walker.read_until('\n').unwrap_err());
}
