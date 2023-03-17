use std::io::Write;

#[derive(Debug)]
struct Object<'a> {
    description: &'a str,
    tag: Noun,
    location: Option<&'a Object<'a>>,
}

#[derive(Debug)]
enum Verb {
    Quit,
    Look,
    Go,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Noun {
    Around,
    Field,
    Cave,
    Silver,
    Gold,
    Guard,
    Yourself,
    Unknown,
}

type Action = (Option<Verb>, Option<Noun>);

// struct GameState {
//     location_index: usize,
// }

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

fn execute(action: Action) -> String {
    match action {
        (None, None) => "nothing to do".to_string(),
        (None, Some(_)) => panic!("this an invalid state."),
        (Some(Verb::Quit), _) => "quit".to_string(),
        (Some(Verb::Look), Some(Noun::Around)) => {
            format!("you are in {}", unsafe { PLAYER.description })
        }
        (Some(Verb::Look), _) => "what exactly would you like to see?".to_string(),
        (Some(Verb::Go), Some(noun)) => {
            if noun == unsafe { PLAYER.location.unwrap().tag } {
                "you can't get much closer than this".to_string()
            } else {
                let mut output = "that ain't a place".to_string();
                unsafe {
                    for location in OBJECTS.iter() {
                        if noun == location.tag {
                            PLAYER.location = Some(location);
                            output = "ok, got it".to_string();
                            break;
                        }
                    }
                }
                output
            }
        }
        (Some(Verb::Go), None) => "use your words, buddy, where exactly?".to_string(),
    }
}

// Game State

fn main() {
    // "Global" variables
    let stdin = std::io::stdin();

     let mut OBJECTS: [Object; 6] = [
        Object {
            description: "an open field",
            tag: Noun::Field,
            location: None,
        },
        Object {
            description: "a little cave",
            tag: Noun::Cave,
            location: None,
        },
        Object {
            description: "a silver coin",
            tag: Noun::Silver,
            location: Some(FIELD),
        },
        Object {
            description: "a gold coin",
            tag: Noun::Gold,
            location: Some(CAVE),
        },
        Object {
            description: "a burly guard",
            tag: Noun::Guard,
            location: Some(FIELD),
        },
        Object {
            description: "yourself",
            tag: Noun::Yourself,
            location: Some(FIELD),
        },
    ];

    let FIELD: &Object = unsafe { &OBJECTS[0] };
    let CAVE: &Object = unsafe { &OBJECTS[1] };
    let SILVER: &Object = unsafe { &OBJECTS[2] };
    let GOLD: &Object = unsafe { &OBJECTS[3] };
    let GUARD: &Object = unsafe { &OBJECTS[4] };
    let mut PLAYER: &mut Object = unsafe { &mut OBJECTS[5] };
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
