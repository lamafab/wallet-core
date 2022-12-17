use crate::{Driver, Error, Other, Primitive, Struct, Type, Walker, FunctionNameWithParams};

#[test]
fn drive_function_name_with_params() {
    let sample = "some_func(int my_var);";
    let mut walker = Walker::new(sample.as_bytes());

	let x = FunctionNameWithParams::drive(&mut walker).unwrap();
	println!(">> {:?}", x);
}
