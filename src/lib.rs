use std::fs;

pub fn read_file_by_line_to_vec(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("tf man, cant read the file")
        .lines()
        .map(String::from)
        .collect()
}
