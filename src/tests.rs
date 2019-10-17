use super::*;

static TEST_GRAMMAR_PATH: &str = "test.cfg";

#[test]
fn first_set_test() {
    let contents = fs::read_to_string(TEST_GRAMMAR_PATH)
        .expect("Failed to find the test grammar file.");

    let lines = get_file_lines(contents);
    let rules: Vec<Rule> = lines.iter()
        .map(|x| line_to_rule(x))
        .collect();

    let mut expected: HashSet<&str> = HashSet::new();

    for s in &vec!["(", "num", "name"] {
        expected.insert(s);
    }

    assert_eq!(first("expr", &rules), expected);
    assert_eq!(first("term", &rules), expected);
    assert_eq!(first("factor", &rules), expected);

    expected.clear();

    for s in &vec!["epsilon", "+", "-"] {
        expected.insert(s);
    }

    assert_eq!(first("expr'", &rules), expected);

    expected.clear();

    for s in &vec!["epsilon", "*", "/"] {
        expected.insert(s);
    }

    assert_eq!(first("term'", &rules), expected);
}

#[test]
fn follow_set_test() {
    let contents = fs::read_to_string(TEST_GRAMMAR_PATH)
        .expect("Failed to find the test grammar file.");

    let lines = get_file_lines(contents);
    let rules: Vec<Rule> = lines.iter()
        .map(|x| line_to_rule(x))
        .collect();

    let mut expected: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = Vec::new();

    for s in &vec!["$", ")"] {
        expected.insert(s);
    }

    assert_eq!(follow(("expr", &mut stack), &rules), expected);
    stack.clear();
    assert_eq!(follow(("expr'", &mut stack), &rules), expected);
    stack.clear();

    expected.clear();

    for s in &vec!["$", "+", "-", ")"] {
        expected.insert(s);
    }

    assert_eq!(follow(("term", &mut stack), &rules), expected);
    stack.clear();
    assert_eq!(follow(("term'", &mut stack), &rules), expected);
    stack.clear();

    expected.clear();

    for s in &vec!["$", "+", "-", ")", "*", "/"] {
        expected.insert(s);
    }

    assert_eq!(follow(("factor", &mut stack), &rules), expected);
    stack.clear();
}
