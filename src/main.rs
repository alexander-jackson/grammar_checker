use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use std::collections::HashSet;

use colored::*;

struct Args {
    filename: Option<String>,
    key: Option<String>,
    first: bool,
    follow: bool,
    first_plus: bool,
    check_first_plus: bool,
    show_mappings: bool,
    protos: Option<String>,
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
        output: productions.split(' ').collect(),
    }
}

/// Creates a Rule struct from a line in the grammar file
fn line_to_rule(line: &str) -> Rule {
    let split: Vec<&str> = line.split(" ::= ").collect();

    let prods: Vec<Production> = split[1]
        .split(" | ")
        .map(|x| create_production(x))
        .collect();

    Rule {
        non_terminal: split[0],
        derivations: prods,
    }
}

/// Calculates the FIRST set of a symbol given the rules of the grammar
fn first<'a>(symbol: &'a str, rules: &[Rule<'a>]) -> HashSet<&'a str> {
    let mut first_set: HashSet<&'a str> = HashSet::new();

    if !rules.iter().any(|x| x.non_terminal == symbol) {
        // Symbol is a terminal node
        first_set.insert(symbol);
    } else {
        // Symbol is a non-terminal node
        // Find its rules
        let symbol_rules: &Vec<Production<'a>> = &rules
            .iter()
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
fn follow<'a, 'b>(
    (symbol, stack): (&'a str, &'b mut Vec<&'a str>),
    rules: &[Rule<'a>],
) -> HashSet<&'a str> {
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
        let pos: usize = p.output.iter().position(|x| x == &symbol).unwrap();

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

fn first_plus<'a>(symbol: &'a str, rules: &[Rule<'a>]) -> Vec<HashSet<&'a str>> {
    let mut first_plus_set = Vec::new();

    // Check if this is a terminal
    if !rules.iter().any(|x| x.non_terminal == symbol) {
        return first_plus_set;
    }

    // Find the rules for this non-terminal
    let pos: usize = rules.iter().position(|x| x.non_terminal == symbol).unwrap();

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
fn disjoint<'a>(sets: &'a [HashSet<&'a str>]) -> bool {
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
    !(x.is_empty() || x.starts_with("//") || x.starts_with('#') || x.starts_with(';'))
}

/// Splits the lines of a grammar file up into a Vec<String>
fn get_file_lines(contents: String) -> Vec<String> {
    contents
        .split('\n')
        .filter(|x| valid_string(x))
        .map(|l| l.trim())
        .map(|l| l.replace("\"", "'"))
        .collect()
}

fn join_lines(lines: &[String]) -> Vec<String> {
    let mut joined: Vec<String> = Vec::new();

    // Get the line numbers that start a rule definition
    let containing: Vec<usize> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| (i, l.contains("::=")))
        .filter(|(_i, l)| *l)
        .map(|(i, _l)| i)
        .collect();

    let len = containing.len();

    for c in 0..len - 1 {
        let (i, j) = (containing[c], containing[c + 1]);

        joined.push(lines[i..j].join(" "));
    }

    let last = containing[len - 1];
    joined.push(lines[last..lines.len()].join(" "));

    joined
}

fn check_first_plus<'a>(rules: &[Rule<'a>]) {
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

fn show_mappings<'a>(key: &str, rules: &[Rule<'a>]) {
    println!("{}", key.green());

    if !rules.iter().any(|x| x.non_terminal == key) {
        println!("This is a terminal node.");
    } else {
        let first_plus_set = first_plus(key, rules);
        let derivations = &rules
            .iter()
            .find(|x| x.non_terminal == key)
            .unwrap()
            .derivations;

        for (f, d) in first_plus_set.iter().zip(derivations.iter()) {
            println!("{:?} => {:?}", f, d);
        }
    }
}

fn generate_prototypes<'a>(path: String, rules: &[Rule<'a>], joined: &[String]) {
    let lines: Vec<String> = rules
        .iter()
        .zip(joined.iter())
        .map(|(r, j)| {
            format!(
                r#"// {}
void parse_{}();

"#,
                j, r.non_terminal
            )
        })
        .collect();

    let mut file = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(&path)
        .unwrap();

    for l in lines {
        write!(file, "{}", l).expect("Failed to write to the file.");
    }
}

fn generate_code<'a>(
    non_terminal: &str,
    f_plus: &[HashSet<&str>],
    derivations: &[Production<'a>],
    terminals: &[&str],
    non_terminals: &[&str],
) {
    let mut output: String = String::new();
    let fname: &str = &format!("parse_{}", non_terminal);
    output.push_str(&format!("void {}() {{\n", fname));

    output.push_str(&format!(
        "std::cout << \"Calling {}\" << std::endl;",
        fname
    ));

    let mut iters = 0;

    for (f, d) in f_plus.iter().zip(derivations.iter()) {
        // Given that we match any token in f, perform code for d
        let mut opts: Vec<&str> = Vec::new();

        for x in f.iter() {
            opts.push(x);
        }

        let options: String = opts.join(" | ");

        output.push_str(&format!(
            "\t{}if (match({})) {{\n",
            if iters > 0 { "else " } else { "" },
            &options
        ));

        let logic: String = d
            .output
            .iter()
            .map(|x| {
                if terminals.contains(x) {
                    format!("match_terminal({});", x)
                } else {
                    format!("parse_{}();\nstd::cout << \"Returned to {}\" << std::endl;", x, fname)
                }
            })
            .collect::<Vec<String>>()
            .join("\n\t\t");

        output.push_str(&logic);
        output.push_str("\n\t}\n");

        iters += 1;
    }

    output.push_str(&format!(
        "else {{ std::cout << \"Error in {}\" << std::endl;\nexit(1); }}",
        fname
    ));

    output.push_str("}\n");

    println!("{}", &output);
}

fn generate_parser<'a>(rules: &[Rule<'a>], lines: &[String]) {
    // Get all tokens in the grammar
    let mut tokens: Vec<&str> = Vec::new();

    for r in rules {
        if !tokens.contains(&r.non_terminal) {
            tokens.push(&r.non_terminal);
        }

        for d in &r.derivations {
            for t in &d.output {
                if !tokens.contains(t) {
                    tokens.push(t);
                }
            }
        }
    }

    let mut terminals: Vec<&str> = Vec::new();
    let mut non_terminals: Vec<&str> = Vec::new();

    for t in tokens {
        match rules.iter().find(|x| x.non_terminal == t) {
            Some(_) => non_terminals.push(t),
            None => terminals.push(t),
        };
    }

    for nt in &non_terminals {
        let f_plus = first_plus(&nt, &rules);
        let derivations = &rules
            .iter()
            .find(|x| &x.non_terminal == nt)
            .unwrap()
            .derivations;
        generate_code(&nt, &f_plus, derivations, &terminals, &non_terminals);
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
        show_mappings: args.contains("--show_mappings"),
        protos: args.opt_value_from_str("--protos")?,
    };

    let input_file = match args.filename {
        Some(f) => f,
        None => panic!("Please enter a filename using --filename."),
    };

    let contents = fs::read_to_string(input_file).expect("Failed to find the file.");

    let lines = get_file_lines(contents);
    let joined = join_lines(&lines);
    let rules: Vec<Rule> = joined.iter().map(|x| line_to_rule(x)).collect();

    // generate_parser(&rules, &joined);

    if args.check_first_plus {
        check_first_plus(&rules);
    }

    if let Some(k) = args.key {
        let keys: Vec<&str> = if k.contains(',') {
            k.split(',').map(|x| x.trim()).collect()
        } else if k == "all" {
            rules.iter()
                .map(|x| x.non_terminal)
                .collect()
        } else {
            vec![&k]
        };

        for key in keys {
            if args.first {
                println!("first({}) = {:?}", key, first(&key, &rules));
            }

            if args.follow {
                println!(
                    "follow({}) = {:?}",
                    key,
                    follow((&key, &mut Vec::new()), &rules)
                );
            }

            if args.first_plus {
                let sets = first_plus(&key, &rules);
                let output: String = format!("first_plus({}) = {:?}", key, sets);

                if disjoint(&sets) {
                    println!("{}", output.green());
                } else {
                    println!("{}", output.red());
                }
            }

            if args.show_mappings {
                show_mappings(&key, &rules);
            }
        }
    }

    if let Some(f) = args.protos {
        generate_prototypes(f, &rules, &joined);
    }

    Ok(())
}

#[cfg(test)]
mod tests;
