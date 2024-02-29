use crate::Row;


#[derive(Default)]
pub struct Document {
  rows: Vec<Row>,
}

impl Document {
  #[must_use]
  pub fn open() -> Self {
    let mut rows: Vec<Row> = vec![];

    rows.push(Row::from("Hello, world!"));

    Self {
      rows
    }
  }

  #[must_use]
  pub fn row(&self, index:usize) -> Option<&Row> {
    self.rows.get(index)
  }
}