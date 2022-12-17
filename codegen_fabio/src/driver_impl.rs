use crate::{
    Driver, Error, Function, FunctionNameWithParams, FunctionParam, Marker, Other, ParsedAST,
    Primitive, Result, Struct, Type, Walker, AST,
};
use std::io::{BufRead, BufReader, Read};
use std::str;

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
    !name.chars()
        .any(|char| !char.is_ascii_alphanumeric() && (char != '_' && char != '-'))
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

impl Driver for FunctionNameWithParams {
    type Parsed = Self;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
		println!("ABOUT TO PARSE FUNCTION");
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
		println!("ABOUT TO PARSE PARAMS");
        let mut params = vec![];
        loop {
            // Wipe optional separators.
            let _ = walker.ensure_separator();

            // Parameter type
            let param_ty = {
                let mut w = Walker::from(walker.read_until_separator()?.trim());
                Type::drive(&mut w)?
            };

            println!("PARAM_TY: {:?}", param_ty);

            walker.next();
            walker.ensure_separator()?;

            // Possible parameter markers.
            let mut markers = vec![];
            loop {
                let maybe_marker = match walker.read_until(',') {
                    Ok(m) => m,
                    Err(Error::Eof) => break,
                    err => return Err(err.unwrap_err()),
                };

                println!("MAYBE_MARKER: {maybe_marker}");

                // If this fails (and `maybe_marker` does not end with a closing
                // bracket), then this implies that there *is* a marker, meaning
                // we caught both the marker and the parameter name (or another
                // marker). E.g. "_NotNull my_var", which is not a valid
                // variable name.
                if !valid_var_name(maybe_marker) && !maybe_marker.ends_with(')') {
                    let mut walker = Walker::from(maybe_marker);
                    markers.push(Marker::drive(&mut walker)?);
                    walker.ensure_separator()?;
                } else {
                    break;
                }
            }

            // Parse param name
            let param_name = walker.read_until_fn(|char| char == ')' || char == ',', false)?.trim();

			println!("PARAM NAME: '{param_name}'");

            // Sanity check
            if !valid_var_name(param_name) {
				println!("ABOUT TO FAIL");
                return Err(Error::Todo);
            }

            params.push(FunctionParam {
                name: param_name.to_string(),
                ty: param_ty,
                markers,
            });

            walker.next();
			// Wipe optional separators.
            let _ = walker.ensure_separator();

			println!("AT THE END");

			println!("REST: {}", walker.read_eof()?);

            if walker
                .ensure_consume_fn(|char| char == ')', crate::EnsureVariant::Exactly(1))
                .is_ok()
            {
                // All parameters parsed.
				println!("BREAKING");
                break;
            } else {
                // Continue with next parameter, consume comma.
                walker.ensure_consume_fn(|char| char == ',', crate::EnsureVariant::Exactly(1))?;
            }

			println!("CONTINUING");
        }

		println!(">> TO EOF");
		walker.ensure_eof()?;
		println!(">> AFTER EOF");

        Ok(FunctionNameWithParams {
            name: function_name,
            params,
        })
    }
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
			let x = walker.read_until(';')?;
			println!("TO BE PARSED: {x}");
			let mut w = Walker::from(x);

            if let Ok(f) = FunctionNameWithParams::drive(&mut w) {
                name = f.name;
                params = f.params;
				walker.next();
                break;
            } else if let Ok(other) = Marker::drive(walker) {
                markers.push(other);
            } else {
                panic!()
            }

			// TODO:
            //walker.next();
			// Wipe separator.
            let _ = walker.ensure_separator();
        }

        // Check for possible marker
		/*
        if let Ok(marker) = Marker::drive(walker) {
            markers.push(marker);
            walker.next();
        }
		*/

		let rest = walker.read_eof()?;
		println!("REST: {rest}");

        // Expect semicolon.
		println!(">> ENSURE ONE SEMI");
        walker.ensure_one_semicolon()?;
		println!(">> AFTER ONE SEMI");

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
        let keyword = walker.read_until_separator()?;
        let other = Other(keyword.to_string());

        walker.next();
        walker.ensure_eof()?;

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

                let primitive = Primitive::drive(walker)?;
                match primitive {
                    Primitive::Char => Primitive::UnsignedChar,
                    Primitive::Int => Primitive::UnsignedInt,
                    Primitive::Short => Primitive::UnsignedShort,
                    Primitive::Long => Primitive::UnsignedLong,
                    Primitive::Bool => panic!(),
                    // Explicitly disallow all other.
                    _ => todo!(),
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
        walker.ensure_eof()?;

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
                return Err(Error::Todo);
            }
        } else {
            return Err(Error::Todo);
        };

        walker.next();

        Ok(strct)
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

        // TODO...

        Ok(Marker::Other(Other(word.to_string())))
    }
}
