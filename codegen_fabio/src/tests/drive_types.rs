use crate::{Driver, Error, Other, Primitive, Struct, Type, Walker};

#[test]
fn drive_other_token_strictness_check() {
    let mut walker = Walker::from("some ");
    assert!(Other::drive(&mut walker).is_err());

    let mut walker = Walker::from(" some");
    assert!(Other::drive(&mut walker).is_err());

    let mut walker = Walker::from("some other");
    assert!(Other::drive(&mut walker).is_err());
}

#[test]
fn drive_other_valid_drives() {
    let mut walker = Walker::from("this");
    assert_eq!(
        Other("this".to_string()),
        Other::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("_is");
    assert_eq!(Other("_is".to_string()), Other::drive(&mut walker).unwrap());

    let mut walker = Walker::from("some-data");
    assert_eq!(
        Other("some-data".to_string()),
        Other::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("-to");
    assert_eq!(Other("-to".to_string()), Other::drive(&mut walker).unwrap());

    let mut walker = Walker::from("8_parse");
    assert_eq!(
        Other("8_parse".to_string()),
        Other::drive(&mut walker).unwrap()
    );
}

#[test]
fn drive_types_token_strictness_check() {
    let mut walker = Walker::from("int ");
    assert!(Type::drive(&mut walker).is_err());

    let mut walker = Walker::from(" char");
    assert!(Type::drive(&mut walker).is_err());

    let mut walker = Walker::from("int char");
    assert!(Type::drive(&mut walker).is_err());
}

#[test]
fn drive_type_valid_types() {
    let mut walker = Walker::from("int");
    assert_eq!(
        Type::Primitive(Primitive::Int),
        Type::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("unsigned char");
    assert_eq!(
        Type::Primitive(Primitive::UnsignedChar),
        Type::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("struct MyStruct");
    assert_eq!(
        Type::Struct(Struct("MyStruct".to_string())),
        Type::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("some-data");
    assert_eq!(
        Type::Custom(Other("some-data".to_string())),
        Type::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("bool");
    assert_eq!(
        Type::Primitive(Primitive::Bool),
        Type::drive(&mut walker).unwrap()
    );
}

#[test]
fn drive_primitives_token_strictness_check() {
    let mut walker = Walker::from("char ");
    assert!(Primitive::drive(&mut walker).is_err());

    let mut walker = Walker::from(" char");
    assert!(Primitive::drive(&mut walker).is_err());

    let mut walker = Walker::from("char int");
    assert!(Primitive::drive(&mut walker).is_err());

    // Some invalid types
    let mut walker = Walker::from("some");
    assert!(Primitive::drive(&mut walker).is_err());

    let mut walker = Walker::from("some");
    assert!(Primitive::drive(&mut walker).is_err());
}

#[test]
fn drive_primitives_valid_drives() {
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
