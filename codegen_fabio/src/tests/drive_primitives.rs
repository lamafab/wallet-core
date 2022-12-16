use crate::{Driver, Error, Primitive, Walker};

// TODO: Test invalids
// TODO: This should re-init `sample`, cleaner to read.
#[test]
fn drive_primitive() {
    let sample = "char unsigned char int unsigned int short unsigned short long unsigned long bool";
    let mut walker = Walker::new(sample.as_bytes());

    // Char
    assert_eq!(Primitive::Char, Primitive::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

    // UnsignedChar
    assert_eq!(
        Primitive::UnsignedChar,
        Primitive::drive(&mut walker).unwrap()
    );
    walker.ensure_space().unwrap();

    // Int
    assert_eq!(Primitive::Int, Primitive::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

    // UnsignedInt
    assert_eq!(
        Primitive::UnsignedInt,
        Primitive::drive(&mut walker).unwrap()
    );
    walker.ensure_space().unwrap();

    // Short
    assert_eq!(Primitive::Short, Primitive::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

    // UnsignedShort
    assert_eq!(
        Primitive::UnsignedShort,
        Primitive::drive(&mut walker).unwrap()
    );
    walker.ensure_space().unwrap();

    // Long
    assert_eq!(Primitive::Long, Primitive::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

    // UnsignedLong
    assert_eq!(
        Primitive::UnsignedLong,
        Primitive::drive(&mut walker).unwrap()
    );
    walker.ensure_space().unwrap();

    // Bool
    assert_eq!(Primitive::Bool, Primitive::drive(&mut walker).unwrap());
}
