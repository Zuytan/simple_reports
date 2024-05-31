use file_storage::FileStorage;

pub mod file_storage;

pub trait Savable {
    fn savable_name() -> String
    where
        Self: Sized;
    fn to_fields(&self) -> Vec<(String, String)>
    where
        Self: Sized;
    fn from_fields(fields: Vec<(String, String)>) -> Result<Self, String>
    where
        Self: Sized;
}

pub enum StorageType {
    FileStorage(&'static str),
}

impl StorageType {
    pub fn build(&self) -> impl Storage {
        match self {
            StorageType::FileStorage(path) => FileStorage::new(&path),
        }
    }
}
pub trait Storage {
    fn init(&self) -> Result<(), String>;
    fn save<S: Savable>(&self, savable: S) -> Result<(), String>;
    fn load<S: Savable>(&self) -> Result<Vec<S>, String>;
}
