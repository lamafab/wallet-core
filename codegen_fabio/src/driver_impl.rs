use crate::{
    AstVariants, CommentBlock, Driver, Error, Function, FunctionNameWithParams, FunctionParam,
    Marker, Other, Primitive, Result, Struct, Type, Walker, AST,
};
use std::io::Read;
use std::{primitive, str};

fn valid_var_name(name: &str) -> bool {
    // Name cannot be empty.
    if name.is_empty() {
        return false;
    }

    //Name cannot start with a number.
    if name.chars().next().unwrap().is_numeric() {
        return false;
    }

    // Check for valid characters.
    !name
        .chars()
        .any(|char| !char.is_ascii_alphanumeric() && (char != '_' && char != '-'))
}

impl Driver for Function {
    type Parsed = Function;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // Parse return value.
        let return_ty = {
            let mut w = Walker::from(walker.read_until_separator()?);
            Type::drive(&mut w)?
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
            let mut func_params = walker.read_until(')')?.to_string();
            // Parse with trailing closing bracket.
            func_params.push(')');

            let mut w = Walker::from(func_params.as_str());
            if let Ok(f) = FunctionNameWithParams::drive(&mut w) {
                name = f.name;
                params = f.params;
                break;
            } else if let Ok(other) = Marker::drive(walker) {
                markers.push(other);
            } else {
                return Err(Error::Todo);
            }

            walker.next();
            // Wipe separators.
            let _ = walker.ensure_separator();
        }

        walker.next();
        // Wipe trailing closing bracket.
        walker.ensure_consume_fn(|char| char == ')', crate::EnsureVariant::Exactly(1))?;
        // Wipe separators.
        let _ = walker.ensure_separator();

        // Parse additional markers at the end of the function
        loop {
            let maybe_marker =
                walker.read_until_fn(|char| char == ' ' || char == '\n' || char == ';', false)?;
            if !maybe_marker.is_empty() && valid_var_name(maybe_marker) {
                let mut w = Walker::from(maybe_marker);
                markers.push(Marker::drive(&mut w)?);
                walker.next();
            } else {
                break;
            }
        }

        //walker.next();

        // Wipe separators.
        let _ = walker.ensure_separator();
        // Expect semicolon.
        walker.ensure_one_semicolon()?;
        walker.ensure_eof()?;

        Ok(Function {
            name,
            params,
            return_ty,
            markers,
        })
    }
}

impl Driver for FunctionNameWithParams {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // Parse function name.
        let function_name = walker.read_until('(')?.trim_end().to_string();

        if !valid_var_name(&function_name) {
            return Err(Error::Todo);
        }

        // Consume reader, skip passed the opening bracket.
        walker.next();
        // Wipe optional separators
        let _ = walker.ensure_separator();
        walker.ensure_consume_fn(|char| char == '(', crate::EnsureVariant::Exactly(1))?;

        // Parse parameters
        let mut params = vec![];

        let param_body = walker.read_until(')')?;

        // TODO: Rename
        let chunks = param_body.split(',').collect::<Vec<&str>>();
        for chunk in chunks {
            let mut walker = Walker::from(chunk.trim());

            // Wipe optional separators.
            let _ = walker.ensure_separator();

            // Parameter type
            let param_ty = {
                let mut w = Walker::from(walker.read_until_separator()?.trim());
                Type::drive(&mut w)?
            };

            walker.next();
            walker.ensure_separator()?;

            // Parse the parameter name and possible markers.
            let mut markers = vec![];
            let param_name;
            loop {
                // TODO: Hacky, make this cleaner
                let to_eof = walker.read_eof()?.to_string();
                let to_sep = walker.read_until_separator()?;

                if to_eof == to_sep {
                    // Validate parameter name.
                    if !valid_var_name(to_sep) {
                        return Err(Error::Todo);
                    }

                    param_name = to_sep.to_string();
                    break;
                } else {
                    let mut w = Walker::from(walker.read_until_separator()?);
                    let marker = Marker::drive(&mut w)?;
                    markers.push(marker);
                    walker.next();
                    // Wipe possible separators.
                    let _ = walker.ensure_separator();
                }
            }

            walker.next();

            params.push(FunctionParam {
                name: param_name.to_string(),
                ty: param_ty,
                markers,
            });
        }

