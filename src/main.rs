use std::fs;

#[derive(Debug)]
struct Rule<'a> {
    non_terminal: &'a str,
    derivations: Vec<Production<'a>>,
}

#[derive(Debug)]
struct Production<'a> {
    output: Vec<&'a str>,
}

fn create_production(productions: &str) -> Production {
    Production {
        output: productions.split(" ")
            .collect()
    }
}

fn line_to_rule(line: &str) -> Rule {
    let split: Vec<&str> = line.split(" ::= ")
        .collect();

    let prods: Vec<Production> = split[1].split(" | ")
        .map(|x| create_production(x))
        .collect();

    Rule {
        non_terminal: split[0],
        derivations: prods,
    }
}

fn main() {
    let input_file: &str = "grammar.cfg";
    let contents = fs::read_to_string(input_file)
        .expect("Failed to find the file.");

    let lines: Vec<String> = contents.split("\n")
        .map(|l| l.replace("\"", "'"))
        .filter(|x| !x.is_empty())
        .collect();

    let rules: Vec<Rule> = lines.iter()
        .map(|x| line_to_rule(x))
        .collect();

    dbg!(&rules);
}
