fn main() {
    if let Err(e) = grammar_checker::main() {
        eprintln!("{:?}", e);
    }
}

#[cfg(test)]
mod tests;
