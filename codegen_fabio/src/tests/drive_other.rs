use crate::{Driver, Error, Primitive, Walker, Other, Type, Struct};

#[test]
fn drive_other() {
    let sample = "this _is some-data -to parse_8";
    let mut walker = Walker::new(sample.as_bytes());

	assert_eq!(Other("this".to_string()), Other::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Other("_is".to_string()), Other::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Other("some-data".to_string()), Other::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Other("-to".to_string()), Other::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Other("parse_8".to_string()), Other::drive(&mut walker).unwrap());
}

#[test]
fn drive_type() {
    let sample = "int unsigned char struct MyStruct some-data bool";
    let mut walker = Walker::new(sample.as_bytes());

	assert_eq!(Type::Primitive(Primitive::Int), Type::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Type::Primitive(Primitive::UnsignedChar), Type::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Type::Struct(Struct("MyStruct".to_string())), Type::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Type::Custom(Other("some-data".to_string())), Type::drive(&mut walker).unwrap());
    walker.ensure_space().unwrap();

	assert_eq!(Type::Primitive(Primitive::Bool), Type::drive(&mut walker).unwrap());
}