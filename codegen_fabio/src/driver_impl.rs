use crate::{
    CommentBlock, Driver, Error, Function, FunctionNameWithParams, FunctionParam, Marker, Other,
    ParsedAST, Primitive, Result, Struct, Type, Walker, AST,
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
            let mut w = Walker::from(walker.read_until(';')?);

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

            walker.next();
            // Wipe separator.
            let _ = walker.ensure_separator();
        }

        // Parse additional markers at the end of the function

        // Expect semicolon.
        walker.ensure_one_semicolon()?;

        Ok(Function {
            name,
            params,
            return_ty,
            markers,
        })
    }
}

impl Driver for Other {
    type Parsed = Other;

    fn drive<R: Read>(walker: &mut Walker<R>) -> Result<Self::Parsed> {
        // TODO: Addtional validity checks?
        let other = Other(walker.read_until_separator()?.to_string());
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
