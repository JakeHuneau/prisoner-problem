use rand::seq::SliceRandom;
use rand::thread_rng;

use prisoner_problem::configuration::Settings;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn create_numbers(num_prisoners: usize) -> Vec<usize> {
    let mut numbers: Vec<usize> = (0..num_prisoners).collect();
    numbers.shuffle(&mut thread_rng());
    numbers
}

fn single_prisoner(
    prisoner_number: usize,
    numbers: &Vec<usize>,
    num_guesses: usize,
    initial_guess: usize,
) -> bool {
    let mut guess_index = initial_guess;
    let mut guess_result: usize;
    for _ in 0..num_guesses {
        guess_result = numbers[guess_index];
        if prisoner_number == guess_result {
            return true;
        }
        guess_index = guess_result;
    }
    false
}

fn single_run(num_prisoners: usize, num_guesses: usize, debug: bool) -> bool {
    let numbers = create_numbers(num_prisoners);

    for prisoner_number in 0..num_prisoners {
        if !single_prisoner(prisoner_number, &numbers, num_guesses, prisoner_number) {
            if debug {
                print!("{}", '\u{274C}'); // x
            }
            return false;
        }
    }
    if debug {
        print!("{}", '\u{2714}'); // check mark
    }
    true
}

fn main() {
    let settings = Settings::new().unwrap();

    let total_wins: usize = (0..settings.num_runs)
        .into_par_iter()
        .map(|_| {
            if single_run(settings.num_prisoners, settings.num_guesses, settings.debug) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("\nWin %: {}", total_wins as f64 / settings.num_runs as f64);
}
