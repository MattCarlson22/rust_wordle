fn main() {
    let word = String::from("crane");
    game_loop(&word);
}
fn read_a_line(str1: &mut String) -> String {
    std::io::stdin().read_line(str1).unwrap();
    let out = str1.to_string();
    return out;
}
fn compare_guess(guess: &String, word: &String) -> String {
    let mut out = String::new(); 
    for i in 0 .. word.len() {
        if guess.to_ascii_lowercase()[i..i+1] == word.to_ascii_lowercase()[i..i+1] {
            out.push_str(&guess[i..i+1]);
        } else if word.to_ascii_lowercase().contains(&guess.to_ascii_lowercase()[i..i+1]) {
            out.push_str("+");
        } else {
            out.push_str("*");
        }
    }
    return out;
}
fn game_loop(word: &String) {
    println!("What is your guess?");

    let mut line = String::new();
    let guess = read_a_line(&mut line);
    let answer = compare_guess(&guess, &word);

    println!("{}",answer);

    if !answer.eq(&guess.trim()) {
        game_loop(&word);
    }
}