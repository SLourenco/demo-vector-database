use crate::storage::table::Table;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};

pub(crate) trait Index {
    fn read_index_file(&self) -> Result<Vec<IndexEntry>, Error>;
    fn add_new_center(&self, data: Vec<usize>) -> Result<IndexEntry, Error>;
    fn number_of_clusters(&self) -> usize {
        2
    }
}

impl Index for Table {
    fn read_index_file(&self) -> Result<Vec<IndexEntry>, Error> {
        let file =
            BufReader::new(File::open(self.name.clone()).expect("Unable to open table index file"));

        let mut index = Vec::new();
        for line in file.lines() {
            let entry: IndexEntry = serde_json::from_str(line?.as_str())?;
            index.push(entry);
        }
        Ok(index)
    }

    fn add_new_center(&self, data: Vec<usize>) -> Result<IndexEntry, Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.name.clone())?;

        let idx = IndexEntry {
            filename: format!(
                "{}_part{}.vdbx",
                self.name,
                rand::rng().sample(rand::distr::Alphanumeric)
            )
            .to_string(),
            center: data,
        };
        let d = serde_json::to_string(&idx)?;
        file.write_all(d.as_bytes())?;
        file.write_all(b"\n")?;

        Ok(idx)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct IndexEntry {
    pub filename: String,
    pub center: Vec<usize>,
}
