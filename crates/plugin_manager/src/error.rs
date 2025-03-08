use std::borrow::Cow;

use wasmi::core::ValType;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Decompression(zip::result::ZipError),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
    UnknownFile {
        filename: String,
    },
    MissingManifest,
    MissingCode,
    InvalidName {
        name: String,
    },
    InvalidCode(wasmi::Error),
    InvalidEventHandlerName(String),
    MissingEventHandler(String),
    ExportFunctionSignature {
        function_name: String,
        expected_params: Box<[ValType]>,
        expected_return: Box<[ValType]>,
        actual_params: Box<[ValType]>,
        actual_return: Box<[ValType]>,
    },
}

fn wasmi_valtype_to_rust_type(ty: &ValType) -> &str {
    match ty {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        ValType::FuncRef => "FuncRef",
        ValType::ExternRef => "ExternRef",
    }
}

fn join_wasmi_types(types: &[ValType]) -> String {
    types
        .iter()
        .map(wasmi_valtype_to_rust_type)
        .fold(String::new(), |mut acc, ty| {
            acc.push_str(", ");
            acc.push_str(ty);
            acc
        })
}

fn wasmi_return_to_rust_return(return_type: &[ValType]) -> Cow<'_, str> {
    let Some(first) = return_type.first() else {
        return Cow::Borrowed("()");
    };
    let first = wasmi_valtype_to_rust_type(first);
    if return_type.len() == 1 {
        return Cow::Borrowed(first);
    }

    let init = format!("({first}");
    let mut result = return_type
        .iter()
        .skip(1)
        .map(wasmi_valtype_to_rust_type)
        .fold(init, |mut acc, ty| {
            acc.push_str(", ");
            acc.push_str(ty);
            acc
        });
    result.push(')');

    Cow::Owned(result)
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Decompression(error) => write!(f, "decompression error: {error}"),
            Self::Deserialize(error) => {
                write!(f, "deserialization error: {error}")
            }
            Self::Serialize(error) => {
                write!(f, "serialize error: {error}")
            }
            Self::UnknownFile { filename } => write!(f, "unknown archive file: {filename}"),
            Self::MissingManifest => write!(f, "missing manifest.toml"),
            Self::MissingCode => write!(f, "missing code.wasm"),
            Self::InvalidName { name } => write!(f, "invalid name error: {name}"),
            Self::InvalidCode(error) => write!(f, "invalid code error: {error}"),
            Self::InvalidEventHandlerName(name) => {
                write!(f, "invalid event handler name error: {name}")
            }
            Self::MissingEventHandler(name) => {
                write!(
                    f,
                    "plugin code contains event that is not in manifest: {name}"
                )
            }
            Self::ExportFunctionSignature {
                function_name,
                expected_params,
                expected_return,
                actual_params,
                actual_return,
            } => {
                let expected_params = join_wasmi_types(expected_params);
                let actual_params = join_wasmi_types(actual_params);
                let expected_return = wasmi_return_to_rust_return(expected_return);
                let actual_return = wasmi_return_to_rust_return(actual_return);

                write!(
                    f,
                    "Exported '{function_name}' function wrong signature, expected: ({expected_params}) -> {expected_return}, got ({actual_params}) -> {actual_return}"
                )
            }
        }
    }
}
impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Decompression(error)
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
