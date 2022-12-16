use crate::{Error, Walker};

#[test]
fn read_until() {
    let sample = "this is\nsome\ndata\n";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("this is", walker.read_until('\n').unwrap());
    assert_eq!("this is", walker.read_until('\n').unwrap());
    walker.next();

    // Stuck at newline
    assert_eq!("", walker.read_until('\n').unwrap());
    walker.next();

    assert_eq!("", walker.read_until('\n').unwrap());

    // Consume newline.
    walker.ensure_newline().unwrap();

    assert_eq!("some", walker.read_until('\n').unwrap());
    walker.next();

    // Consume newline.
    walker.ensure_newline().unwrap();

    // TODO: Should this return an error?
    assert_eq!("data", walker.read_until('\n').unwrap());
    walker.next();
}

#[test]
fn read_until_eof_erro() {
    let sample = "this is some data";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!(Error::Eof, walker.read_until('\n').unwrap_err());
}

#[test]
fn ensure_space() {
    let sample = "this is  some   data ";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("this", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_space().unwrap();

    assert_eq!("is", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_space().unwrap();

    assert_eq!("some", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_space().unwrap();

    assert_eq!("data", walker.read_until_separator().unwrap());
    walker.next();
    walker.ensure_space().unwrap();
}
