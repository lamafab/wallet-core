use crate::{Driver, Error, Primitive, Walker, Other};

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