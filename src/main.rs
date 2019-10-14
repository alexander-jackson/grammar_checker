use std::fs;

use std::collections::HashSet;

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

fn first<'a>(symbol: &'a str, rules: &Vec<Rule<'a>>) -> HashSet<&'a str> {
    let mut set: HashSet<&'a str> = HashSet::new();

    if !rules.iter().any(|x| x.non_terminal == symbol) {
        // Symbol is a terminal node
        set.insert(symbol);
    } else {
        // Symbol is a non-terminal node
        // Find its rules
        let mut symbol_rules: &Vec<Production<'a>> = &Vec::new();

        for r in rules {
            if r.non_terminal == symbol {
                symbol_rules = &r.derivations;
            }
        }

        for p in symbol_rules {
            let children: HashSet<&'a str> = first(p.output[0], rules);

            for c in children {
                set.insert(c);
            }
        }
    }

    set
}

fn main() {
    let input_file: &str = "test.cfg";
    let start: &str = "goal";
    let contents = fs::read_to_string(input_file)
        .expect("Failed to find the file.");

    let lines: Vec<String> = contents.split("\n")
        .map(|l| l.replace("\"", "'"))
        .filter(|x| !x.is_empty())
        .collect();

    let rules: Vec<Rule> = lines.iter()
        .map(|x| line_to_rule(x))
        .collect();

    for r in &rules {
        if r.non_terminal != start {
            let symbol = r.non_terminal;
            println!("first({}) = {:?}", symbol, first(symbol, &rules));
        }
    }
}
