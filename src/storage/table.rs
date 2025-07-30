use crate::storage::index::{Index, IndexEntry};
use crate::storage::similarity;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::Path;

#[derive(Default, Clone)]
pub(crate) struct Table {
    pub name: String,
    indexes: Vec<IndexEntry>,
    pub size: usize,
}

pub(crate) fn init_table(full_filename: String) -> Result<Table, Error> {
    let path = Path::new(full_filename.as_str());
    if !path.exists() {
        File::create(full_filename.clone())?;
        return Ok(Table {
            name: full_filename,
            ..Default::default()
        });
    }

    let file =
        BufReader::new(File::open(full_filename.clone()).expect("Unable to open table file"));
    let mut cnt = 0;

    let mut indexes = Vec::new();
    for line in file.lines() {
        let index: IndexEntry = serde_json::from_str(line?.as_str())?;
        indexes.push(index);
        cnt = cnt + 1;
    }

    Ok(Table {
        name: full_filename,
        indexes,
        size: cnt,
        ..Default::default()
    })
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Record {
    pub id: usize,
    pub name: String,
    pub runtime_minutes: usize,
    pub vector: Vec<usize>,
}

impl Table {
    pub fn add_data(&mut self, data: Record) -> Result<usize, Error> {
        let index = self.read_index_file()?;
        let filename;
        if index.len() < self.number_of_clusters() {
            let index_entry = self.add_new_center(data.vector.clone())?;
            self.indexes.push(index_entry.clone());
            filename = index_entry.filename;
        } else {
            let index_entry = self.closest_index(data.vector.clone())?;
            filename = index_entry.filename;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename.clone())?;

        let d = serde_json::to_string(&data)?;
        file.write_all(d.as_bytes())?;
        file.write_all(b"\n")?;
        self.size += 1;
        Ok(1)
    }

    pub fn get_data(&self, id: usize) -> Result<Record, Error> {
        panic!("not implemented")
    }

    pub fn delete_data(&mut self, id: usize) -> Result<bool, Error> {
        panic!("not implemented")
    }

    pub fn search(
        &self,
        vec: Vec<usize>,
        limit: usize,
        similarity: &str,
    ) -> Result<Vec<(f64, Record)>, Error> {
        let index = self.closest_index(vec.clone())?;
        let file =
            BufReader::new(File::open(index.filename.clone()).expect("Unable to open table file"));

        let mut results = Vec::new();
        for line in file.lines() {
            let record: Record = serde_json::from_str(line?.as_str())?;
            if similarity == "cosine" {
                let s = similarity::cosine_similarity(vec.clone(), record.vector.clone());
                results.push((s, record));
            }
        }

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        results.truncate(limit);
        Ok(results)
    }

    fn closest_index(&self, data: Vec<usize>) -> Result<IndexEntry, Error> {
        if self.indexes.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "no indexes configured"));
        }

        let mut closest = 0_f64;
        let mut closest_idx = Default::default();
        for index in self.indexes.clone() {
            let s = similarity::cosine_similarity(index.center.clone(), data.clone());
            if s > closest {
                closest = s;
                closest_idx = index;
            }
        }
        Ok(closest_idx)
    }
}
