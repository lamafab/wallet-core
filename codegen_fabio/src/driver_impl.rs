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

        dbg!(&return_ty);

        // Check for possible markers, function name and function params.
        let mut markers = vec![];
        let name;
        let params;

        // TODO: Comment on behavior
        loop {
            walker.next();

            let maybe_func_params = walker.read_until(')')?;
            let mut w = Walker::from(maybe_func_params);
            dbg!(maybe_func_params);

            if let Ok(f) = FunctionNameWithParams::drive(&mut w) {
                dbg!(&f);

                name = f.name;
                params = f.params;
                break;
            }

            let maybe_marker = walker.read_until_separator()?;
            let mut w = Walker::from(maybe_marker);
            if let Ok(marker) = Marker::drive(&mut w) {
                dbg!(&marker);
                markers.push(marker);
                continue;
            }

            return Err(Error::Todo);
        }

        // Parse additional markers at the end of the function
        loop {
            walker.next();

            dbg!(walker.read_eof()?);
            let maybe_marker = walker.read_until_separator()?;
            dbg!(&maybe_marker);

            // TODO: This should be handled by `Marker::drive`.
            // Don't parse semicolon as a custom marker.
            if maybe_marker == ";" {
                break;
            }

            let mut w = Walker::from(maybe_marker);

            if let Ok(marker) = Marker::drive(&mut w) {
                dbg!(&marker);
                markers.push(marker);
            } else {
                walker.next();
                break;
            }
        }

        dbg!(walker.read_eof()?);

        // Insist on semicolon termination
        let semi = walker.read_eof()?;
        dbg!(semi);
        if semi != ";" {
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

        dbg!(&function_name);

        if !valid_var_name(&function_name) {
            return Err(Error::Todo);
        }

        walker.next();

        // Parse parameters
        let mut params = vec![];
        let buffer = walker.read_until(')')?;
        let param_body = &buffer[..buffer.len() - 1];
        dbg!(param_body);

        // TODO: Rename
        let chunks = param_body.split(',').collect::<Vec<&str>>();
        for chunk in chunks {
            dbg!(&chunk);
            let mut chunk_walker = Walker::from(chunk.trim());

            // Parameter type
            let param_ty = {
                let mut w = Walker::from(chunk_walker.read_until_separator()?);
                Type::drive(&mut w)?
            };

            dbg!(&param_ty);

            // Parse the parameter name and possible markers.
            let mut markers = vec![];
            let mut param_name = None;
            loop {
                chunk_walker.next();

                // TODO: Comment on behavior
                let maybe_marker = chunk_walker.read_until_separator()?;
                dbg!(maybe_marker);
                if maybe_marker.is_empty() {
                    if param_name.is_none() {
                        return Err(Error::Todo);
                    }

                    markers.pop();
                    break;
                }

                let mut w = Walker::from(maybe_marker);
                if let Ok(marker) = Marker::drive(&mut w) {
                    dbg!(&marker);
                    param_name = Some(maybe_marker.to_string());
                    markers.push(marker);
                }
            }

            chunk_walker.next();

            dbg!(&param_name);
            dbg!(&markers);

            params.push(FunctionParam {
                // Panic implies bug.
                name: param_name.unwrap(),
                ty: param_ty,
                markers,
            });
        }

        walker.next();

        dbg!(walker.read_eof()?);
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

    // TODO: Experimenting around, clean this all up.
    // TODO: This should not loop, the `Engine` should be responsible for reading the full file.
    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        let mut ast = AST::new();

        loop {
            if walker.is_eof()? {
                break;
            }

            let token = walker.read_until_fn(|char| char == '\n', true)?;

            let token_len = token.len();
            dbg!(&token);

            if token.starts_with("//") {
                // TODO
            } else {
                // Assume function
                // TODO: Implement a `read_until_with` method
                let mut slice = walker
                    .read_until_fn(|char| char == ';', true)
                    .unwrap()
                    .to_string();
                slice.push(';');

                let mut w = Walker::from(slice.as_str());
                dbg!(&slice);

                if let Ok(func) = Function::drive(&mut w) {
                    ast.push(crate::AstVariants::Function(func));
                    walker.next();
                    walker.ensure_one_semicolon().unwrap();
                    walker
                        .ensure_consume_fn(|char| char == '\n', crate::EnsureVariant::AtLeast(0))
                        .unwrap();
                    continue;
                }
            }

            walker.last_read_amt = token_len;
            walker.next();
            walker
                .ensure_consume_fn(|char| char == '\n', crate::EnsureVariant::AtLeast(0))
                .unwrap();
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
