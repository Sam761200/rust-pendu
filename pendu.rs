use std::io::{self, Write};

const MAX_TRIES: u8 = 6;

fn main() {
    println!("Bienvenue dans le jeu du pendu !");

    let secret_word = String::from("rust"); // Mot secret à deviner
    let mut guessed_letters = vec!['_'; secret_word.len()]; // Lettres devinées jusqu'à présent
    let mut tries = 0; // Nombre de tentatives effectuées

    loop {
        print_word(&guessed_letters);
        println!("Il vous reste {} essais.", MAX_TRIES - tries);

        let guess = read_guess();
        let guess_char = guess.chars().next().unwrap(); // Prend la première lettre devinée

        if secret_word.contains(guess_char) {
            update_guessed_letters(guess_char, &secret_word, &mut guessed_letters);
            if !guessed_letters.contains(&'_') {
                println!("Bravo, vous avez deviné le mot !");
                break;
            }
        } else {
            tries += 1;
            println!("La lettre {} n'est pas dans le mot.", guess_char);
            if tries == MAX_TRIES {
                println!("Vous avez perdu. Le mot était {}.", secret_word);
                break;
            }
        }
    }
}

fn print_word(guessed_letters: &Vec<char>) {
    for letter in guessed_letters {
        print!("{} ", letter);
    }
    println!();
}

fn read_guess() -> String {
    print!("Devinez une lettre : ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Erreur lors de la lecture de l'entrée.");
    guess.trim().to_lowercase()
}

fn update_guessed_letters(
    guess: char,
    secret_word: &String,
    guessed_letters: &mut Vec<char>,
) {
    let mut updated = false;
    for (i, letter) in secret_word.chars().enumerate() {
        if letter == guess && guessed_letters[i] == '_' {
            guessed_letters[i] = letter;
            updated = true;
        }
    }

    if updated {
        println!("Bonne devinette !");
    } else {
        println!("Cette lettre a déjà été devinée.");
    }
}
