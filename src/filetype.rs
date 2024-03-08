pub struct FileType {
    name: String,
    hl_opts: HighlightningOptions,
}

#[derive(Default)]
pub struct HighlightningOptions {
    pub numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightningOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl From<&String> for FileType {
    fn from(file_name: &String) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightningOptions { numbers: true },
            };
        }

        Self::default()
    }
}