use std::fs;
use std::error::Error;
use std::convert::TryFrom;

use colored::*;

use crate::*;

pub struct Args {
    filename: String,
    key: Option<String>,
    first: bool,
    follow: bool,
    first_plus: bool,
    check_first_plus: bool,
    show_mappings: bool,
    parser: bool,
    protos: Option<String>,
}

pub fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        filename: args.value_from_str("--filename")?,
        key: args.opt_value_from_str("--key")?,
        first: args.contains("--first"),
        follow: args.contains("--follow"),
        first_plus: args.contains("--first_plus"),
        check_first_plus: args.contains("--check_first_plus"),
        show_mappings: args.contains("--show_mappings"),
        parser: args.contains("--parser"),
        protos: args.opt_value_from_str("--protos")?,
    };

    Ok(args)
}

pub fn handle_args(args: Args) -> Result<(), Box<dyn Error>> {
    let input_file: String = args.filename;
    let contents = fs::read_to_string(input_file).expect("Failed to find the file.");

    let lines = get_file_lines(contents);
    let joined = join_lines(&lines);
    let rules: Vec<Rule> = joined
        .iter()
        .map(|x| Rule::try_from(&x[..]).unwrap())
        .collect();

    if args.check_first_plus {
        check_first_plus(&rules);
    }

    if let Some(k) = args.key {
        let keys: Vec<&str> = if k.contains(',') {
            k.split(',').map(|x| x.trim()).collect()
        } else if k == "all" {
            rules.iter().map(|x| x.non_terminal).collect()
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

    if args.parser {
        generate_parser(&rules);
    }

    if let Some(f) = args.protos {
        generate_prototypes(f, &rules, &joined);
    }

    Ok(())
}
