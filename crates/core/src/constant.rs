#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum StorageErrorCode {
    NotFound = -1,
    MissingMemoryExport = -2,
    InvalidKey = -3,
    InvalidValue = -5,
    OutOfStorage = -6,
    Permission = -7,
}

impl From<i32> for StorageErrorCode {
    fn from(value: i32) -> Self {
        match value {
            -1 => Self::NotFound,
            -2 => Self::MissingMemoryExport,
            -3 => Self::InvalidKey,
            -5 => Self::InvalidValue,
            -6 => Self::OutOfStorage,
            -7 => Self::Permission,
            _ => panic!("{value} not a valid StorageErrorCode"),
        }
    }
}
