# Text Editor Application

This project is a simple text editor application built in Rust. It provides basic functionalities for editing text, including inserting and deleting text, as well as saving files.

## Project Structure

```
text-editor-app
├── src
│   ├── main.rs        # Entry point of the application
│   ├── editor.rs      # Contains the Editor struct for text editing functionalities
│   ├── ui.rs          # Handles the user interface
│   └── utils.rs       # Utility functions for file operations and text manipulation
├── Cargo.toml         # Cargo configuration file
└── README.md          # Documentation for the project
```

## Source Instructions

1. Ensure you have Rust and Cargo installed on your machine.
2. Clone the repository or download the project files.
3. Navigate to the project directory:
   ```
   cd text-editor-app
   ```
4. Build the project using Cargo:
   ```
   cargo build
   ```
5. Run the application:
   ```
   cargo run
   ```

## Usage Guidelines

- Use the provided user interface to interact with the text editor.
- You can insert text, delete text, and save your work using the available options in the UI.
- Refer to the source code in the `src` directory for more details on the implementation and available methods.
