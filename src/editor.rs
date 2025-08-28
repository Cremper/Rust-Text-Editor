pub struct Editor {
    pub content: String,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            content: String::new(),
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn delete_text(&mut self, count: usize) {
        let len = self.content.chars().count();
        if count >= len {
            self.content.clear();
        } else {
            let new_len = len - count;
            let truncated: String = self.content.chars().take(new_len).collect();
            self.content = truncated;
        }
    }

    pub fn save_file(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, &self.content)
    }
}