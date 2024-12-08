use rand::Rng;
use std::{fs, io, usize};

fn get_random_word() -> String {
    let file_path: &str = "words.txt";
    let file_content = fs::read_to_string(file_path).expect("can't get the file.");
    let content_lines: Vec<&str> = file_content.lines().collect();

    let rnd_word: usize = rand::thread_rng().gen_range(0..content_lines.len());

    content_lines.get(rnd_word).unwrap().to_string()
}

fn main() {
    let word_to_guess = get_random_word();
    // let word_to_guess: &str = "sists";// word_to_guess.trim();
    let word_to_guess: &str = word_to_guess.trim();
    let mut retries: i32 = 0;
    let mut correct_guess = false;
    let mut separated_word: Vec<(usize, char, bool)> = Vec::new();

    for (i, l) in word_to_guess.char_indices() {
        separated_word.push((i, l, false));
    }

    println!("the word to guess: {word_to_guess}");
    println!("enter your word");

    'begining: while retries < 6 {
        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("error in the input.");

        let word: &str = match word.len() {
            6 => {
                let aux_word = word.trim();

                for letter in aux_word.chars() {
                    if !letter.is_alphabetic() {
                        continue 'begining;
                    }
                }

                aux_word
            }
            _ => {
                println!("the word must have 5 characters length.");
                continue;
            }
        };

        if word == word_to_guess {
            correct_guess = true;
            break 'begining;
        }

        let mut first_indx: i32 = 0;

        for letter in word.chars() {
            let mut correct_letter: bool = false;
            let mut correct_index: bool = false;

            for (i, l, _g) in &mut separated_word {
                if letter != *l {
                    continue;
                }

                correct_letter = true;

                if first_indx as usize == *i {
                    correct_index = true;
                }

            }

            if !correct_letter {
                println!("{letter} - *")
            } else if correct_letter && correct_index {
                println!("{letter} - !")
            } else {
                println!("{letter} - ?")
            }

            first_indx += 1;
        }

        retries += 1;
    }

    if correct_guess {
        println!("congratulations! You guessed the word.")
    } else {
        println!("oops. You are an idiot! :)")
    }
}
