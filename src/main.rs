use std::fs;

fn main() {
    let input_file: &str = "grammar.cfg";
    let contents = fs::read_to_string(input_file)
        .expect("Failed to find the file.");

    let lines: Vec<String> = contents.split("\n")
        .map(|l| l.replace("\"", "'"))
        .collect();

    dbg!(&lines);
}
