use std::borrow::Cow;

use wasmi::{TrapCode, ValType};
use wasmi_wasi::wasi_common;

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
    InvalidEventHandlerName(String),
    MissingMemoryExport,
    MissingEventHandler(String),
    ExportFunctionSignature {
        function_name: String,
        expected_params: Box<[ValType]>,
        expected_return: Box<[ValType]>,
        actual_params: Box<[ValType]>,
        actual_return: Box<[ValType]>,
    },
    Linker(wasmi::errors::LinkerError),
    Instantiation(wasmi::errors::InstantiationError),
    Wasmi {
        plugin_name: Box<str>,
        error: wasmi::Error,
    },
    Wasi(wasi_common::Error),
}

impl Error {
    #[must_use]
    pub fn as_trap_code(&self) -> Option<TrapCode> {
        let Error::Wasmi { error, .. } = self else {
            return None;
        };
        error.as_trap_code()
    }

    #[must_use]
    pub fn is_out_of_fuel_trap_code(&self) -> bool {
        self.as_trap_code()
            .is_some_and(|trap| matches!(trap, TrapCode::OutOfFuel))
    }

    #[must_use]
    pub fn plugin_name(&self) -> Option<&str> {
        let Error::Wasmi { plugin_name, .. } = self else {
            return None;
        };
        Some(plugin_name.as_ref())
    }
}

fn wasmi_valtype_to_rust_type(ty: &ValType) -> &str {
    match ty {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        ValType::FuncRef => "FuncRef",
        ValType::ExternRef => "ExternRef",
        ValType::V128 => "V128",
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
            Self::InvalidEventHandlerName(name) => {
                write!(f, "invalid event handler name error: {name}")
            }
            Self::MissingMemoryExport => write!(f, "missing `memory` export"),
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
            Self::Linker(error) => write!(f, "linker error: {error}"),
            Self::Instantiation(error) => write!(f, "instantiation error: {error}"),
            Self::Wasmi { plugin_name, error } => {
                write!(f, "wasmi error in {plugin_name}: {error}")
            }
            Self::Wasi(error) => write!(f, "wasi error: {error}"),
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
impl From<wasmi::errors::LinkerError> for Error {
    fn from(error: wasmi::errors::LinkerError) -> Self {
        Self::Linker(error)
    }
}
impl From<wasmi::errors::InstantiationError> for Error {
    fn from(error: wasmi::errors::InstantiationError) -> Self {
        Self::Instantiation(error)
    }
}

pub(crate) trait ToPluginError<T>: Sized {
    fn to_error(self, plugin_name: &str) -> Result<T>;
}

impl<T> ToPluginError<T> for std::result::Result<T, wasmi::Error> {
    fn to_error(self, plugin_name: &str) -> Result<T> {
        self.map_err(|error| Error::Wasmi {
            plugin_name: plugin_name.into(),
            error,
        })
    }
}
