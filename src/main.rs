use std::fs;
use std::error::Error;

use std::collections::HashSet;

use colored::*;

struct Args {
    filename: Option<String>,
    key: Option<String>,
    first: bool,
    follow: bool,
    first_plus: bool,
    check_first_plus: bool,
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

/// Creates a Production struct from the derivations of a rule
fn create_production(productions: &str) -> Production {
    Production {
        output: productions.split(" ")
            .collect()
    }
}

/// Creates a Rule struct from a line in the grammar file
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

/// Calculates the FIRST set of a symbol given the rules of the grammar
fn first<'a>(symbol: &'a str, rules: &Vec<Rule<'a>>) -> HashSet<&'a str> {
    let mut first_set: HashSet<&'a str> = HashSet::new();

    if !rules.iter().any(|x| x.non_terminal == symbol) {
        // Symbol is a terminal node
        first_set.insert(symbol);
    } else {
        // Symbol is a non-terminal node
        // Find its rules
        let symbol_rules: &Vec<Production<'a>> = &rules.iter()
            .find(|r| r.non_terminal == symbol)
            .unwrap()
            .derivations;

        for p in symbol_rules {
            let children: HashSet<&'a str> = first(p.output[0], rules);

            for c in children {
                first_set.insert(c);
            }
        }
    }

    first_set
}

/// Calculates the FOLLOW set of a symbol given the rules of the grammar
fn follow<'a, 'b>((symbol, stack): (&'a str, &'b mut Vec<&'a str>), rules: &Vec<Rule<'a>>) -> HashSet<&'a str> {
    let mut follow_set: HashSet<&str> = HashSet::new();

    if stack.contains(&symbol) {
        return follow_set;
    }

    // Find all places where the symbol occurs on the right
    let mut interesting: Vec<(&str, &Production)> = Vec::new();

    for r in rules {
        for p in &r.derivations {
            if p.output.contains(&symbol) {
                interesting.push((r.non_terminal, p));
            }
        }
    }

    for (t, p) in &interesting {
        let pos: usize = p.output
            .iter()
            .position(|x| x == &symbol)
            .unwrap();

        let len: usize = p.output.len();

        if pos + 1 == len {
            // We are at the end of the rule
            if t != &symbol {
                stack.push(symbol);
                let f: HashSet<&str> = follow((t, stack), rules);

                for e in f {
                    follow_set.insert(e);
                }
            }
        } else {
            // Add the first set of the next token
            let f: HashSet<&str> = first(p.output[pos + 1], rules);

            for e in f {
                if e == "epsilon" {
                    stack.push(symbol);
                    let f2: HashSet<&str> = follow((t, stack), rules);

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

fn first_plus<'a>(symbol: &'a str, rules: &Vec<Rule<'a>>) -> Vec<HashSet<&'a str>> {
    let mut first_plus_set = Vec::new();

    // Check if this is a terminal
    if !rules.iter().any(|x| x.non_terminal == symbol) {
        return first_plus_set;
    }

    // Find the rules for this non-terminal
    let pos: usize = rules.iter()
        .position(|x| x.non_terminal == symbol)
        .unwrap();

    let derivations = &rules[pos].derivations;

    for (i, d) in derivations.iter().enumerate() {
        first_plus_set.push(HashSet::new());

        let first_set = first(d.output[0], rules);

        for f in &first_set {
            first_plus_set[i].insert(f);
        }

        if first_set.contains("epsilon") {
            let follow_set = follow((symbol, &mut Vec::new()), rules);

            for f in &follow_set {
                first_plus_set[i].insert(f);
            }
        }
    }

    first_plus_set
}

/// Checks whether all the HashSets are disjoint from each other
fn disjoint<'a>(sets: &'a Vec<HashSet<&'a str>>) -> bool {
    let mut values: HashSet<&'a str> = HashSet::new();

    // Iterate the sets
    for set in sets {
        for value in set {
            if !values.insert(value) {
                return false;
            }
        }
    }

    true
}

/// Checks whether the input string `x` is a valid string for the file
fn valid_string(x: &str) -> bool {
    !(x.is_empty()
      || x.starts_with("//")
      || x.starts_with("#")
      || x.starts_with(";"))
}

/// Splits the lines of a grammar file up into a Vec<String>
fn get_file_lines(contents: String) -> Vec<String> {
    contents.split("\n")
        .map(|l| l.replace("\"", "'"))
        .filter(|x| valid_string(x))
        .collect()
}

fn check_first_plus<'a>(rules: &Vec<Rule<'a>>) {
    // Iterate all the non-terminals
    for r in rules {
        let sets = first_plus(&r.non_terminal, &rules);
        let output: String = format!("first_plus({}) = {:?}", r.non_terminal, sets);

        if disjoint(&sets) {
            println!("{}", output.green());
        } else {
            println!("{}", output.red());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        filename: args.opt_value_from_str("--filename")?,
        key: args.opt_value_from_str("--key")?,
        first: args.contains("--first"),
        follow: args.contains("--follow"),
        first_plus: args.contains("--first_plus"),
        check_first_plus: args.contains("--check_first_plus"),
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

    if args.check_first_plus {
        check_first_plus(&rules);
    }

    if let Some(k) = args.key {
        if args.first {
            println!("first({}) = {:?}", k, first(&k, &rules));
        }

        if args.follow {
            println!("follow({}) = {:?}", k, follow((&k, &mut Vec::new()), &rules));
        }

        if args.first_plus {
            let sets = first_plus(&k, &rules);
            let output: String = format!("first_plus({}) = {:?}", k, sets);

            if disjoint(&sets) {
                println!("{}", output.green());
            } else {
                println!("{}", output.red());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
