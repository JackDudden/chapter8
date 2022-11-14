use std::io;

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    io::stdin().read_line(&mut input).ok().expect("Error reading string");
    let input = input.to_lowercase();
    let input = input.split_whitespace();

    for word in input {
        let first = word.chars().next().unwrap();

        let temp = if ['a', 'e', 'i', 'o', 'u'].contains(&first) {
            let new_word = format!("{}-hay ", word);
            new_word
        } else {
            let mut new_word = String::new();

            for (i, c) in word.char_indices() {
                if i == 0 {
                    continue;
                } else {
                    new_word.push(c)
                }
            }

            let new_word = format!("{}-{}ay ", new_word, first);
            new_word
        };

        output.push_str(&temp);
    }
    let output = output.trim_end();
    println!("{}", output);
}