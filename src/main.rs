
fn ask_prompt(stdout: &std::io::Stdout) {

}
fn get_input(stdin: &std::io::Stdin) -> Option<String> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).ok()?;
    Some(buffer)
}


fn main() {
    // "Global" variables
    let mut input = "look around".to_string();
    let stdin = std::io::stdin();

    println!("Welcome to the Rusty Gate Adventure.");
    println!("Unsurprisingly, you are faced with rusty gate.");


    // Get input

    // Parse input
    // Execute

    println!("");
    println!("Bye!");
}
