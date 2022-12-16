use crate::Walker;

#[test]
fn read_until() {
    let sample = "this is\nsome\ndata";
    let mut walker = Walker::new(sample.as_bytes());

    assert_eq!("this is", walker.read_until('\n'));
    assert_eq!("this is", walker.read_until('\n'));
    walker.next();

	// Stuck at newline
    assert_eq!("", walker.read_until('\n'));
    walker.next();

    assert_eq!("", walker.read_until('\n'));

    // Consume newline.
    walker.ensure_newline().unwrap();

    assert_eq!("some", walker.read_until('\n'));
    walker.next();

    // Consume newline.
    walker.ensure_newline().unwrap();

    assert_eq!("data", walker.read_until('\n'));
    walker.next();
}
