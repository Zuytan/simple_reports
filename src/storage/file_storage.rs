use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
};

use super::Storage;

pub struct FileStorage {
    folder: String,
}

impl FileStorage {
    pub fn new(folder: &str) -> Self {
        Self {
            folder: folder.to_owned(),
        }
    }
}

impl Storage for FileStorage {
    fn init(&self) -> Result<(), String> {
        let folder = Path::new(&self.folder);
        if folder.exists() {
            Ok(())
        } else {
            Err("Folder does not exist".to_string())
        }
    }

    fn save<S: super::Savable>(&self, savable: S) -> Result<(), String> {
        let fields = savable.to_fields();
        if fields.is_empty() {
            return Err("Fields are empty".to_string());
        }
        let mut id = None;
        for (idx, field) in fields.iter().enumerate() {
            if field.0 == "id" {
                id = Some((idx, field.1.clone()));
                break;
            }
        }
        if id.is_none() {
            return Err("Cannot save because there is no \"ID\" in the Savable struct".to_string());
        }
        let id = id.unwrap();
        let file_name = S::savable_name();
        let file_path_str = format!("{}/{}.csv", self.folder, file_name);
        let path = Path::new(&file_path_str);

        // Check if file exists
        let file_exists = path.exists();
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let mut lines = BufReader::new(&file).lines();
        let header_line;
        let mut data_lines = Vec::new();

        if file_exists {
            header_line = match lines.next() {
                Some(Ok(line)) => line,
                Some(Err(e)) => return Err(e.to_string()),
                None => {
                    // File is empty, create a header
                    fields
                        .iter()
                        .map(|field| field.0.clone())
                        .collect::<Vec<String>>()
                        .join(";")
                }
            };

            let mut id_found = false;
            while let Some(Ok(line)) = lines.next() {
                let splitted = line.split(";").collect::<Vec<&str>>();
                let curr_id = match splitted.get(id.0) {
                    Some(id) => id.to_string(),
                    None => return Err("DB not well formatted".to_string()),
                };
                if curr_id == id.1 {
                    id_found = true;
                    data_lines.push(
                        fields
                            .iter()
                            .map(|field| field.1.clone())
                            .collect::<Vec<String>>()
                            .join(";"),
                    );
                } else {
                    data_lines.push(line);
                }
            }

            if !id_found {
                data_lines.push(
                    fields
                        .iter()
                        .map(|field| field.1.clone())
                        .collect::<Vec<String>>()
                        .join(";"),
                );
            }

            // Rewind and write
            file.set_len(0).map_err(|e| e.to_string())?;
            file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
            file.write_all(header_line.as_bytes())
                .map_err(|e| e.to_string())?;
            file.write_all(b"\n").map_err(|e| e.to_string())?;
            for line in data_lines {
                file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
                file.write_all(b"\n").map_err(|e| e.to_string())?;
            }
        } else {
            // File does not exist, create it with header and data
            header_line = fields
                .iter()
                .map(|field| field.0.clone())
                .collect::<Vec<String>>()
                .join(";");
            let data_line = fields
                .iter()
                .map(|field| field.1.clone())
                .collect::<Vec<String>>()
                .join(";");

            file.write_all(header_line.as_bytes())
                .map_err(|e| e.to_string())?;
            file.write_all(b"\n").map_err(|e| e.to_string())?;
            file.write_all(data_line.as_bytes())
                .map_err(|e| e.to_string())?;
            file.write_all(b"\n").map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn load<S: super::Savable>(&self) -> Result<Vec<S>, String> {
        let file_name = S::savable_name();
        let file_path_str = format!("{}/{}.csv", self.folder, file_name);
        let path = Path::new(&file_path_str);

        if !path.exists() {
            return Err("File does not exist".to_string());
        }

        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let mut reader = BufReader::new(file);
        let mut lines = reader.lines();

        let header_line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e.to_string()),
            None => return Err("File is empty".to_string()),
        };

        let headers: Vec<String> = header_line.split(";").map(|s| s.to_string()).collect();
        let mut savables = Vec::new();

        for line in lines {
            let line = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            let values: Vec<String> = line.split(";").map(|s| s.to_string()).collect();
            let mut fields = Vec::new();
            for (header, value) in headers.iter().zip(values.iter()) {
                fields.push((header.clone(), value.clone()));
            }
            match S::from_fields(fields) {
                Ok(savable) => savables.push(savable),
                Err(e) => return Err(e),
            }
        }

        Ok(savables)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Savable;

    use super::*;
    use std::fs;
    use std::path::Path;

    #[derive(Debug, PartialEq)]
    pub struct User {
        pub id: String,
        pub name: String,
    }

    impl Savable for User {
        fn to_fields(&self) -> Vec<(String, String)> {
            vec![
                ("id".to_string(), self.id.clone()),
                ("name".to_string(), self.name.clone()),
            ]
        }

        fn from_fields(fields: Vec<(String, String)>) -> Result<Self, String> {
            let mut id = None;
            let mut name = None;

            for (key, value) in fields {
                match key.as_str() {
                    "id" => id = Some(value),
                    "name" => name = Some(value),
                    _ => {}
                }
            }

            if let (Some(id), Some(name)) = (id, name) {
                Ok(User { id, name })
            } else {
                Err("Missing fields".to_string())
            }
        }

        fn savable_name() -> String {
            "user".to_owned()
        }
    }
    fn setup_test_folder(folder: &str) {
        let path = Path::new(folder);
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
        fs::create_dir(path).unwrap();
    }

    #[test]
    fn test_init_existing_folder() {
        let folder = "test_storage";
        setup_test_folder(folder);
        let storage = FileStorage::new(folder);
        assert!(storage.init().is_ok());
    }

    #[test]
    fn test_init_non_existing_folder() {
        let folder = "non_existing_folder";
        let storage = FileStorage::new(folder);
        assert!(storage.init().is_err());
    }

    #[test]
    fn test_save_new_entry() {
        let folder = "test_storage";
        setup_test_folder(folder);
        let storage = FileStorage::new(folder);
        let user = User {
            id: "1".to_string(),
            name: "Alice".to_string(),
        };

        assert!(storage.save(user).is_ok());

        let loaded_users = storage.load::<User>().unwrap();
        assert_eq!(loaded_users.len(), 1);
        assert_eq!(
            loaded_users[0],
            User {
                id: "1".to_string(),
                name: "Alice".to_string()
            }
        );
    }

    #[test]
    fn test_update_existing_entry() {
        let folder = "test_storage";
        setup_test_folder(folder);
        let storage = FileStorage::new(folder);
        let user1 = User {
            id: "1".to_string(),
            name: "Alice".to_string(),
        };
        let user2 = User {
            id: "1".to_string(),
            name: "Bob".to_string(),
        };

        assert!(storage.save(user1).is_ok());
        assert!(storage.save(user2).is_ok());

        let loaded_users = storage.load::<User>().unwrap();
        assert_eq!(loaded_users.len(), 1);
        assert_eq!(
            loaded_users[0],
            User {
                id: "1".to_string(),
                name: "Bob".to_string()
            }
        );
    }

    #[test]
    fn test_load_multiple_entries() {
        let folder = "test_storage";
        setup_test_folder(folder);
        let storage = FileStorage::new(folder);
        let user1 = User {
            id: "1".to_string(),
            name: "Alice".to_string(),
        };
        let user2 = User {
            id: "2".to_string(),
            name: "Bob".to_string(),
        };

        assert!(storage.save(user1).is_ok());
        assert!(storage.save(user2).is_ok());

        let loaded_users = storage.load::<User>().unwrap();
        assert_eq!(loaded_users.len(), 2);
        assert!(loaded_users.contains(&User {
            id: "1".to_string(),
            name: "Alice".to_string()
        }));
        assert!(loaded_users.contains(&User {
            id: "2".to_string(),
            name: "Bob".to_string()
        }));
    }
}
