use crate::{Driver, Error, FunctionNameWithParams, Other, Primitive, Struct, Type, Walker};

#[test]
fn drive_function_name_with_params() {
    let sample = "some_func(int my_var, bool some)";
    let mut walker = Walker::new(sample.as_bytes());

    let x = FunctionNameWithParams::drive(&mut walker).unwrap();
    println!(">> {:?}", x);
}
