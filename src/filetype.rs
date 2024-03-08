pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_opts
    }
}

impl From<&String> for FileType {
    fn from(file_name: &String) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                },
            };
        }

        Self::default()
    }
}

impl HighlightingOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }

    pub fn strings(self) -> bool {
        self.strings
    }

    pub fn characters(self) -> bool {
        self.characters
    }
}