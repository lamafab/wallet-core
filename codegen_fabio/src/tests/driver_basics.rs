use crate::{Walker, WalkerTwo};

#[test]
fn www_read_keyword() {
    let sample = "this is\nsome\n $data\n";
    let mut walker = WalkerTwo::new(sample.as_bytes());

    assert_eq!("this", walker.read_keyword().unwrap().unwrap());
    assert_eq!("is", walker.read_keyword().unwrap().unwrap());
    assert_eq!("some", walker.read_keyword().unwrap().unwrap());
    assert_eq!("$data", walker.read_keyword().unwrap().unwrap());
}

#[test]
fn www_read_until() {
    let sample = "this is\nsome\n $data\n";
    let mut walker = WalkerTwo::new(sample.as_bytes());

    assert_eq!("this is\n", walker.read_until('\n').unwrap().unwrap());
    assert_eq!("some\n", walker.read_until('\n').unwrap().unwrap());
    assert_eq!(" $data\n", walker.read_until('\n').unwrap().unwrap());
}

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
fn read_until_separator() {
    let sample = " some\n";
    let mut walker = Walker::new(sample.as_bytes());

    // `read_until_separator` allows EOF
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

    // Does not allow EOF
    assert!(walker.read_until('\n').is_err());
    walker.next();
}
