pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Decompression(zip::result::ZipError),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
    UnknownFile { filename: String },
    MissingManifest,
    MissingCode,
    InvalidName { name: String },
    InvalidCode(wasmi::Error),
    InvalidEventHandlerName(String),
    MissingEventHandler(String),
    EventHandlerSignature(String),
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
            Self::EventHandlerSignature(name) => {
                write!(
                    f,
                    "Event handler has incorrect signature (should be (*const u8, usize) -> bool): {name}"
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