        walker.next();
        walker.ensure_consume_fn(|char| char == ')', crate::EnsureVariant::Exactly(1))?;
        walker.ensure_eof()?;

        Ok(FunctionNameWithParams {
            name: function_name,
            params,
        })
    }
}

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
            return Err(Error::Todo);
        };

        walker.next();
        walker.ensure_eof()?;

        Ok(ty)
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

                match Primitive::drive(walker)? {
                    Primitive::Char => Primitive::UnsignedChar,
                    Primitive::Int => Primitive::UnsignedInt,
                    Primitive::Short => Primitive::UnsignedShort,
                    Primitive::Long => Primitive::UnsignedLong,
                    Primitive::Bool => panic!(),
                    // Explicitly disallow all other.
                    _ => return Err(Error::Todo),
                }
            }
            "signed" => {
                walker.next();
                walker.ensure_separator()?;

                let primitive = Primitive::drive(walker)?;
                match primitive {
                    Primitive::Char | Primitive::Int | Primitive::Short | Primitive::Long => {
                        primitive
                    }
                    // Explicitly disallow all other.
                    _ => return Err(Error::Todo),
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
        walker.ensure_eof()?;

        Ok(primitive)
    }
}

impl Driver for Other {
    type Parsed = Other;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // TODO: Addtional validity checks?
		let other = walker.read_until_separator()?.to_string();
		if other.is_empty() {
			return Err(Error::Todo);
		}
        let other = Other(other);
        walker.next();
        walker.ensure_eof()?;
        Ok(other)
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
                return Err(Error::Todo);
            }
        } else {
            return Err(Error::Todo);
        };

        walker.next();

        Ok(strct)
    }
}

impl Driver for AST {
    type Parsed = Self;

	// TODO: Experimenting around, clean this all up.
	// TODO: This should not loop, the `Engine` should be responsible for reading the full file.
    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
		let mut ast = AST::new();
		let mut counter = 0;

		loop {
			if counter == 10 {
				//break;
			}
			counter += 1;

			let token = walker.read_until_fn(|char| char == '\n' || char == '\r', true).unwrap().to_string();
			let token_len = token.len();
			dbg!(&token);

			if token.is_empty() {
				let rest = walker.read_eof().unwrap();
				if rest.is_empty() {
					break;
				}
			}

			if token.starts_with("//") {
				// TODO
			} else {
				// Assume function
				// TODO: Implement a `read_until_with` method
				let mut slice = walker.read_until_fn(|char| char == ';', true).unwrap().to_string();
				slice.push(';');
				let mut w = Walker::from(slice.as_str());
				dbg!(&slice);
				if let Ok(func) = Function::drive(&mut w) {
					println!(">>> PASSED!");
					ast.push(crate::AstVariants::Function(func));
					walker.next();
					walker.ensure_one_semicolon().unwrap();
					walker.ensure_consume_fn(|char| char == '\n', crate::EnsureVariant::AtLeast(0)).unwrap();
					continue;
				}
			}

			walker.last_read_amt = token_len;
			walker.next();
			walker.ensure_consume_fn(|char| char == '\n', crate::EnsureVariant::AtLeast(0)).unwrap();
		}

        Ok(ast)
    }
}

// TODO: Test individually.
impl Driver for Marker {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let word = walker.read_until_separator()?;

        if word.is_empty() {
            return Err(Error::Todo);
        }

        let word_string = word.to_string();

        // TODO...
        walker.next();

        Ok(Marker::Other(Other(word_string)))
    }
}

impl Driver for CommentBlock {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // TODO: Be stricter

        let line = walker.read_eof()?.to_string();
        if line.starts_with("///") {
            walker.next();
            walker.ensure_eof()?;
            Ok(CommentBlock(line.replace("///", "").trim().to_string()))
        } else {
            Err(Error::Todo)
        }
    }
}
