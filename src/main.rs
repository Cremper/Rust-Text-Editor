mod editor;
mod ui;

use editor::Editor;
use ui::{UI, Input};

fn main() {
    // Initialize the text editor
    let mut editor = Editor::new();

    // Main event loop
    loop {
        // Render the UI
        UI::render(&editor);

        // Capture user input
        let input = UI::capture_input();
        if input.is_none() {
            continue;
        }

        match input.unwrap() {
            Input::Insert(text) => editor.insert_text(&text),
            Input::Delete(count) => editor.delete_text(count),
            Input::Save(filename) => {
                if let Err(e) = editor.save_file(&filename) {
                    eprintln!("Failed to save file: {}", e);
                }
            }
            Input::Exit => break,
        }
    }
}