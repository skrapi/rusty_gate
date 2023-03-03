#[derive(Debug)]
enum Verb {
    Quit,
    Look,
    Go,
}

#[derive(Debug)]
enum Noun {}

type Action = (Option<Verb>, Option<Noun>);

fn parse_input(input: &str) -> (Option<Verb>, Option<Noun>) {
    let mut words = input.split_whitespace();
    match words.next() {
        None => (None, None),
        Some("quit") => (Some(Verb::Quit), None),
        Some("look") => (Some(Verb::Look), None),
        Some("go") => (Some(Verb::Go), None),
        Some(&_) => (None, None),
    }
}

fn execute(action: Action) -> String {
    match action {
        (None, None) => "nothing to do".to_string(),
        (None, Some(_)) => todo!(),
        (Some(verb), _optional_noun) => match verb {
            Verb::Quit => "quit".to_string(),
            Verb::Look => "it's very dark in there.".to_string(),
            Verb::Go => "it's too dark to go anywhere.".to_string(),
        },
    }
}

use std::io::Write;

fn main() {
    // "Global" variables
    let stdin = std::io::stdin();

    println!("Welcome to the Rusty Gate Adventure.");
    println!("Unsurprisingly, you are faced with a rusty gate.");

    loop {
        // Prompt for input
        print!("--> ");
        std::io::stdout().flush().unwrap();

        // Get input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Parse input
        let action = parse_input(&input);

        // Execute
        let response = execute(action);

        // Print response
        if response.eq_ignore_ascii_case("quit") {
            break;
        } else {
            println!("{response}");
        }
    }

    println!("");
    println!("Ah! The coward's path I see.");
}
