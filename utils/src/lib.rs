use std::fs;

pub fn read_input(filename: &str) -> String {
    let content = fs::read_to_string(filename)
        .expect("konnte die datei nicht lesen.");

    content.trim().to_string()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_input(filename)
        .lines()
        .map(|line| line.to_string())
        .collect()
}
