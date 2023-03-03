use std::io::Write;

#[derive(Debug)]
struct Location<'a> {
    description: &'a str,
    tag: Noun,
}

#[derive(Debug)]
enum Verb {
    Quit,
    Look,
    Go,
}

#[derive(Debug, PartialEq)]
enum Noun {
    Around,
    Field,
    Cave,
    Unknown,
}

type Action = (Option<Verb>, Option<Noun>);

struct GameState {
    location_index: usize,
}

fn parse_input(input: &str) -> (Option<Verb>, Option<Noun>) {
    let mut words = input.split_whitespace();
    match words.next() {
        None => (None, None),
        Some("quit") => (Some(Verb::Quit), None),
        Some("look") => match words.next() {
            Some("around") => (Some(Verb::Look), Some(Noun::Around)),
            Some(_) | None => (Some(Verb::Look), None),
        },
        Some("go") => match words.next() {
            Some("field") => (Some(Verb::Go), Some(Noun::Field)),
            Some("cave") => (Some(Verb::Go), Some(Noun::Cave)),
            Some(_) => (Some(Verb::Go), Some(Noun::Unknown)),
            None => (Some(Verb::Go), None),
        },
        Some(&_) => (None, None),
    }
}

fn execute(game_state: &mut GameState, action: Action) -> String {
    match action {
        (None, None) => "nothing to do".to_string(),
        (None, Some(_)) => panic!("this an invalid state."),
        (Some(Verb::Quit), _) => "quit".to_string(),
        (Some(Verb::Look), Some(Noun::Around)) => format!(
            "you are in {}",
            LOCATIONS[game_state.location_index].description
        ),
        (Some(Verb::Look), _) => "what exactly would you like to see?".to_string(),
        (Some(Verb::Go), Some(noun)) => {
            if noun == LOCATIONS[game_state.location_index].tag {
                "you can't get much closer than this".to_string()
            } else {
                let mut output = "that ain't a place".to_string();
                for (index, location) in LOCATIONS.iter().enumerate() {
                    if noun == location.tag {
                        game_state.location_index = index;
                        output = "ok, got it".to_string();
                        break;
                    }
                }
                output
            }
        }
        (Some(Verb::Go), None) => "use your words, buddy, where exactly?".to_string(),
    }
}

// Game State
const LOCATIONS: [Location; 2] = [
    Location {
        description: "an open field",
        tag: Noun::Field,
    },
    Location {
        description: "a little cave",
        tag: Noun::Cave,
    },
];

fn main() {
    // "Global" variables
    let stdin = std::io::stdin();
    let mut game_state = GameState { location_index: 0 };

    println!("Welcome to the Rusty Gate Adventure.");
    println!("unsurprisingly, you are faced with a rusty gate");

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
        let response = execute(&mut game_state, action);

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
