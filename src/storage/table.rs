use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

#[derive(Default, Clone)]
pub(crate) struct Table {
    pub name: String,
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

    for _ in file.lines() {
        cnt = cnt + 1;
    }

    Ok(Table {
        name: full_filename,
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
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.name.clone())?;

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
    ) -> Result<Vec<(f64, &Record)>, Error> {
        Ok(Vec::new())
        // let mut results = Vec::new();
        // for record in self.records.iter() {
        //     if similarity == "cosine" {
        //         let s = similarity::cosine_similarity(vec.clone(), record.vector.clone());
        //         results.push((s, record));
        //     }
        // }
        //
        // results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        // results.truncate(limit);
        // Ok(results)
    }
}
