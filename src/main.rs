use std::fs;
use std::error::Error;

use std::collections::HashSet;

struct Args {
    filename: Option<String>,
    key: Option<String>,
    first: bool,
    follow: bool,
}

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

fn follow<'a>(symbol: &'a str, rules: &Vec<Rule<'a>>) -> HashSet<&'a str> {
    // Find all places where the symbol occurs on the right
    let mut interesting: Vec<(&Production, &str)> = Vec::new();
    let mut follow_set: HashSet<&str> = HashSet::new();

    for r in rules {
        for p in &r.derivations {
            if p.output.contains(&symbol) {
                interesting.push((p, r.non_terminal));
            }
        }
    }

    for (p, t) in &interesting {
        let pos: usize = p.output
            .iter()
            .position(|x| x == &symbol)
            .unwrap();

        let len: usize = p.output.len();

        if pos + 1 == len {
            // We are at the end of the rule
            if t != &symbol {
                let f: HashSet<&str> = follow(t, rules);

                for e in f {
                    follow_set.insert(e);
                }
            }
        } else {
            // Add the first set of the next token
            let f: HashSet<&str> = first(p.output[pos + 1], rules);

            for e in f {
                if e == "epsilon" {
                    let f2: HashSet<&str> = follow(t, rules);

                    for e2 in f2 {
                        follow_set.insert(e2);
                    }
                } else {
                    follow_set.insert(e);
                }
            }
        }
    }

    if interesting.is_empty() {
        // This is the start node
        follow_set.insert("$");
    }

    follow_set
}

fn get_file_lines(contents: String) -> Vec<String> {
    contents.split("\n")
        .map(|l| l.replace("\"", "'"))
        .filter(|x| !x.is_empty())
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        filename: args.opt_value_from_str("--filename")?,
        key: args.opt_value_from_str("--key")?,
        first: args.contains("--first"),
        follow: args.contains("--follow"),
    };

    let input_file = match args.filename {
        Some(f) => f,
        None => panic!("Please enter a filename using --filename."),
    };

    let contents = fs::read_to_string(input_file)
        .expect("Failed to find the file.");

    let lines = get_file_lines(contents);
    let rules: Vec<Rule> = lines.iter()
        .map(|x| line_to_rule(x))
        .collect();

    if let Some(k) = args.key {
        if args.first {
            println!("first({}) = {:?}", k, first(&k, &rules));
        }

        if args.follow {
            println!("follow({}) = {:?}", k, follow(&k, &rules));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
