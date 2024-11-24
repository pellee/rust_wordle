use rand::Rng;
use std::{fs, io};

fn get_random_word() -> String {
    let file_path: &str = "words.txt";
    let file_content = fs::read_to_string(file_path).expect("can't get the file.");
    let content_lines: Vec<&str> = file_content.lines().collect();

    let rnd_word: usize = rand::thread_rng().gen_range(0..content_lines.len());

    content_lines.get(rnd_word).unwrap().to_string()
}

fn main() {
    let word_to_guess = get_random_word();
    let word_to_guess: &str = word_to_guess.trim();
    let mut retries: i32 = 0;
    let mut correct_guess = false;

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
            let mut second_indx: i32 = 0;
            let mut correct_letter: bool = false;
            let mut correct_index: bool = false;

            for letter_guess in word_to_guess.chars() {
                if letter != letter_guess {
                    second_indx += 1;
                    continue;
                }

                correct_letter = true;

                if first_indx == second_indx {
                    correct_index = true;
                }

                break;
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
