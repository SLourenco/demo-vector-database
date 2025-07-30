mod index;
mod similarity;
pub mod table;

use crate::storage::table::Table;
use std::collections::BTreeMap;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Default)]
pub(crate) struct Storage {
    tables: BTreeMap<String, Table>,
}

pub(crate) fn init_storage() -> Result<Storage, Error> {
    fs::create_dir_all("database")?;
    let mut storage = Storage::default();
    for entry in fs::read_dir("database")? {
        let path = entry?.path();
        let Some(file) = path.file_name() else {
            return Err(Error::new(ErrorKind::InvalidData, "Path is not a file"));
        };
        let Some(filename) = file.to_str() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "File does not have a name",
            ));
        };

        if filename.ends_with(".vdbx") {
            // ignore indexed files
            continue;
        }

        let table = table::init_table(String::from(format!("database/{}", filename)))?;
        storage.tables.insert(String::from(filename), table);
    }
    Ok(storage)
}

impl Storage {
    pub fn init_table(&mut self, name: &str) -> Result<Table, Error> {
        let table = table::init_table(String::from(format!("database/{}.vdb", name)))?;
        self.tables.insert(String::from(name), table.clone());
        Ok(table)
    }

    pub fn delete_table(&mut self, name: &str) -> Result<bool, Error> {
        panic!("not implemented")
    }

    pub fn get_table(&mut self, name: &str) -> Result<&mut table::Table, Error> {
        self.tables
            .get_mut(name)
            .ok_or(Error::new(ErrorKind::NotFound, "table does not exist"))
    }
}
