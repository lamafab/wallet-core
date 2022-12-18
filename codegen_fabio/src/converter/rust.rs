use crate::{AstVariants, Function, Primitive, Type, AST};

fn type_to_c_str(ty: &Type) -> &'static str {
    match ty {
        Type::Primitive(primitive) => match primitive {
            Primitive::Char => "libc::c_char",
            Primitive::UnsignedChar => "libc::c_uchar",
            Primitive::Int => "libc::c_int",
            Primitive::UnsignedInt => "libc::c_uint",
            Primitive::Short => "libc::c_short",
            Primitive::UnsignedShort => "libc::c_ushort",
            Primitive::Long => "libc::c_long",
            Primitive::UnsignedLong => "libc::c_ulong",
            Primitive::Bool => "bool",
        },
        Type::ConstPrimitive(primitive) => type_to_c_str(&Type::Primitive(primitive.clone())),
        Type::ConstOther(other) => type_to_c_str(&Type::Custom(other.clone())),
        Type::Custom(_) => "*const u8",
        Type::Struct(_) => "*const u8",
    }
}

// TODO: Check for relevant markers (e.g. `_NonNull` and adjust code accordingly).
fn convert_params_to_string(func: &Function) -> String {
    let mut params: String = func
        .params
        .iter()
        .map(|param| format!("{}: {}, ", param.name, type_to_c_str(&param.ty)))
        .collect();

    // Remove trailing comma and space.
    params.pop();
    params.pop();

    params
}

fn convert_param_names_only_to_string(func: &Function) -> String {
    let mut params: String = func
        .params
        .iter()
        .map(|param| format!("{}, ", param.name))
        .collect();

    // Remove trailing comma and space.
    params.pop();
    params.pop();

    params
}

// TODO: Consider using (Buf-)Write(r).
pub(crate) fn convert(ast: &AST) -> String {
    let mut out = String::new();

    // Prepare "extern C" block.
    out.push_str("extern \"C\" {\n");

    for entry in &ast.list {
        // Write extern interface for each block.
        if let AstVariants::Function(func) = entry {
            let params = convert_params_to_string(func);
            out.push_str(&format!(
                "    fn {func_name}({params}) -> {return_type}\n",
                func_name = func.name,
                return_type = type_to_c_str(&func.return_ty)
            ));
        }
    }

    out.push_str("}\n\n");

    // Write Rust function interfaces and bodies.
    for entry in &ast.list {
        // Write extern interface for each block.
        if let AstVariants::Function(func) = entry {
            let params = convert_params_to_string(func);
            out.push_str(&format!(
                "pub fn {func_name}({params}) -> {return_type} {{\n",
                func_name = func.name.to_lowercase(),
                return_type = type_to_c_str(&func.return_ty),
            ));

            let param_names = convert_param_names_only_to_string(func);
            out.push_str(&format!(
                "    unsafe {{ {func_name}({param_names}) }}\n",
                func_name = func.name
            ));

            out.push_str("}\n\n");
        }
    }

    out
}
