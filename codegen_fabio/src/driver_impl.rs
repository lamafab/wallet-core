use crate::{
    Driver, Function, FunctionParam, FunctionParams, Marker, Other, ParsedAST, Primitive, Result,
    Struct, Type, Walker, AST, Error,
};
use std::io::{BufRead, BufReader, Read};
use std::str;

impl Driver for Type {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let ty = if let Ok(primitive) = Primitive::drive(walker) {
            Type::Primitive(primitive)
        } else if walker.read_until_separator()? == "struct" {
            Type::Struct(Struct::drive(walker)?)
        } else if let Ok(other) = Other::drive(walker) {
            Type::Custom(other)
        } else {
            panic!()
        };

        walker.next();

        Ok(ty)
    }
}

impl Driver for FunctionParams {
    type Parsed = Self;

    fn drive<R: Read>(_: &mut Walker<R>) -> Result<Self::Parsed> {
        todo!()
    }
}

impl Driver for Function {
    type Parsed = Function;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // Parse return value.
        let return_ty = if let Ok(primitive) = Primitive::drive(walker) {
            Type::Primitive(primitive)
        } else if let Ok(other) = Other::drive(walker) {
            Type::Custom(other)
        } else {
            panic!()
        };

        walker.next();

        // Expect separator.
        walker.ensure_separator()?;

        // Check for possible markers, function name and function params.
        let mut markers = vec![];
        let name;
        let params;

        // TODO: Comment on behavior
        loop {
            if let Ok(f) = FunctionParams::drive(walker) {
                name = f.name;
                params = f.params;
                break;
            } else if let Ok(other) = Marker::drive(walker) {
                markers.push(other);
            } else {
                panic!()
            }

            walker.next();
            walker.ensure_separator()?;
        }

        // Check for possible marker
        if let Ok(marker) = Marker::drive(walker) {
            markers.push(marker);
            walker.next();
        }

        // Expect semicolon.
        walker.ensure_one_semicolon()?;

        Ok(Function {
            name,
            params,
            return_ty,
        })
    }
}

impl Driver for Other {
    type Parsed = Other;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
		// TODO: Should just use `read_until_separator`.
        let keyword = walker.read_until_fn(
            |char| !char.is_ascii_alphanumeric() && char != '_' && char != '-',
            true,
        )?;

        let other = Other(keyword.to_string());
        walker.next();

        Ok(other)
    }
}

impl Driver for Primitive {
    type Parsed = Primitive;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let word = walker.read_until_separator()?;

        let primitive = match word {
            "unsigned" => {
                walker.next();
                walker.ensure_separator()?;

                if let Ok(primitive) = Primitive::drive(walker) {
                    match primitive {
                        Primitive::Char => Primitive::UnsignedChar,
                        Primitive::Int => Primitive::UnsignedInt,
                        Primitive::Short => Primitive::UnsignedShort,
                        Primitive::Long => Primitive::UnsignedLong,
                        Primitive::Bool => panic!(),
                        // Explicitly disallow all other.
                        _ => todo!(),
                    }
                } else {
                    panic!()
                }
            }
            "signed" => {
                walker.next();
                walker.ensure_separator()?;

                if let Ok(primitive) = Primitive::drive(walker) {
                    match primitive {
                        Primitive::Char | Primitive::Int | Primitive::Short | Primitive::Long => {
                            primitive
                        }
                        // Explicitly disallow all other.
                        _ => todo!(),
                    }
                } else {
                    panic!()
                }
            }
            "char" => Primitive::Char,
            "int" => Primitive::Int,
            "short" => Primitive::Short,
            "long" => Primitive::Long,
            "bool" => Primitive::Bool,
            _ => return Err(Error::Todo),
        };

        walker.next();

        Ok(primitive)
    }
}

impl Driver for AST {
    type Parsed = ParsedAST;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let token = walker.read_until_separator()?;
        let amt_read = token.len();

        match token {
            "#pragma" => {
                todo!()
            }
            "include" => {
                todo!()
            }
            _ => {
                walker.next();

                if let Ok(_parsed) = Function::drive(walker) {
                    Ok(ParsedAST::Function)
                } else {
                    // TODO: Error
                    todo!()
                }
            }
        }
    }
}

impl Driver for Struct {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
		let prefix = walker.read_until_separator()?;

		let strct = if prefix == "struct" {
			walker.next();
			walker.ensure_separator()?;

			let name = walker.read_until_separator()?;
			if !name.is_empty() {
				Struct(name.to_string())
			} else {
				return Err(Error::Todo)
			}
		} else {
			return Err(Error::Todo)
		};

		walker.next();

		Ok(strct)
    }
}
