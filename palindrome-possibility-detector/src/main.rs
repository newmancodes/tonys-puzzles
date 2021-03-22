mod palindrome_detector;

use std::env;
use palindrome_detector::PalindromePossibilityDetector;
use palindrome_detector::EasyPalindromePossibilityDetector;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please supply a value to test!");
    }

    let detector = EasyPalindromePossibilityDetector::new();
    let candidate = &args[1];
    let could_be_a_palindrome = detector.could_be_a_palindrome(&candidate);

    println!(
        "{} could {}be a palindrome.",
        candidate,
        match could_be_a_palindrome {
            true => "",
            false => "not "
        }
    );
}
