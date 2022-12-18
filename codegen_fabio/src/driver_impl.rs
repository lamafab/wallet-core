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
            let buffer = walker.read_until_separator()?;
            let mut w = Walker::from(&buffer[..buffer.len()]);
            Type::drive(&mut w)?
        };

        // Check for possible markers, function name and function params.
        let mut markers = vec![];
        let name;
        let params;

        // TODO: Comment on behavior
        loop {
            walker.next();

            let maybe_func_params = walker.read_until(')')?;
            let mut w = Walker::from(maybe_func_params);

            if let Ok(f) = FunctionNameWithParams::drive(&mut w) {
                name = f.name;
                params = f.params;
                break;
            }

            let maybe_marker = walker.read_until_separator()?;
            let mut w = Walker::from(maybe_marker);
            if let Ok(marker) = Marker::drive(&mut w) {
                markers.push(marker);
                continue;
            }

            return Err(Error::Todo);
        }

        walker.next();

        // Parse additional markers at the end of the function
        let mut semicolon_terminated = false;
        loop {
            let maybe_marker = walker.read_until_separator()?;

            // TODO: This should be handled by `Marker::drive`.
            // Don't parse semicolon as a custom marker.
            if maybe_marker == ";" {
                semicolon_terminated = true;
                break;
            } else if maybe_marker.is_empty() {
                // EOF
                break;
            }

            // If the marker is immediately followed by a semicolon, make sure to detect that.
            let mut w = if let Some(stripped) = maybe_marker.strip_suffix(';') {
                semicolon_terminated = true;
                Walker::from(stripped)
            } else {
                semicolon_terminated = false;
                Walker::from(maybe_marker)
            };

            if let Ok(marker) = Marker::drive(&mut w) {
                markers.push(marker);
            } else {
                break;
            }

            walker.next();
        }

        walker.next();

        // Insist on semicolon termination
        if !semicolon_terminated {
            return Err(Error::Todo);
        }

        walker.next();

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
        let buffer = walker.read_until('(')?;
        let function_name = buffer[..buffer.len() - 1].trim().to_string();

        if !valid_var_name(&function_name) {
            return Err(Error::Todo);
        }

        walker.next();

        // Parse parameters
        let mut params = vec![];
        let buffer = walker.read_until(')')?;
        let param_body = &buffer[..buffer.len() - 1];

        // TODO: Rename
        let chunks = param_body.split(',').collect::<Vec<&str>>();
        for chunk in chunks {
            let mut chunk_walker = Walker::from(chunk.trim());

            // Parameter type
            let param_ty = {
                let mut w = Walker::from(chunk_walker.read_until_separator()?);
                Type::drive(&mut w)?
            };

            // Parse the parameter name and possible markers.
            let mut markers = vec![];
            let mut param_name = None;
            loop {
                chunk_walker.next();

                // TODO: Comment on behavior
                let maybe_marker = chunk_walker.read_until_separator()?;
                if maybe_marker.is_empty() {
                    if param_name.is_none() {
                        return Err(Error::Todo);
                    }

                    markers.pop();
                    break;
                }

                let mut w = Walker::from(maybe_marker);
                if let Ok(marker) = Marker::drive(&mut w) {
                    param_name = Some(maybe_marker.to_string());
                    markers.push(marker);
                }
            }

            chunk_walker.next();

            params.push(FunctionParam {
                // Panic implies bug.
                name: param_name.unwrap(),
                ty: param_ty,
                markers,
            });
        }

        walker.next();
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

    // TODO: This should just parse a single component, the `Engine` should be
    // responsible for feeding it the entire file(s).
    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let mut ast = AST::new();

        loop {
            walker.next();

            let line = match walker.read_until('\n') {
                Ok(line) => line.to_string(),
                Err(Error::Eof) => break,
                Err(err) => return Err(err),
            };

            let origin_amt = walker.last_read_amt;

            // Some components can be identified upfront.
            if line.starts_with("//") {
                // TODO
            } else if line.starts_with("#define") {
                // TODO
            } else if line.starts_with("#include") {
                // TODO
            }
            // Handle components with no clear indicator
            else {
                // We assume its a function and try to parse it.
                let maybe_function = match walker.read_until(';') {
                    Ok(line) => line,
                    Err(Error::Eof) => {
                        continue;
                    },
                    Err(err) => return Err(err),
                };

                let mut w = Walker::from(maybe_function);
                if let Ok(function) = Function::drive(&mut w) {
                    ast.push(AstVariants::Function(function));
                    continue;
                }

                // TODO: handle other components...

                // Fallback
                let mut w = Walker::from(walker.read_until_separator()?);
                ast.push(AstVariants::Other(Other::drive(&mut w)?));
            }

            // Consume line, continue with next component.
            // TODO: Find a cleaner way to do this.
            walker.last_read_amt = origin_amt;
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
