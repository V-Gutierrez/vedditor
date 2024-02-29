use std::{fs, io::Error};

use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    #[must_use]
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows: Vec<Row> = vec![];

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self { rows })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
