use crate::{
    Driver, Error, Function, FunctionNameWithParams, FunctionParam, Other, Primitive, Struct, Type,
    Walker,
};

#[test]
fn drive_function_name_with_params() {
    let expected = FunctionNameWithParams {
        name: "some_func".to_string(),
        params: vec![
            FunctionParam {
                name: "my_var".to_string(),
                ty: Type::Primitive(Primitive::Int),
                markers: vec![],
            },
            FunctionParam {
                name: "some".to_string(),
                ty: Type::Primitive(Primitive::Bool),
                markers: vec![],
            },
        ],
    };

    let sample = "some_func(int my_var, bool some)";
    let mut walker = Walker::from(sample);
    assert_eq!(
        expected,
        FunctionNameWithParams::drive(&mut walker).unwrap()
    );

    let sample = "some_func (  int\nmy_var,\nbool some  \n)";
    let mut walker = Walker::from(sample);
    assert_eq!(
        expected,
        FunctionNameWithParams::drive(&mut walker).unwrap()
    );

    let sample = "some_func(int my_var, bool some) \n";
    let mut walker = Walker::from(sample);
    assert_eq!(
        expected,
        FunctionNameWithParams::drive(&mut walker).unwrap()
    );

    let sample = "\nsome_func(int my_var, bool some)";
    let mut walker = Walker::from(sample);
    assert_eq!(
        expected,
        FunctionNameWithParams::drive(&mut walker).unwrap()
    );
}

#[test]
fn drive_function_name_with_markers_params() {
    let expected = FunctionNameWithParams {
        name: "some_func".to_string(),
        params: vec![
            FunctionParam {
                name: "my_var".to_string(),
                ty: Type::Primitive(Primitive::Int),
                markers: vec![],
            },
            FunctionParam {
                name: "some".to_string(),
                ty: Type::Primitive(Primitive::Bool),
                markers: vec![],
            },
        ],
    };

    let sample = "some_func(int _NotNull my_var, bool some)";
    let mut walker = Walker::from(sample);
    println!(
        ">> {:?}",
        FunctionNameWithParams::drive(&mut walker).unwrap()
    );
}

#[test]
fn drive_function_full() {
    let sample = "const int* _NotNull some_func(int my_var, bool some) OtherMarker;";
    //let sample =
        //"bool TWStringEqual(const TWString* _Nonnull lhs, TWString* _Nonnull rhs) TW_VISIBILITY_DEFAULT;";
    let mut walker = Walker::from(sample);

    let res = Function::drive(&mut walker).unwrap();
    println!("{:?}", res);
}
