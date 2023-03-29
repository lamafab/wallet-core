use crate::{
    Driver, Function, FunctionNameWithParams, FunctionParam, Marker, Other, Primitive, Type, Walker,
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
                markers: vec![Marker::Other("_Nonnull".to_string())],
            },
            FunctionParam {
                name: "some".to_string(),
                ty: Type::Primitive(Primitive::Bool),
                markers: vec![],
            },
        ],
    };

    let sample = "some_func(int _Nonnull my_var, bool some)";
    let mut walker = Walker::from(sample);
    assert_eq!(
        expected,
        FunctionNameWithParams::drive(&mut walker).unwrap()
    );
}

#[test]
fn drive_function_full() {
    let expected = Function {
        name: "TWStringGet".to_string(),
        params: vec![
            FunctionParam {
                name: "string".to_string(),
                ty: Type::Custom(Other("TWString*".to_string())),
                markers: vec![Marker::Other("_Nonnull".to_string())],
            },
            FunctionParam {
                name: "index".to_string(),
                ty: Type::Custom(Other("size_t".to_string())),
                markers: vec![],
            },
        ],
        return_ty: Type::Primitive(Primitive::Char),
        markers: vec![Marker::Other("TW_VISIBILITY_DEFAULT".to_string())],
    };

    let sample = "char TWStringGet(TWString* _Nonnull string, size_t index) TW_VISIBILITY_DEFAULT;";
    let mut walker = Walker::from(sample);
    assert_eq!(expected, Function::drive(&mut walker).unwrap());
}
