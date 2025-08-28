fn read_file(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

fn write_file(path: &str, contents: &str) -> std::io::Result<()> {
    std::fs::write(path, contents)
}

fn trim_whitespace(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn read_and_trim_file(path: &str) -> std::io::Result<String> {
    let content = read_file(path)?;
    Ok(trim_whitespace(&content))
}