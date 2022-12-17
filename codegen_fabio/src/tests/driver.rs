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

    // Stuck at newline
    assert_eq!("", walker.read_until('\n').unwrap());
    walker.next();

    assert_eq!("", walker.read_until('\n').unwrap());

    // Consume newline.
    walker.ensure_separator().unwrap();

    assert_eq!("some", walker.read_until('\n').unwrap());
    walker.next();

    // Consume newline.
    walker.ensure_separator().unwrap();

    assert_eq!("data", walker.read_until('\n').unwrap());
    walker.next();
}

#[test]
fn read_until_token_first() {
    let sample = " some\n";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_separator().unwrap();

    assert_eq!("some", walker.read_until_separator().unwrap());
    walker.next();
}

#[test]
fn read_until_eof_error() {
    let sample = "this is some data";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!(Error::Eof, walker.read_until('\n').unwrap_err());
}

#[test]
fn ensure_one_semicolon() {
    let sample = "this;is;;some";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("this", walker.read_until(';').unwrap());
    walker.next();
    walker.ensure_one_semicolon().unwrap();

    assert_eq!("is", walker.read_until(';').unwrap());
    walker.next();
    walker.ensure_one_semicolon().unwrap();

    // Stuck at second semicolon
    assert_eq!("", walker.read_until(';').unwrap());
}

#[test]
fn ensure_separator() {
    let sample = "this\nis\n\nsome\n\n\ndata\n";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("this", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_separator().unwrap();

    assert_eq!("is", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_separator().unwrap();

    assert_eq!("some", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_separator().unwrap();

    assert_eq!("data", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_separator().unwrap();
}
