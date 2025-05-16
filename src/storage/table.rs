use std::io::Error;

pub(crate) struct Table {
    pub name: String,
    pub records: Vec<Record>,
}

pub(crate) struct Record {
    pub id: usize,
    pub name: String,
    pub runtime_minutes: usize,
    pub vector: Vec<usize>,
}

impl Table {
    pub fn add_data(&mut self, data: Record) -> Result<usize, Error> {
        self.records.push(data);
        Ok(1)
    }

    pub fn get_data(&self, id: usize) -> Result<Record, Error> {
        panic!("not implemented")
    }

    pub fn delete_data(&mut self, id: usize) -> Result<bool, Error> {
        panic!("not implemented")
    }

    pub fn search(&self, vec: Vec<usize>, limit: usize) -> Result<Vec<Record>, Error> {
        panic!("not implemented")
    }
}
