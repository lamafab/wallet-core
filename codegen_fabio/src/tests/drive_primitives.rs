use crate::{Driver, Error, Primitive, Walker};

#[test]
fn drive_primitives_token_strictness_check() {
    let mut walker = Walker::from(" char");
    assert!(Primitive::drive(&mut walker).is_err());

    let mut walker = Walker::from("char ");
    assert!(Primitive::drive(&mut walker).is_err());

    let mut walker = Walker::from("\nchar");
    assert!(Primitive::drive(&mut walker).is_err());

    let mut walker = Walker::from("char\n");
    assert!(Primitive::drive(&mut walker).is_err());
}

// TODO: Test invalids
// TODO: This should re-init `sample`, cleaner to read.
#[test]
fn drive_primitives() {
    // Char
    let mut walker = Walker::from("char");
    assert_eq!(Primitive::Char, Primitive::drive(&mut walker).unwrap());

    // UnsignedChar
    let mut walker = Walker::from("unsigned char");
    assert_eq!(
        Primitive::UnsignedChar,
        Primitive::drive(&mut walker).unwrap()
    );

    // Int
    let mut walker = Walker::from("int");
    assert_eq!(Primitive::Int, Primitive::drive(&mut walker).unwrap());

    // UnsignedInt
    let mut walker = Walker::from("unsigned int");
    assert_eq!(
        Primitive::UnsignedInt,
        Primitive::drive(&mut walker).unwrap()
    );

    // Short
    let mut walker = Walker::from("short");
    assert_eq!(Primitive::Short, Primitive::drive(&mut walker).unwrap());

    // UnsignedShort
    let mut walker = Walker::from("unsigned short");
    assert_eq!(
        Primitive::UnsignedShort,
        Primitive::drive(&mut walker).unwrap()
    );

    // Long
    let mut walker = Walker::from("long");
    assert_eq!(Primitive::Long, Primitive::drive(&mut walker).unwrap());

    // UnsignedLong
    let mut walker = Walker::from("unsigned long");
    assert_eq!(
        Primitive::UnsignedLong,
        Primitive::drive(&mut walker).unwrap()
    );

    // Bool
    let mut walker = Walker::from("bool");
    assert_eq!(Primitive::Bool, Primitive::drive(&mut walker).unwrap());
}

fn drive_primitives_invalid() {
    let mut walker = Walker::from("some_type");
    assert!(Primitive::drive(&mut walker).is_err());
}
