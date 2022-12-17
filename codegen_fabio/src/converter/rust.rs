use std::primitive;

use crate::{AstVariants, Primitive, Type, AST};

fn type_to_str(ty: &Type) -> &'static str {
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
        Type::Custom(_) => "*const u8",
        Type::Struct(_) => "*const u8",
    }
}

pub(crate) fn convert(ast: &AST) -> String {
    let mut out = String::new();

    // Prepare "extern C" block
    out.push_str("extern \"C\" {\n");

    for entry in &ast.list {
        if let AstVariants::Function(func) = entry {
            let mut params: String = func
                .params
                .iter()
                .map(|param| format!("{}: {},", param.name, type_to_str(&param.ty)))
                .collect();

			// Remove trailing comma.
			params.pop();

            out.push_str(&format!("    fn {}({}) -> {}\n", func.name, params, type_to_str(&func.return_ty)));
        }
    }

    out.push_str("}\n");

	out
}
