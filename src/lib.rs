// Copyright 2019 Bernardo Ferrari.
// Licensed under the MIT license <https://opensource.org/licenses/MIT>.

use std::fs::read_to_string;
use std::path::Path;

use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use distance::*;
use structopt::StructOpt;

/// Welcome to Changelog Disaster Mitigator!
/// {n}This tool helps you make sure your changelog messages won't be offensive or hilarious once Play Store modifies it.
/// {n}You might supply an input path (via --path), only an input (via --input) or nothing. If you supply nothing, it will read from the clipboard.
/// {n}You may supply the maximum size, in chars, that a screen can have. If you don't supply anything, it will check against every size between ~56 and 110, which should cover all cases.
/// {n}{n}Examples:
/// {n}cdmitigator -i "We update the Uber app as often as possible"
/// {n}cdmitigator -p changelog.txt
/// {n}cdmitigator -s96
/// {n}cdmitigator -s0
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// The path to the file that is going be read
    #[structopt(short, long, parse(from_os_str))]
    path: Option<std::path::PathBuf>,

    /// The input that is going to be read
    #[structopt(short, long)]
    input: Option<String>,

    /// The total size in chars for the output. Usually ranges from 50 to 110.
    #[structopt(short, long, default_value = "0")]
    size: usize,

    /// The levenshtein distance to catch bad words.
    #[structopt(short, long, default_value = "1")]
    distance: usize,

    #[structopt(short, long)]
    errors_only: bool,
}

fn split_string(description: &str, start_size: usize, end_size: usize) -> (&str, &str) {
    let (end_index, _) = description.char_indices().rev().nth(end_size).unwrap();
    let start = &description[..start_size];
    let end = &description[end_index..];

    (start.trim_end(), end.trim_start())
}

fn read_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().expect("Failed to open clipboard")
}

fn read_file(path: &Path) -> String {
    read_to_string(path).unwrap_or_else(|_| panic!("Failed to open {}", path.display()))
}

pub fn retrieve_input() -> (String, usize, usize, bool) {
    let opt = Cli::from_args();

    // tries to find a path input, else a standard input, else just read the clipboard.
    return (match opt.path {
        Some(path) => read_file(&path),
        None => match opt.input {
            Some(input) => input,
            None => read_clipboard()
        }
    }, opt.size, opt.distance, opt.errors_only);
}

pub enum Event {
    Nothing,
    Shorter,
    Warning,
    Error(String, usize),
}

pub fn event_to_str(e: &Event) -> ColoredString {
    match e {
        Event::Shorter => String::from("[Ok] Input is shorter than size. Stopping...").green(),
        Event::Warning => String::from("[Url detected!] at end of the string. Play Store might shrink it.").yellow(),
        Event::Error(word, dist) => format!("[Bad word detected!] \"{}\" with {} distance.", word, dist).to_string().red(),
        Event::Nothing => String::from("").green(),
    }
}

pub fn play_modifier(description: &str, size: usize, distance: usize) -> (String, Event) {
    let len = description.len();

    if len < size {
        return (description.parse().unwrap(), Event::Shorter);
    };

    let (start, end) = if description[..len / 2].contains('\n') {
        // in changelogs with newlines in the first half of the string, it was detected that
        // play store shrinks it differently. This tries to mimic it.
        let one_third_size = size / 3;
        split_string(description, one_third_size * 2, one_third_size)
    } else {
        // the standard split.
        split_string(description, ((size as f64) / 2.0).round() as usize, size / 2)
    };

    let mut max_distance: usize = usize::max_value();
    let mut max_word: &str = "";

    // inspired from https://github.com/RobertJGabriel/Google-profanity-words
    let bad_words = ["anal", "anus", "ass", "balls", "bastard", "bitch", "bloody", "boob", "butt", "clitoris", "cock", "crap", "damn", "dildo", "dyke", "fuck", "hell", "jerk", "jizz", "labia", "lmao", "lmfao", "nigger", "nigga", "omg", "penis", "piss", "poop", "pube", "pussy", "queer", "scrotum", "sex", "shit", "slut", "spunk", "tit", "tosser", "twat", "vagina", "wank", "whore", "wtf"];
    let start_end_word = [start.split_whitespace().last().unwrap(), end.split_whitespace().nth(0).unwrap()].concat();

    for word in bad_words.iter() {
        let distance = levenshtein(word, &start_end_word);
        if distance < max_distance {
            max_distance = distance;
            max_word = word;
        }
    }

    let joined_str = [start.trim_end(), "…", end.trim_start()].concat();

    if max_distance <= distance {
        (joined_str, Event::Error(max_word.to_string(), max_distance))
    } else {
        let warning = if end.contains("https://") || end.contains("http://") {
            Event::Warning
        } else {
            Event::Nothing
        };

        (joined_str, warning)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn one_result() {
//        let description = "We update the Uber app as often as possible to help make it faster and more reliable for you. This version includes several bug fixes and performance improvements.
//
//        Love the app? Rate us! Your feedback helps us to improve the Uber app.
//        Have a question? Tap Help in the Uber app or visit help.uber.com.";
//        let (value, _) = play_modifier(&description, 60);
//
//        assert_eq!(
//            value,
//            "We update the Uber app as ofte...ber app or visit help.uber.com."
//        );
//    }
//
//    #[test]
//    fn two_result() {
////        let description = "Bug fixes and improvements
////
////        We'd like to invite you to help shape the future of the app by providing valuable feedback. Come join our Android community here https://www.reddit.com/r/MediumApp/.";
////        let (value, _) = play_modifier(&description, 60);
////
////        assert_eq!(
////            value,
////            "Bug fixes and improvements\n\n        We\'d...dit.com/r/MediumApp/."
////        );
//    }
//}
