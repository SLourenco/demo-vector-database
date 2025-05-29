mod similarity;
pub mod table;

use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};

#[derive(Default)]
pub(crate) struct Storage {
    tables: BTreeMap<String, table::Table>,
}

impl Storage {
    pub fn create_table(&mut self, name: &str) -> Result<&mut table::Table, Error> {
        let table = table::Table {
            name: String::from(name),
            records: Vec::new(),
        };

        self.tables.insert(String::from(name), table);
        self.tables.get_mut(name).ok_or(Error::new(
            ErrorKind::Other,
            "could not find table inserted",
        ))
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
