pub struct UI;

pub enum Input {
    Insert(String),
    Delete(usize),
    Save(String),
    Exit,
}

impl UI {
    pub fn render(editor: &crate::editor::Editor) {
        // Example: print the content
        println!("--- Text Editor ---");
        println!("{}", editor.content);
        println!("-------------------");
        println!("Type ':q' to exit, ':w filename' to save, ':d n' to delete n chars, or just type to insert.");
    }

    pub fn capture_input() -> Option<Input> {
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush().ok()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let input = input.trim();

        if input == ":q" {
            Some(Input::Exit)
        } else if input.starts_with(":w ") {
            let filename = input[3..].trim().to_string();
            Some(Input::Save(filename))
        } else if input.starts_with(":d ") {
            if let Ok(n) = input[3..].trim().parse() {
                Some(Input::Delete(n))
            } else {
                None
            }
        } else {
            Some(Input::Insert(input.to_string()))
        }
    }
}