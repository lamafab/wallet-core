use crate::{Walker, WalkerTwo};

#[test]
fn read_keyword() {
    let sample = "this is\nsome\n $data\n";
    let mut walker = WalkerTwo::new(sample.as_bytes());

    assert_eq!("this", walker.read_keyword().unwrap().unwrap());
    assert_eq!("is", walker.read_keyword().unwrap().unwrap());
    assert_eq!("some", walker.read_keyword().unwrap().unwrap());
    assert_eq!("$data", walker.read_keyword().unwrap().unwrap());

    assert!(walker.read_keyword().unwrap().is_none());
}

#[test]
fn read_until() {
    let sample = "this is\nsome\n $data\n";
    let mut walker = WalkerTwo::new(sample.as_bytes());

    assert_eq!("this is\n", walker.read_until('\n').unwrap().unwrap());
    assert_eq!("some\n", walker.read_until('\n').unwrap().unwrap());
    assert_eq!(" $data\n", walker.read_until('\n').unwrap().unwrap());

    assert!(walker.read_keyword().unwrap().is_none());
}

#[test]
fn read_until_one_of() {
    let sample = "this is\nsome\n $data\n";
    let mut walker = WalkerTwo::new(sample.as_bytes());

    assert_eq!(
        "this ",
        walker.read_until_one_of(&[' ', '\n']).unwrap().unwrap()
    );
    assert_eq!(
        "is\n",
        walker.read_until_one_of(&[' ', '\n']).unwrap().unwrap()
    );
    assert_eq!(
        "some\n",
        walker.read_until_one_of(&[' ', '\n']).unwrap().unwrap()
    );
    assert_eq!(
        " ",
        walker.read_until_one_of(&[' ', '\n']).unwrap().unwrap()
    );
    assert_eq!(
        "$data\n",
        walker.read_until_one_of(&[' ', '\n']).unwrap().unwrap()
    );

    assert!(walker.read_keyword().unwrap().is_none());
}

#[test]
fn read_until_eof() {
    let sample = "";
    let mut walker = WalkerTwo::new(sample.as_bytes());

    assert!(walker.read_until(' ').unwrap().is_none());
}
